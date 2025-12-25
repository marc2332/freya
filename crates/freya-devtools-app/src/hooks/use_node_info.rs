use freya_core::integration::NodeId;
use freya_devtools::NodeInfo;
use freya_radio::hooks::use_radio;

use crate::state::DevtoolsChannel;

pub fn use_node_info(node_id: NodeId, window_id: u64) -> Option<NodeInfo> {
    let radio = use_radio(DevtoolsChannel::UpdatedTree);
    let state = radio.read();

    state
        .nodes
        .get(&window_id)?
        .iter()
        .find(|node| node.node_id == node_id)
        .cloned()
}
