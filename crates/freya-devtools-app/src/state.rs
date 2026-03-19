use std::{
    collections::{
        HashMap,
        HashSet,
    },
    sync::Arc,
};

use async_lock::Mutex;
use async_tungstenite::WebSocketSender;
use freya_core::{
    integration::NodeId,
    prelude::spawn,
};
use freya_devtools::{
    IncomingMessage,
    IncomingMessageAction,
    NodeInfo,
};
use freya_radio::hooks::RadioChannel;
use smol::net::TcpStream;
use tungstenite::Message;

pub struct DevtoolsState {
    pub(crate) nodes: HashMap<u64, Vec<NodeInfo>>,
    pub(crate) expanded_nodes: HashSet<(u64, NodeId)>,
    pub(crate) client: Arc<Mutex<Option<WebSocketSender<TcpStream>>>>,
    pub(crate) animation_speed: f32,
}

impl DevtoolsState {
    pub fn send_action(&self, action: IncomingMessageAction) {
        let message = Message::Text(
            serde_json::to_string(&IncomingMessage { action })
                .unwrap()
                .into(),
        );
        let client = self.client.clone();
        spawn(async move {
            if let Some(client) = client.lock().await.as_mut() {
                client.send(message).await.ok();
            }
        });
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub enum DevtoolsChannel {
    Global,
    UpdatedTree,
    Misc,
}

impl RadioChannel<DevtoolsState> for DevtoolsChannel {}
