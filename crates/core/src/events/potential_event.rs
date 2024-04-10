use freya_native_core::NodeId;

use crate::prelude::PlatformEvent;

/// Potential events are events that might get emitted or not.
#[derive(Clone, Debug)]
pub struct PotentialEvent {
    pub(crate) node_id: NodeId,
    pub(crate) event: PlatformEvent,
    pub(crate) layer: Option<i16>,
}
