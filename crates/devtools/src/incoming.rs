use freya_native_core::NodeId;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct IncomingMessage {
    pub action: IncomingMessageAction,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum IncomingMessageAction {
    HighlightNode { window_id: u64, node_id: NodeId },
    SetSpeedTo { speed: f32 },
}
