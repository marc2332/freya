use std::cmp::Ordering;

use freya_native_core::{
    events::EventName,
    NodeId,
};

use super::PlatformEventData;

/// Potential events are events that might get emitted or not.
#[derive(Clone, Debug, PartialEq)]
pub struct PotentialEvent {
    pub node_id: NodeId,
    pub name: EventName,
    pub data: PlatformEventData,
    pub layer: Option<i16>,
}

impl Eq for PotentialEvent {}

impl PartialOrd for PotentialEvent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PotentialEvent {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .layer
            .cmp(&self.layer)
            .then_with(|| self.name.cmp(&other.name))
    }
}
