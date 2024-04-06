use std::sync::Arc;

pub use euclid::Rect;
use freya_native_core::SendAnyMap;

use crate::{
    geometry::{Area, Size2D},
    node::Node,
    prelude::{AreaModel, Gaps},
};

/// Cached layout results of a Node
#[derive(Debug, Default, Clone)]
pub struct LayoutNode {
    /// Area that ocuppies this node
    pub area: Area,

    /// Area inside this Node
    pub inner_area: Area,

    /// Ocuppied sizes from the inner children in this Node
    pub inner_sizes: Size2D,

    /// Outer margin
    pub margin: Gaps,

    /// Associated data
    pub data: Option<Arc<SendAnyMap>>,
}

impl PartialEq for LayoutNode {
    fn eq(&self, other: &Self) -> bool {
        self.area == other.area
            && self.inner_area == other.inner_area
            && self.inner_sizes == other.inner_sizes
            && self.margin == other.margin
    }
}

impl LayoutNode {
    // The area without any margin
    pub fn visible_area(&self) -> Area {
        self.area.after_gaps(&self.margin)
    }
}

pub trait NodeKey: Clone + PartialEq + Eq + std::hash::Hash + Copy + std::fmt::Debug {}

impl NodeKey for usize {}

#[cfg(feature = "dioxus")]
impl NodeKey for freya_native_core::NodeId {}

pub trait DOMAdapter<Key: NodeKey> {
    fn root_id(&self) -> Key;

    /// Get the Node size
    fn get_node(&self, node_id: &Key) -> Option<Node>;

    /// Get the height in the DOM of the given Node
    fn height(&self, node_id: &Key) -> Option<u16>;

    /// Get the parent of a Node
    fn parent_of(&self, node_id: &Key) -> Option<Key>;

    /// Get the children of a Node
    fn children_of(&mut self, node_id: &Key) -> Vec<Key>;

    /// Check whether the given Node is valid (isn't a placeholder, unconnected node..)
    fn is_node_valid(&mut self, node_id: &Key) -> bool;

    /// Get the closest common parent Node of two Nodes
    fn closest_common_parent(&self, node_a: &Key, node_b: &Key) -> Option<Key> {
        let height_a = self.height(node_a)?;
        let height_b = self.height(node_b)?;

        let (node_a, node_b) = match height_a.cmp(&height_b) {
            std::cmp::Ordering::Less => (
                *node_a,
                balance_heights(self, *node_b, *node_a).unwrap_or(*node_b),
            ),
            std::cmp::Ordering::Equal => (*node_a, *node_b),
            std::cmp::Ordering::Greater => (
                balance_heights(self, *node_a, *node_b).unwrap_or(*node_a),
                *node_b,
            ),
        };

        let mut currents = (node_a, node_b);

        loop {
            // Common parent of node_a and node_b
            if currents.0 == currents.1 {
                return Some(currents.0);
            }

            let parent_a = self.parent_of(&currents.0);
            if let Some(parent_a) = parent_a {
                currents.0 = parent_a;
            } else if self.root_id() != currents.0 {
                // Skip unconected nodes
                break;
            }

            let parent_b = self.parent_of(&currents.1);
            if let Some(parent_b) = parent_b {
                currents.1 = parent_b;
            } else if self.root_id() != currents.1 {
                // Skip unconected nodes
                break;
            }
        }

        None
    }
}

/// Walk to the ancestor of `base` with the same height of `target`
fn balance_heights<Key: NodeKey>(
    dom_adapter: &(impl DOMAdapter<Key> + ?Sized),
    base: Key,
    target: Key,
) -> Option<Key> {
    let target_height = dom_adapter.height(&target)?;
    let mut current = base;
    loop {
        if dom_adapter.height(&current)? == target_height {
            break;
        }

        let parent_current = dom_adapter.parent_of(&current);
        if let Some(parent_current) = parent_current {
            current = parent_current;
        }
    }
    Some(current)
}
