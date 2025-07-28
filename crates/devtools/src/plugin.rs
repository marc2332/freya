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
    nodes: Arc<Mutex<Vec<NodeInfo>>>,
    websockets: SharedWebsockets,
    init: Option<()>,
}

impl FreyaPlugin for DevtoolsPlugin {
    fn on_event(&mut self, event: &PluginEvent, _handle: PluginHandle) {
        match event {
            PluginEvent::AfterRender { freya_dom, .. } => {
                if self.init.is_none() {
                    let nodes = self.nodes.clone();
                    let websockets = self.websockets.clone();
                    tokio::spawn(async move {
                        run_server(nodes, websockets).await.unwrap();
                    });
                    self.init.replace(());
                }

                let rdom = freya_dom.rdom();
                let layout = freya_dom.layout();

                let mut new_nodes = Vec::new();

                rdom.traverse_depth_first(|node| {
                    // Ignore root element and NativeContainer
                    if node.height() >= 2 {
                        let layout_node = layout.get(node.id()).cloned();
                        if let Some(layout_node) = layout_node {
                            let node_type = node.node_type();
                            new_nodes.push(NodeInfo {
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
                *self.nodes.lock().unwrap() = new_nodes;

                // Notify the existing subscribers of this change
                let outgoing_message = Message::Text(
                    serde_json::to_string(&Outgoing {
                        notification: OutgoingNotification::Nodes(
                            self.nodes.lock().unwrap().clone(),
                        ),
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
            PluginEvent::WindowCreated(_) => {}
            _ => {}
        }
    }
}
