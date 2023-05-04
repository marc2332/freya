use dioxus_native_core::NodeId;
pub use euclid::Rect;

use crate::{
    geometry::{Area, Size2D},
    node::Node,
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
}

pub trait NodeKey: Clone + PartialEq + Eq + std::hash::Hash + Copy + std::fmt::Debug {}

impl NodeKey for usize {}
impl NodeKey for NodeId {}

pub trait NodeResolver<NodeKey> {
    /// Get a Node's Size configuration
    fn get_node(&self, node_id: &NodeKey) -> Option<Node>;

    /// Get the height in the DOM of the given Node
    fn height(&self, node_id: &NodeKey) -> Option<u16>;

    /// Get the parent's Node ID from the given Node
    fn parent_of(&self, node_id: &NodeKey) -> Option<NodeKey>;

    /// Get a list of IDs of all the Nodes from the given Node
    fn children_of(&self, node_id: &NodeKey) -> Vec<NodeKey>;
}
