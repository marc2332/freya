use std::collections::HashSet;

use dioxus_radio::prelude::*;
use freya_native_core::prelude::NodeId;
use freya_renderer::{
    devtools::DevtoolsReceiver,
    HoveredNode,
};

pub struct DevtoolsState {
    pub(crate) hovered_node: HoveredNode,
    pub(crate) devtools_receiver: DevtoolsReceiver,
    pub(crate) devtools_tree: HashSet<NodeId>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum DevtoolsChannel {
    Global,
    UpdatedDOM,
}

impl RadioChannel<DevtoolsState> for DevtoolsChannel {}
