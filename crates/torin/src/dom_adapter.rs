pub use euclid::Rect;

use crate::{
    geometry::{Area, Size2D},
    node::Node,
    prelude::{BoxModel, Gaps},
};

/// Cached layout results of a Node
#[derive(Debug, PartialEq, Clone, Default)]
pub struct NodeAreas {
    /// Area that ocuppies this node
    pub area: Area,

    /// Area inside this Node
    pub inner_area: Area,

    /// Ocuppied sizes from the inner children in this Node
    pub inner_sizes: Size2D,

    /// Outer margin
    pub margin: Gaps,
}

impl NodeAreas {
    // The area without any outer gap (e.g margin)
    pub fn box_area(&self) -> Area {
        self.area.box_area(&self.margin)
    }
}

pub trait NodeKey: Clone + PartialEq + Eq + std::hash::Hash + Copy + std::fmt::Debug {}

impl NodeKey for usize {}

#[cfg(feature = "dioxus")]
impl NodeKey for dioxus_native_core::NodeId {}

pub trait DOMAdapter<NodeKey> {
    /// Get the Node size
    fn get_node(&self, node_id: &NodeKey) -> Option<Node>;

    /// Get the height in the DOM of the given Node
    fn height(&self, node_id: &NodeKey) -> Option<u16>;

    /// Get the parent of a Node
    fn parent_of(&self, node_id: &NodeKey) -> Option<NodeKey>;

    /// Get the children of a Node
    fn children_of(&mut self, node_id: &NodeKey) -> Vec<NodeKey>;

    /// Check whether the given Node is valid (isn't a placeholder, unconnected node..)
    fn is_node_valid(&mut self, node_id: &NodeKey) -> bool;

    /// Get the closest common parent Node of two Nodes
    fn closest_common_parent(&self, node_id_a: &NodeKey, node_id_b: &NodeKey) -> Option<NodeKey>;
}
