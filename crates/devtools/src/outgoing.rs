use serde::{
    Deserialize,
    Serialize,
};

use crate::node_info::NodeInfo;

#[derive(Deserialize, Serialize)]
pub struct Outgoing {
    pub notification: OutgoingNotification,
}

#[derive(Deserialize, Serialize)]
pub enum OutgoingNotification {
    Update {
        window_id: u64,
        nodes: Vec<NodeInfo>,
    },
}
