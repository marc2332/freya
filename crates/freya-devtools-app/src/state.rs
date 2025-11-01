use std::{
    collections::{
        HashMap,
        HashSet,
    },
    sync::Arc,
};

use async_lock::Mutex;
use async_tungstenite::WebSocketSender;
use freya_core::integration::NodeId;
use freya_devtools::NodeInfo;
use freya_radio::hooks::RadioChannel;
use smol::net::TcpStream;

pub struct DevtoolsState {
    pub(crate) nodes: HashMap<u64, Vec<NodeInfo>>,
    pub(crate) expanded_nodes: HashSet<(u64, NodeId)>,
    pub(crate) client: Arc<Mutex<Option<WebSocketSender<TcpStream>>>>,
    pub(crate) animation_speed: f32,
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub enum DevtoolsChannel {
    Global,
    UpdatedTree,
    Misc,
}

impl RadioChannel<DevtoolsState> for DevtoolsChannel {}
