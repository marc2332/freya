use dioxus_native_core::{prelude::NodeType, real_dom::NodeImmutable, tree::TreeRef, NodeId};
use freya_node_state::LayoutState;
use rustc_hash::FxHashMap;
use torin::prelude::*;

use crate::dom::DioxusDOM;

/// RealDOM adapter for Torin.
pub struct DioxusDOMAdapter<'a> {
    pub rdom: &'a DioxusDOM,

    valid_nodes_cache: Option<FxHashMap<NodeId, bool>>,
}

impl<'a> DioxusDOMAdapter<'a> {
    pub fn new(rdom: &'a DioxusDOM) -> Self {
        Self {
            rdom,
            valid_nodes_cache: None,
        }
    }

    pub fn new_with_cache(rdom: &'a DioxusDOM) -> Self {
        Self {
            rdom,
            valid_nodes_cache: Some(FxHashMap::default()),
        }
    }
}

impl DOMAdapter<NodeId> for DioxusDOMAdapter<'_> {
    fn get_node(&self, node_id: &NodeId) -> Option<Node> {
        let node = self.rdom.get(*node_id)?;
        let mut size = node.get::<LayoutState>().unwrap().clone();

        // The root node expands by default
        if *node_id == self.rdom.root_id() {
            size.width = Size::Percentage(Length::new(100.0));
            size.height = Size::Percentage(Length::new(100.0));
        }

        Some(Node {
            width: size.width,
            height: size.height,
            minimum_width: size.minimum_width,
            minimum_height: size.minimum_height,
            maximum_width: size.maximum_width,
            maximum_height: size.maximum_height,
            direction: size.direction,
            padding: size.padding,
            margin: size.margin,
            main_alignment: size.main_alignment,
            cross_alignment: size.cross_alignment,
            offset_x: Length::new(size.offset_x),
            offset_y: Length::new(size.offset_y),
            has_layout_references: size.node_ref.is_some(),
        })
    }

    fn height(&self, node_id: &NodeId) -> Option<u16> {
        self.rdom.tree_ref().height(*node_id)
    }

    fn parent_of(&self, node_id: &NodeId) -> Option<NodeId> {
        self.rdom.tree_ref().parent_id(*node_id)
    }

    fn children_of(&mut self, node_id: &NodeId) -> Vec<NodeId> {
        self.rdom
            .tree_ref()
            .children_ids(*node_id)
            .into_iter()
            .filter(|id| is_node_valid(self.rdom, &mut self.valid_nodes_cache, id))
            .collect::<Vec<NodeId>>()
    }

    fn is_node_valid(&mut self, node_id: &NodeId) -> bool {
        is_node_valid(self.rdom, &mut self.valid_nodes_cache, node_id)
    }

    fn closest_common_parent(&self, node_id_a: &NodeId, node_id_b: &NodeId) -> Option<NodeId> {
        find_common_parent(self.rdom, *node_id_a, *node_id_b)
    }
}

/// Walk to the ancestor of `base` with the same height of `target`
fn balance_heights(rdom: &DioxusDOM, base: NodeId, target: NodeId) -> Option<NodeId> {
    let tree = rdom.tree_ref();
    let target_height = tree.height(target)?;
    let mut current = base;
    loop {
        if tree.height(current)? == target_height {
            break;
        }

        let parent_current = tree.parent_id(current);
        if let Some(parent_current) = parent_current {
            current = parent_current;
        }
    }
    Some(current)
}

/// Return the closest common ancestor of both Nodes
fn find_common_parent(rdom: &DioxusDOM, node_a: NodeId, node_b: NodeId) -> Option<NodeId> {
    let tree = rdom.tree_ref();
    let height_a = tree.height(node_a)?;
    let height_b = tree.height(node_b)?;

    let (node_a, node_b) = match height_a.cmp(&height_b) {
        std::cmp::Ordering::Less => (
            node_a,
            balance_heights(rdom, node_b, node_a).unwrap_or(node_b),
        ),
        std::cmp::Ordering::Equal => (node_a, node_b),
        std::cmp::Ordering::Greater => (
            balance_heights(rdom, node_a, node_b).unwrap_or(node_a),
            node_b,
        ),
    };

    let mut currents = (node_a, node_b);

    loop {
        // Common parent of node_a and node_b
        if currents.0 == currents.1 {
            return Some(currents.0);
        }

        let parent_a = tree.parent_id(currents.0);
        if let Some(parent_a) = parent_a {
            currents.0 = parent_a;
        } else if rdom.root_id() != currents.0 {
            // Skip unconected nodes
            break;
        }

        let parent_b = tree.parent_id(currents.1);
        if let Some(parent_b) = parent_b {
            currents.1 = parent_b;
        } else if rdom.root_id() != currents.1 {
            // Skip unconected nodes
            break;
        }
    }

    None
}

/// Check is the given Node is valid or not, this means not being a placeholder or an unconnected Node.
fn is_node_valid(
    rdom: &DioxusDOM,
    valid_nodes_cache: &mut Option<FxHashMap<NodeId, bool>>,
    node_id: &NodeId,
) -> bool {
    // Check if Node was valid from cache
    if let Some(valid_nodes_cache) = valid_nodes_cache {
        if let Some(is_valid) = valid_nodes_cache.get(node_id) {
            return *is_valid;
        }
    }

    let node = rdom.get(*node_id);

    let is_valid = 'validation: {
        if let Some(node) = node {
            let is_placeholder = matches!(*node.node_type(), NodeType::Placeholder);

            // Placeholders can't be measured
            if is_placeholder {
                break 'validation false;
            }

            // Make sure this Node isn't part of an unconnected Node
            // This walkes up to the ancestor that has a height of 0 and checks if it has the same ID as the root Node
            // If it has the same ID, it means that is not an unconnected ID, otherwise, it is and should be skipped.
            let tree = rdom.tree_ref();
            let mut current = *node_id;
            loop {
                let height = tree.height(current);
                if let Some(height) = height {
                    if height == 0 {
                        break;
                    }
                }

                let parent_current = tree.parent_id(current);
                if let Some(parent_current) = parent_current {
                    current = parent_current;
                }
            }

            current == rdom.root_id()
        } else {
            false
        }
    };

    // Save the validation result in the cache
    if let Some(valid_nodes_cache) = valid_nodes_cache {
        valid_nodes_cache.insert(*node_id, is_valid);
    }

    is_valid
}
