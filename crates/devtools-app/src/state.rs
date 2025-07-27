use std::collections::HashSet;

use dioxus_radio::prelude::*;
use freya_devtools::NodeInfo;
use freya_native_core::prelude::NodeId;

pub struct DevtoolsState {
    pub(crate) nodes: Vec<NodeInfo>,
    pub(crate) expanded_nodes: HashSet<NodeId>, // pub(crate) devtools_tree: HashSet<NodeId>,
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub enum DevtoolsChannel {
    Global,
    UpdatedDOM,
}

impl RadioChannel<DevtoolsState> for DevtoolsChannel {}
