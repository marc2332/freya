use dioxus::prelude::*;
use dioxus_native_core::NodeId;

use crate::TreeNode;

pub fn use_selected_node(cx: &ScopeState, node_id: &NodeId) -> Option<TreeNode> {
    let children = use_shared_state::<Vec<TreeNode>>(cx)?;
    let children = children.read();

    let node = children.iter().find(|node| &node.id == node_id)?;

    Some(node.clone())
}
