use dioxus_radio::prelude::*;
use freya_renderer::{devtools::DevtoolsReceiver, HoveredNode};

pub struct DevtoolsState {
    pub(crate) hovered_node: HoveredNode,
    pub(crate) devtools_receiver: DevtoolsReceiver,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum DevtoolsChannel {
    Global,
    UpdatedDOM,
}

impl RadioChannel<DevtoolsState> for DevtoolsChannel {}
