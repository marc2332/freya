use freya_core::node::NodeState;
use freya_native_core::{
    NodeId,
    tags::TagName,
};
use serde::{
    Deserialize,
    Serialize,
};
use torin::prelude::LayoutNode;

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

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct NodeInfo {
    pub window_id: u64,
    pub is_window: bool,
    pub id: NodeId,
    pub parent_id: Option<NodeId>,
    pub children_len: usize,
    pub tag: TagName,
    pub height: u16,
    pub state: NodeState,
    pub layout_node: LayoutNode,
}
