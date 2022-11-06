use std::collections::HashMap;

use dioxus_core::ElementId;

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
    pub nodes: HashMap<ElementId, NodeLayoutInfo>,
    pub dirty_nodes: HashMap<ElementId, ()>,
}

impl LayoutMemorizer {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            dirty_nodes: HashMap::new(),
        }
    }

    /// Check if a node's layout is memorized or not
    pub fn is_node_layout_memorized(&mut self, element_id: &ElementId) -> bool {
        self.nodes.contains_key(element_id)
    }

    /// Memorize a node's layout
    pub fn add_node_layout(&mut self, element_id: ElementId, layout_info: NodeLayoutInfo) {
        self.nodes.insert(element_id, layout_info);
    }

    /// Check if a node's layout is no longer valid
    pub fn is_dirty(&self, element_id: &ElementId) -> bool {
        self.dirty_nodes.contains_key(element_id)
    }

    /// Mark a node's layout as no longer valid
    pub fn mark_as_dirty(&mut self, element_id: ElementId) {
        self.dirty_nodes.insert(element_id, ());
    }

    // Unmark a node's layout as no longer valid
    pub fn remove_as_dirty(&mut self, element_id: &ElementId) {
        self.dirty_nodes.remove(element_id);
    }

    // Get the memorized layout of a certain node
    pub fn get_node_layout(&mut self, element_id: &ElementId) -> Option<NodeLayoutInfo> {
        self.nodes.get(element_id).cloned()
    }
}
