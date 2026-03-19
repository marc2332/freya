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
use torin::prelude::Area;
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
    hovered_node: Arc<Mutex<Option<NodeId>>>,
}

impl DevtoolsPlugin {
    pub fn draw_wireframe(
        canvas: &freya_engine::prelude::Canvas,
        area: &Area,
        outer_color: freya_core::prelude::Color,
        inner_color: freya_core::prelude::Color,
    ) {
        let mut paint_outer = Paint::default();
        paint_outer.set_anti_alias(true);
        paint_outer.set_style(PaintStyle::Fill);
        paint_outer.set_color(outer_color);

        let mut paint_inner = Paint::default();
        paint_inner.set_anti_alias(true);
        paint_inner.set_style(PaintStyle::Fill);
        paint_inner.set_color(inner_color);

        let x = area.min_x();
        let y = area.min_y();
        let x2 = x + area.width();
        let y2 = if area.height() < 0.0 {
            y
        } else {
            y + area.height()
        };

        canvas.draw_line((x, y), (x2, y), &paint_outer);
        canvas.draw_line((x, y + 1.0), (x2, y + 1.0), &paint_inner);
        canvas.draw_line((x2, y), (x2, y2), &paint_outer);
        canvas.draw_line((x2 - 1.0, y), (x2 - 1.0, y2), &paint_inner);
        canvas.draw_line((x2, y2), (x, y2), &paint_outer);
        canvas.draw_line((x2, y2 - 1.0), (x, y2 - 1.0), &paint_inner);
        canvas.draw_line((x, y2), (x, y), &paint_outer);
        canvas.draw_line((x + 1.0, y2), (x + 1.0, y), &paint_inner);
    }

    /// Serializes and broadcasts a message to all connected devtools clients.
    fn broadcast(&self, message: OutgoingMessage) {
        let Ok(serialized) = serde_json::to_string(&message) else {
            return;
        };
        let outgoing_message = Message::Text(serialized.into());
        let websockets = self.websockets.clone();
        smol::spawn(async move {
            for websocket in websockets.lock().await.values_mut() {
                websocket.send(outgoing_message.clone()).await.ok();
            }
        })
        .detach();
    }

    pub fn init(
        &mut self,
        window_id: WindowId,
        animation_clock: &AnimationClock,
        plugin_handle: PluginHandle,
    ) {
        let start_server = {
            let mut windows = self.windows.lock().unwrap();
            let start_server = windows.is_empty();
            windows.insert(
                window_id.into(),
                WindowState {
                    nodes: vec![],
                    animation_clock: animation_clock.clone(),
                },
            );
            start_server
        };

        if start_server {
            let nodes = self.windows.clone();
            let websockets = self.websockets.clone();
            let highlighted_node = self.highlighted_node.clone();
            let hovered_node = self.hovered_node.clone();
            smol::spawn(async move {
                if let Err(err) = run_server(
                    nodes,
                    websockets,
                    highlighted_node,
                    hovered_node,
                    plugin_handle,
                )
                .await
                {
                    eprintln!("Devtools server error: {err:?}");
                }
            })
            .detach();
        }
    }
    pub fn sync(&mut self, window_id: WindowId, scale_factor: f32, tree: &Tree) {
        let window_id: u64 = window_id.into();
        let mut new_nodes = Vec::new();

        tree.traverse_depth(|node_id| {
            let height = tree.heights.get(&node_id).cloned().unwrap();
            let layout_node = tree.layout.get(&node_id).cloned().unwrap();
            let text_style_state = tree.text_style_state.get(&node_id).cloned().unwrap();
            let element = tree.elements.get(&node_id).unwrap();
            let parent_id = tree.parents.get(&node_id).cloned();
            let layer = tree.layer_state.get(&node_id).map(|s| s.layer).unwrap_or(0);
            let children_len = tree
                .children
                .get(&node_id)
                .map(|c| c.len())
                .unwrap_or_default();

            new_nodes.push(NodeInfo {
                window_id,
                is_window: height == 1,
                node_id,
                parent_id,
                children_len,
                height,
                layer,
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

        self.broadcast(OutgoingMessage {
            action: OutgoingMessageAction::Update {
                window_id,
                nodes: new_nodes.clone(),
            },
        });

        if let Some(window_state) = self.windows.lock().unwrap().get_mut(&window_id) {
            window_state.nodes = new_nodes;
        }
    }
}

impl FreyaPlugin for DevtoolsPlugin {
    fn on_event(&mut self, event: &mut PluginEvent, plugin_handle: PluginHandle) {
        match event {
            PluginEvent::WindowClosed { window, .. } => {
                let window_id: u64 = window.id().into();
                self.windows.lock().unwrap().remove(&window_id);
                self.broadcast(OutgoingMessage {
                    action: OutgoingMessageAction::Update {
                        window_id,
                        nodes: vec![],
                    },
                });
            }
            PluginEvent::AfterRender {
                tree,
                window,
                canvas,
                ..
            } => {
                let highlighted_node = *self.highlighted_node.lock().unwrap();
                let hovered_node = *self.hovered_node.lock().unwrap();

                if let Some(layout_node) = highlighted_node.and_then(|n| tree.layout.get(&n)) {
                    let area = layout_node.visible_area();
                    Self::draw_wireframe(
                        canvas,
                        &area,
                        Color::from_rgb(255, 192, 203),
                        Color::GREEN,
                    );
                }

                if let Some(layout_node) = hovered_node.and_then(|n| tree.layout.get(&n)) {
                    let area = layout_node.visible_area();
                    Self::draw_wireframe(canvas, &area, Color::RED, Color::BLUE);
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
