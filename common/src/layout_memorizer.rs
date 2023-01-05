use std::collections::HashMap;

use dioxus_native_core::NodeId;

use crate::NodeArea;

/// Info about different areas and sizes of a certain node.
#[derive(Debug, Clone)]
pub struct NodeLayoutInfo {
    pub area: NodeArea,
    pub remaining_inner_area: NodeArea,
    pub inner_area: NodeArea,
    pub inner_sizes: (f32, f32),
}

/// Stores all the nodes layout and what nodes should be calculated again on the next check.
#[derive(Debug, Default)]
pub struct LayoutMemorizer {
    pub nodes: HashMap<NodeId, NodeLayoutInfo>,
    pub dirty_nodes: HashMap<NodeId, ()>,
    #[cfg(debug_assertions)]
    pub dirty_nodes_counter: i32,
}

impl LayoutMemorizer {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            dirty_nodes: HashMap::new(),
            #[cfg(debug_assertions)]
            dirty_nodes_counter: 0,
        }
    }

    /// Check if a node's layout is memorized or not
    pub fn is_node_layout_memorized(&mut self, node_id: &NodeId) -> bool {
        self.nodes.contains_key(node_id)
    }

    /// Memorize a node's layout
    pub fn add_node_layout(&mut self, node_id: NodeId, layout_info: NodeLayoutInfo) {
        self.nodes.insert(node_id, layout_info);
    }

    /// Check if a node's layout is no longer valid
    pub fn is_dirty(&self, node_id: &NodeId) -> bool {
        self.dirty_nodes.contains_key(node_id)
    }

    /// Mark a node's layout as no longer valid
    pub fn mark_as_dirty(&mut self, node_id: NodeId) {
        #[cfg(debug_assertions)]
        {
            if !self.dirty_nodes.contains_key(&node_id) {
                self.dirty_nodes_counter += 1;
            }
        }
        self.dirty_nodes.insert(node_id, ());
    }

    // Unmark a node's layout as no longer valid
    pub fn remove_as_dirty(&mut self, node_id: &NodeId) {
        self.dirty_nodes.remove(node_id);
    }

    // Get the memorized layout of a certain node
    pub fn get_node_layout(&mut self, node_id: &NodeId) -> Option<NodeLayoutInfo> {
        self.nodes.get(node_id).cloned()
    }
}
