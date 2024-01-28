pub mod accessibility_manager;
pub use accessibility_manager::*;

use accesskit::NodeId as AccessibilityId;
use dioxus_native_core::{
    node::{NodeType, TextNode},
    real_dom::NodeImmutable,
    NodeId,
};
use freya_dom::dom::{DioxusDOM, DioxusNode};
use freya_node_state::AccessibilityNodeState;
use torin::torin::Torin;

use crate::layout::Layers;

/// Direction for the next Accessibility Node to be focused.
#[derive(PartialEq)]
pub enum AccessibilityFocusDirection {
    Forward,
    Backward,
}

/// Shortcut functions to retrieve Acessibility info from a Dioxus Node
trait NodeAccessibility {
    /// Return the first TextNode from this Node
    fn get_inner_texts(&self) -> Option<String>;

    /// Collect all the AccessibilityIDs from a Node's children
    fn get_accessibility_children(&self) -> Vec<AccessibilityId>;
}

impl NodeAccessibility for DioxusNode<'_> {
    /// Return the first TextNode from this Node
    fn get_inner_texts(&self) -> Option<String> {
        let children = self.children();
        let first_child = children.first()?;
        let node_type = first_child.node_type();
        if let NodeType::Text(TextNode { text, .. }) = &*node_type {
            Some(text.to_owned())
        } else {
            None
        }
    }

    /// Collect all the AccessibilityIDs from a Node's children
    fn get_accessibility_children(&self) -> Vec<AccessibilityId> {
        self.children()
            .iter()
            .filter_map(|child| {
                let node_accessibility = &*child.get::<AccessibilityNodeState>().unwrap();
                node_accessibility.accessibility_id
            })
            .collect::<Vec<AccessibilityId>>()
    }
}

pub fn process_accessibility(
    layers: &Layers,
    layout: &Torin<NodeId>,
    rdom: &DioxusDOM,
    accessibility_manager: &mut AccessibilityManager,
) {
    for layer in layers.layers.values() {
        for node_id in layer {
            let node_areas = layout.get(*node_id).unwrap();
            let dioxus_node = rdom.get(*node_id);
            if let Some(dioxus_node) = dioxus_node {
                let node_accessibility = &*dioxus_node.get::<AccessibilityNodeState>().unwrap();
                if let Some(accessibility_id) = node_accessibility.accessibility_id {
                    accessibility_manager.add_node(
                        &dioxus_node,
                        node_areas,
                        accessibility_id,
                        node_accessibility,
                    );
                }
            }
        }
    }
}
