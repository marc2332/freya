use std::{
    collections::HashMap,
    sync::{
        Arc,
        Mutex,
    },
};

use freya_core::{
    node::get_node_state,
    plugins::{
        FreyaPlugin,
        PluginEvent,
        PluginHandle,
    },
};
use freya_native_core::prelude::NodeImmutable;
use futures::{
    SinkExt,
    stream::SplitSink,
};
use hyper::upgrade::Upgraded;
use hyper_util::rt::TokioIo;
use tokio_tungstenite::WebSocketStream;
use tungstenite::Message;

use crate::{
    Outgoing,
    OutgoingNotification,
    outgoing::NodeInfo,
    server::run_server,
};

pub(crate) type Websockets = HashMap<u32, SplitSink<WebSocketStream<TokioIo<Upgraded>>, Message>>;
pub(crate) type SharedWebsockets = Arc<tokio::sync::Mutex<Websockets>>;

#[derive(Default)]
pub struct DevtoolsPlugin {
    nodes: Arc<Mutex<HashMap<u64, Vec<NodeInfo>>>>,
    websockets: SharedWebsockets,
    init: Option<()>,
}

impl FreyaPlugin for DevtoolsPlugin {
    fn on_event(&mut self, event: &PluginEvent, _handle: PluginHandle) {
        match event {
            PluginEvent::WindowClosed { window, .. } => {
                let window_id: u64 = window.id().into();

                // Update nodes snapshot
                self.nodes.lock().unwrap().remove(&window_id);

                // Notify the existing subscribers of this change
                let outgoing_message = Message::Text(
                    serde_json::to_string(&Outgoing {
                        notification: OutgoingNotification::Update {
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
            PluginEvent::AfterRender { fdom, window, .. } => {
                let rdom = fdom.rdom();
                let layout = fdom.layout();

                let window_id: u64 = window.id().into();
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
                                id: node.id(),
                                parent_id: node.parent_id(),
                                children_len: node
                                    .children()
                                    .iter()
                                    .filter(|node| layout.get(node.id()).is_some())
                                    .count(),
                                tag: *node_type.tag().unwrap(),
                                height: node.height(),
                                state: get_node_state(&node),
                                layout_node,
                            });
                        }
                    }
                });

                // Update nodes snapshot
                self.nodes.lock().unwrap().insert(window_id, new_nodes);

                // Notify the existing subscribers of this change
                let outgoing_message = Message::Text(
                    serde_json::to_string(&Outgoing {
                        notification: OutgoingNotification::Update {
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
            PluginEvent::WindowCreated { .. } => {
                if self.init.is_none() {
                    let nodes = self.nodes.clone();
                    let websockets = self.websockets.clone();
                    tokio::spawn(async move {
                        run_server(nodes, websockets).await.unwrap();
                    });
                    self.init.replace(());
                }
            }
            _ => {}
        }
    }
}
