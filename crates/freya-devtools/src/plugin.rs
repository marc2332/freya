use std::{
    collections::HashMap,
    ops::Div,
    sync::{
        Arc,
        Mutex,
    },
};

use async_tungstenite::WebSocketSender;
use freya_core::{
    integration::{
        NodeId,
        Tree,
    },
    prelude::{
        AnimationClock,
        Color,
    },
};
use freya_engine::prelude::{
    Paint,
    PaintStyle,
};
use freya_winit::{
    plugins::{
        FreyaPlugin,
        PluginEvent,
        PluginHandle,
    },
    reexports::winit::window::WindowId,
};
use smol::net::TcpStream;
use tungstenite::Message;

use crate::{
    NodeState,
    OutgoingMessage,
    OutgoingMessageAction,
    node_info::NodeInfo,
    server::run_server,
};

pub(crate) type Websockets = HashMap<u32, WebSocketSender<TcpStream>>;
pub(crate) type SharedWebsockets = Arc<async_lock::Mutex<Websockets>>;

#[derive(Clone)]
pub struct WindowState {
    pub animation_clock: AnimationClock,
    pub nodes: Vec<NodeInfo>,
}

#[derive(Default)]
pub struct DevtoolsPlugin {
    windows: Arc<Mutex<HashMap<u64, WindowState>>>,
    websockets: SharedWebsockets,
    highlighted_node: Arc<Mutex<Option<NodeId>>>,
}

impl DevtoolsPlugin {
    pub fn init(
        &mut self,
        window_id: WindowId,
        animation_clock: &AnimationClock,
        plugin_handle: PluginHandle,
    ) {
        let start_server = self.windows.lock().unwrap().is_empty();

        self.windows.lock().unwrap().insert(
            window_id.into(),
            WindowState {
                nodes: vec![],
                animation_clock: animation_clock.clone(),
            },
        );

        if start_server {
            let nodes = self.windows.clone();
            let websockets = self.websockets.clone();
            let highlighted_node = self.highlighted_node.clone();
            let plugin_handle = plugin_handle.clone();
            smol::spawn(async move {
                run_server(nodes, websockets, highlighted_node, plugin_handle)
                    .await
                    .unwrap();
            })
            .detach();
        }
    }
    pub fn sync(&mut self, window_id: WindowId, scale_factor: f32, tree: &Tree) {
        let window_id: u64 = window_id.into();
        let mut new_nodes = Vec::new();

        tree.traverse_depth(|node_id| {
            // Ignore root element
            let height = tree.heights.get(&node_id).cloned().unwrap();
            let parent_id = tree.parents.get(&node_id).cloned();
            let layout_node = tree.layout.get(&node_id).cloned().unwrap();
            let text_style_state = tree.text_style_state.get(&node_id).cloned().unwrap();
            let children_len = tree
                .children
                .get(&node_id)
                .map(|c| c.len())
                .unwrap_or_default();
            let element = tree.elements.get(&node_id).unwrap();
            new_nodes.push(NodeInfo {
                window_id,
                is_window: height == 1, // We make the NativeContainer's element appear as the Window
                node_id,
                parent_id,
                children_len,
                height,
                state: NodeState {
                    style: element.style().into_owned(),
                    layout: element.layout().into_owned().layout,
                    text_style: text_style_state,
                    accessibility: element.accessibility().into_owned(),
                },
                area: layout_node.area.div(scale_factor),
                inner_area: layout_node.inner_area.div(scale_factor),
            });
        });

        // Update nodes snapshot
        self.windows
            .lock()
            .unwrap()
            .get_mut(&window_id)
            .unwrap()
            .nodes = new_nodes;

        // Notify the existing subscribers of this change
        let outgoing_message = Message::Text(
            serde_json::to_string(&OutgoingMessage {
                action: OutgoingMessageAction::Update {
                    window_id,
                    nodes: self
                        .windows
                        .lock()
                        .unwrap()
                        .get(&window_id)
                        .cloned()
                        .unwrap()
                        .nodes,
                },
            })
            .unwrap()
            .into(),
        );
        let websockets = self.websockets.clone();
        smol::spawn(async move {
            for websocket in websockets.lock().await.values_mut() {
                websocket.send(outgoing_message.clone()).await.unwrap();
            }
        })
        .detach();
    }
}

impl FreyaPlugin for DevtoolsPlugin {
    fn on_event(&mut self, event: &mut PluginEvent, plugin_handle: PluginHandle) {
        match event {
            PluginEvent::WindowClosed { window, .. } => {
                let window_id: u64 = window.id().into();

                // Update nodes snapshot
                self.windows.lock().unwrap().remove(&window_id);

                // Notify the existing subscribers of this change
                let outgoing_message = Message::Text(
                    serde_json::to_string(&OutgoingMessage {
                        action: OutgoingMessageAction::Update {
                            window_id,
                            nodes: vec![],
                        },
                    })
                    .unwrap()
                    .into(),
                );
                let websockets = self.websockets.clone();
                smol::spawn(async move {
                    for websocket in websockets.lock().await.values_mut() {
                        websocket.send(outgoing_message.clone()).await.unwrap();
                    }
                })
                .detach();
            }
            PluginEvent::AfterRender {
                tree,
                window,
                canvas,
                ..
            } => {
                let highlighted_node = *self.highlighted_node.lock().unwrap();
                if let Some(highlighted_node) = highlighted_node {
                    let layout_node = tree.layout.get(&highlighted_node);
                    if let Some(layout_node) = layout_node {
                        let area = layout_node.visible_area();
                        let mut paint = Paint::default();

                        paint.set_anti_alias(true);
                        paint.set_style(PaintStyle::Fill);
                        paint.set_color(Color::MAGENTA);

                        let x = area.min_x();
                        let y = area.min_y();
                        let x2 = x + area.width();
                        let y2 = if area.height() < 0.0 {
                            y
                        } else {
                            y + area.height()
                        };

                        canvas.draw_line((x, y), (x2, y), &paint);
                        canvas.draw_line((x2, y), (x2, y2), &paint);
                        canvas.draw_line((x2, y2), (x, y2), &paint);
                        canvas.draw_line((x, y2), (x, y), &paint);
                    }
                }

                self.sync(window.id(), window.scale_factor() as f32, tree);
            }
            PluginEvent::WindowCreated {
                window,
                animation_clock,
                ..
            } => {
                self.init(window.id(), animation_clock, plugin_handle);
            }
            _ => {}
        }
    }
}
