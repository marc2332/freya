use dioxus_radio::prelude::use_radio;
use freya_devtools::NodeInfo;
use freya_native_core::prelude::NodeId;

use crate::state::DevtoolsChannel;

pub fn use_node_info(node_id: NodeId) -> Option<NodeInfo> {
    let radio = use_radio(DevtoolsChannel::UpdatedDOM);
    let state = radio.read();

    state.nodes.iter().find(|node| node.id == node_id).cloned()
}
