use freya_native_core::{
    events::EventName,
    NodeId,
};

use super::PlatformEventData;

/// Potential events are events that might get emitted or not.
#[derive(Clone, Debug)]
pub struct PotentialEvent {
    pub(crate) node_id: NodeId,
    pub(crate) name: EventName,
    pub(crate) data: PlatformEventData,
    pub(crate) layer: Option<i16>,
}
