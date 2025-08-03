use std::collections::{
    HashMap,
    HashSet,
};

use dioxus_radio::prelude::*;
use freya_devtools::NodeInfo;
use freya_native_core::prelude::NodeId;

pub struct DevtoolsState {
    pub(crate) nodes: HashMap<u64, Vec<NodeInfo>>,
    pub(crate) expanded_nodes: HashSet<(u64, NodeId)>,
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub enum DevtoolsChannel {
    Global,
    UpdatedDOM,
}

impl RadioChannel<DevtoolsState> for DevtoolsChannel {}
