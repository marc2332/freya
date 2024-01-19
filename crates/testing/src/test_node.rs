use dioxus_native_core::prelude::TextNode;
use dioxus_native_core::NodeId;
use dioxus_native_core::{node::NodeType, real_dom::NodeImmutable};
use freya_core::node::NodeState;
use freya_node_state::CustomAttributeValues;
use torin::geometry::Area;

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
    /// Quickly get a child of the Node by the given index, if the child is not found it will panic
    pub fn get(&self, child_index: usize) -> Self {
        self.child(child_index).unwrap_or_else(|| {
            panic!("Child by index {} not found", child_index);
        })
    }

    /// Get a child of the Node by the given index
    pub fn child(&self, child_index: usize) -> Option<Self> {
        let child_id = self.children_ids.get(child_index)?;
        let child: TestNode = self.utils.get_node_by_id(*child_id);
        Some(child)
    }

    /// Get the Node text
    pub fn text(&self) -> Option<&str> {
        if let NodeType::Text(TextNode { text, .. }) = &self.node_type {
            Some(text)
        } else {
            None
        }
    }

    /// Get the Node state
    pub fn state(&self) -> &NodeState {
        &self.state
    }

    /// Get the Node layout
    pub fn layout(&self) -> Option<Area> {
        self.utils()
            .sdom()
            .get()
            .layout()
            .get(self.node_id)
            .map(|l| l.area)
    }

    /// Get a mutable reference to the test utils.
    pub fn utils(&self) -> &TestUtils {
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
}
