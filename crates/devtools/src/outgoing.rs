use serde::{
    Deserialize,
    Serialize,
};

use crate::node_info::NodeInfo;

#[derive(Deserialize, Serialize)]
pub struct OutgoingMessage {
    pub action: OutgoingMessageAction,
}

#[derive(Deserialize, Serialize)]
pub enum OutgoingMessageAction {
    Update {
        window_id: u64,
        nodes: Vec<NodeInfo>,
    },
}
