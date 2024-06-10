use crate::state::DevtoolsChannel;
use dioxus_radio::prelude::use_radio;
use freya_native_core::prelude::NodeId;
use freya_renderer::devtools::NodeInfo;

pub fn use_node_info(node_id: NodeId) -> Option<NodeInfo> {
    let radio = use_radio(DevtoolsChannel::UpdatedDOM);
    let state = radio.read();
    let nodes = state.devtools_receiver.borrow();

    nodes.iter().find(|node| node.id == node_id).cloned()
}
