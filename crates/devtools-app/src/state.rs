use std::{
    collections::{
        HashMap,
        HashSet,
    },
    sync::Arc,
};

use dioxus_radio::prelude::*;
use freya_devtools::NodeInfo;
use freya_native_core::prelude::NodeId;
use futures_util::stream::SplitSink;
use tokio::{
    net::TcpStream,
    sync::Mutex,
};
use tokio_tungstenite::{
    MaybeTlsStream,
    WebSocketStream,
    tungstenite::Message,
};

pub type WebSocket = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;

pub struct DevtoolsState {
    pub(crate) nodes: HashMap<u64, Vec<NodeInfo>>,
    pub(crate) expanded_nodes: HashSet<(u64, NodeId)>,
    pub(crate) client: Arc<Mutex<Option<WebSocket>>>,
    pub(crate) animation_speed: f32,
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub enum DevtoolsChannel {
    Global,
    UpdatedDOM,
    Misc,
}

impl RadioChannel<DevtoolsState> for DevtoolsChannel {}
