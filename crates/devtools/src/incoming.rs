use freya_native_core::NodeId;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct Incoming {
    id: usize,
    message: IncomingMessage,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum IncomingMessage {
    HighlightNode(NodeId),
}
