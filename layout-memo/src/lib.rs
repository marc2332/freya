use std::collections::HashMap;

use dioxus_core::ElementId;

#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub struct NodeArea {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Default)]
pub struct LayoutManager {
    pub nodes: HashMap<ElementId, (NodeArea, NodeArea, NodeArea, (f32, f32))>,
    pub dirty_nodes: HashMap<ElementId, DirtyCause>,
    pub is_calculating: bool,
}

// TODO(marc2332) Remove this as it's basically useless
#[derive(Debug)]
pub enum DirtyCause {
    IChanged,
    BrotherChanged,
    ParentChanged,
    ChildChanged,
}

impl LayoutManager {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            dirty_nodes: HashMap::new(),
            is_calculating: false,
        }
    }

    pub fn set_calculating(&mut self, is_calculating: bool) {
        self.is_calculating = is_calculating;
    }

    pub fn does_exist(&mut self, element_id: &ElementId) -> bool {
        self.nodes.contains_key(element_id)
    }

    pub fn add_node(
        &mut self,
        element_id: ElementId,
        area: NodeArea,
        remaining_inner_area: NodeArea,
        inner_area: NodeArea,
        inner_sizes: (f32, f32),
    ) {
        self.nodes.insert(
            element_id,
            (area, remaining_inner_area, inner_area, inner_sizes),
        );
    }

    pub fn is_dirty(&self, element_id: &ElementId) -> bool {
        self.dirty_nodes.contains_key(element_id)
    }

    pub fn get_dirty_cause(&self, element_id: &ElementId) -> Option<&DirtyCause> {
        self.dirty_nodes.get(element_id)
    }

    pub fn mark_as_dirty(&mut self, element_id: ElementId, cause: DirtyCause) {
        self.dirty_nodes.insert(element_id, cause);
    }

    pub fn remove_as_dirty(&mut self, element_id: &ElementId) {
        self.dirty_nodes.remove(element_id);
    }

    pub fn get_node(
        &mut self,
        element_id: &ElementId,
    ) -> Option<(NodeArea, NodeArea, NodeArea, (f32, f32))> {
        self.nodes.get(element_id).copied()
    }
}
