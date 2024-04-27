use freya_native_core::{prelude::NodeType, real_dom::NodeImmutable, tree::TreeRef, NodeId};
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
        let contains_text = node
            .node_type()
            .tag()
            .map(|t| t.contains_text())
            .unwrap_or_default();

        let mut layout = node.get::<LayoutState>()?.clone();

        // The root node expands by default
        if *node_id == self.rdom.root_id() {
            layout.width = Size::Percentage(Length::new(100.0));
            layout.height = Size::Percentage(Length::new(100.0));
        }

        Some(Node {
            width: layout.width,
            height: layout.height,
            minimum_width: layout.minimum_width,
            minimum_height: layout.minimum_height,
            maximum_width: layout.maximum_width,
            maximum_height: layout.maximum_height,
            direction: layout.direction,
            padding: layout.padding,
            margin: layout.margin,
            main_alignment: layout.main_alignment,
            cross_alignment: layout.cross_alignment,
            offset_x: layout.offset_x,
            offset_y: layout.offset_y,
            has_layout_references: layout.node_ref.is_some(),
            position: layout.position,
            content: layout.content,
            contains_text,
        })
    }

    fn height(&self, node_id: &NodeId) -> Option<u16> {
        self.rdom.tree_ref().height(*node_id)
    }

    fn parent_of(&self, node_id: &NodeId) -> Option<NodeId> {
        self.rdom.tree_ref().parent_id(*node_id)
    }

    fn children_of(&mut self, node_id: &NodeId) -> Vec<NodeId> {
        let mut children = self.rdom.tree_ref().children_ids(*node_id);
        children.retain(|id| is_node_valid(self.rdom, &mut self.valid_nodes_cache, id));
        children
    }

    fn is_node_valid(&mut self, node_id: &NodeId) -> bool {
        is_node_valid(self.rdom, &mut self.valid_nodes_cache, node_id)
    }

    fn root_id(&self) -> NodeId {
        self.rdom.root_id()
    }
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
