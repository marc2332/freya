use freya_core::{
    custom_attributes::CustomAttributeValues,
    node::NodeState,
    states::{
        StyleState,
        ViewportState,
    },
};
use freya_native_core::{
    node::NodeType,
    real_dom::NodeImmutable,
    NodeId,
};
use torin::{
    geometry::Area,
    prelude::LayoutNode,
};

use crate::test_utils::TestUtils;

/// Represents a `Node` in the DOM.
#[derive(Clone)]
pub struct TestNode {
    pub(crate) node_id: NodeId,
    pub(crate) utils: TestUtils,
    pub(crate) height: u16,
    pub(crate) children_ids: Vec<NodeId>,
    pub(crate) state: NodeState,
    pub(crate) node_type: NodeType<CustomAttributeValues>,
}

impl TestNode {
    /// Get a node by its position in this node children list. Will panic if not found.
    #[track_caller]
    pub fn get(&self, child_index: usize) -> Self {
        self.try_get(child_index)
            .unwrap_or_else(|| panic!("Child by index {child_index} not found"))
    }

    /// Get a node by its position in this node children list.
    #[track_caller]
    pub fn try_get(&self, child_index: usize) -> Option<Self> {
        let child_id = self.children_ids.get(child_index)?;
        let child: TestNode = self.utils.get_node_by_id(*child_id);
        Some(child)
    }

    /// Get the Node text
    pub fn text(&self) -> Option<&str> {
        self.node_type.text()
    }

    /// Get the Node state
    pub fn state(&self) -> &NodeState {
        &self.state
    }

    /// Get the Node layout
    pub fn layout(&self) -> Option<LayoutNode> {
        self.utils()
            .sdom()
            .get()
            .layout()
            .get(self.node_id)
            .cloned()
    }

    /// Get the Node layout Area
    pub fn area(&self) -> Option<Area> {
        self.layout().map(|l| l.area)
    }

    /// Get the Node style
    pub fn style(&self) -> StyleState {
        self.utils
            .sdom
            .get()
            .rdom()
            .get(self.node_id)
            .unwrap()
            .get::<StyleState>()
            .unwrap()
            .clone()
    }

    /// Get a mutable reference to the test utils.
    pub(crate) fn utils(&self) -> &TestUtils {
        &self.utils
    }

    /// Get the NodeId from the parent
    pub fn parent_id(&self) -> Option<NodeId> {
        let sdom = self.utils().sdom();
        let fdom = sdom.get();
        let dom = fdom.rdom();
        let node = dom.get(self.node_id).unwrap();
        node.parent_id()
    }

    /// Get the Node height in the DOM
    pub fn dom_height(&self) -> u16 {
        self.height
    }

    /// Check if the Node is visible given it's viewports.
    pub fn is_visible(&self) -> bool {
        let Some(area) = self.area() else {
            return false;
        };
        let sdom = self.utils().sdom();
        let fdom = sdom.get();
        let dom = fdom.rdom();
        let node = dom.get(self.node_id).unwrap();

        let layout = fdom.layout();
        let node_viewports = node.get::<ViewportState>().unwrap();

        node_viewports.is_visible(&layout, &area)
    }

    /// Get the IDs of this Node children.
    pub fn children_ids(&self) -> Vec<NodeId> {
        self.children_ids.clone()
    }

    /// Check if this element is text
    pub fn is_element(&self) -> bool {
        self.node_type.is_element()
    }

    /// Check if this element is text
    pub fn is_text(&self) -> bool {
        self.node_type.is_text()
    }

    /// Check if this element is a placeholder
    pub fn is_placeholder(&self) -> bool {
        self.node_type.is_placeholder()
    }

    /// Get a descendant Node of this Node that matches a certain text.
    pub fn get_by_text(&self, matching_text: &str) -> Option<Self> {
        self.utils()
            .get_node_matching_inside_id(self.node_id, |node| {
                if let NodeType::Text(text) = &*node.node_type() {
                    matching_text == text
                } else {
                    false
                }
            })
            .first()
            .cloned()
    }
}
