use dioxus_native_core::NodeId;

use crate::prelude::FreyaEvent;

#[derive(Clone, Debug)]
pub struct PotentialEvent {
    pub(crate) node_id: NodeId,
    pub(crate) event: FreyaEvent,
    pub(crate) layer: Option<i16>,
}
