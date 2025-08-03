use std::{
    collections::HashMap,
    sync::{
        Arc,
        Mutex,
    },
};

use freya_core::{
    dom::DioxusDOM,
    node_state_snapshot::NodeStateSnapshot,
    plugins::{
        FreyaPlugin,
        PluginEvent,
        PluginHandle,
    },
    values::Color,
};
use freya_engine::prelude::{
    Paint,
    PaintStyle,
};
use freya_native_core::{
    NodeId,
    prelude::NodeImmutable,
};
use futures::{
    SinkExt,
    stream::SplitSink,
};
use hyper::upgrade::Upgraded;
use hyper_util::rt::TokioIo;
use tokio_tungstenite::WebSocketStream;
use torin::torin::Torin;
use tungstenite::Message;
use winit::window::WindowId;

use crate::{
    OutgoingMessage,
    OutgoingMessageAction,
    node_info::NodeInfo,
    server::run_server,
};

pub(crate) type Websockets = HashMap<u32, SplitSink<WebSocketStream<TokioIo<Upgraded>>, Message>>;
pub(crate) type SharedWebsockets = Arc<tokio::sync::Mutex<Websockets>>;

#[derive(Default)]
pub struct DevtoolsPlugin {
    nodes: Arc<Mutex<HashMap<u64, Vec<NodeInfo>>>>,
    websockets: SharedWebsockets,
    init: Option<()>,
    highlighted_node: Arc<Mutex<Option<NodeId>>>,
}

impl DevtoolsPlugin {
    pub fn sync(&mut self, window_id: WindowId, rdom: &DioxusDOM, layout: &Torin<NodeId>) {
        let window_id: u64 = window_id.into();
        let mut new_nodes = Vec::new();

        rdom.traverse_depth_first(|node| {
            // Ignore root elemen
            if node.height() >= 1 {
                let layout_node = layout.get(node.id()).cloned();
                if let Some(layout_node) = layout_node {
                    let node_type = node.node_type();
                    new_nodes.push(NodeInfo {
                        window_id,
                        is_window: node.height() == 1, // We make the NativeContainer's element appear as the Window
                        node_id: node.id(),
                        parent_id: node.parent_id(),
                        children_len: node
                            .children()
                            .iter()
                            .filter(|node| layout.get(node.id()).is_some())
                            .count(),
                        tag: *node_type.tag().unwrap(),
                        height: node.height(),
                        state: node.state_snapshot(),
                        layout_node,
                    });
                }
            }
        });

        // Update nodes snapshot
        self.nodes.lock().unwrap().insert(window_id, new_nodes);

        // Notify the existing subscribers of this change
        let outgoing_message = Message::Text(
            serde_json::to_string(&OutgoingMessage {
                action: OutgoingMessageAction::Update {
                    window_id,
                    nodes: self.nodes.lock().unwrap().get(&window_id).cloned().unwrap(),
                },
            })
            .unwrap()
            .into(),
        );
        let websockets = self.websockets.clone();
        tokio::spawn(async move {
            for websocket in websockets.lock().await.values_mut() {
                websocket.send(outgoing_message.clone()).await.unwrap();
            }
        });
    }
}

impl FreyaPlugin for DevtoolsPlugin {
    fn on_event(&mut self, event: &PluginEvent, plugin_handle: PluginHandle) {
        match event {
            PluginEvent::WindowClosed { window, .. } => {
                let window_id: u64 = window.id().into();

                // Update nodes snapshot
                self.nodes.lock().unwrap().remove(&window_id);

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
                tokio::spawn(async move {
                    for websocket in websockets.lock().await.values_mut() {
                        websocket.send(outgoing_message.clone()).await.unwrap();
                    }
                });
            }
            PluginEvent::AfterRender {
                fdom,
                window,
                canvas,
                ..
            } => {
                let rdom = fdom.rdom();
                let layout = fdom.layout();

                let highlighted_node = self.highlighted_node.lock().unwrap().clone();
                if let Some(highlighted_node) = highlighted_node {
                    let layout_node = layout.get(highlighted_node);
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

                self.sync(window.id(), rdom, &layout);
            }
            PluginEvent::WindowCreated { .. } => {
                if self.init.is_none() {
                    let nodes = self.nodes.clone();
                    let websockets = self.websockets.clone();
                    let highlighted_node = self.highlighted_node.clone();
                    let plugin_handle = plugin_handle.clone();
                    tokio::spawn(async move {
                        run_server(nodes, websockets, highlighted_node, plugin_handle)
                            .await
                            .unwrap();
                    });
                    self.init.replace(());
                }
            }
            _ => {}
        }
    }
}
