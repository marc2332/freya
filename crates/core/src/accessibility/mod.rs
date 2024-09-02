pub mod accessibility_manager;
pub use accessibility_manager::*;
use freya_native_core::{
    node::NodeType,
    real_dom::NodeImmutable,
    tags::TagName,
    NodeId,
};
use freya_node_state::AccessibilityState;
use torin::torin::Torin;

use crate::{
    dom::{
        DioxusDOM,
        DioxusNode,
    },
    types::AccessibilityId,
};

/// Direction for the next Accessibility Node to be focused.
#[derive(PartialEq)]
pub enum AccessibilityFocusDirection {
    Forward,
    Backward,
}

/// Shortcut functions to retrieve Acessibility info from a Dioxus Node
trait NodeAccessibility {
    /// Return the first text node from this Node
    fn get_inner_texts(&self) -> Option<String>;

    /// Collect all the AccessibilityIDs from a Node's children
    fn get_accessibility_children(&self) -> Vec<AccessibilityId>;
}

impl NodeAccessibility for DioxusNode<'_> {
    /// Return the first text node from this Node
    fn get_inner_texts(&self) -> Option<String> {
        let children = self.children();
        let first_child = children.first()?;
        let node_type = first_child.node_type();
        if let NodeType::Text(text) = &*node_type {
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
                if child.node_type().is_visible_element() {
                    if let Some(accessibility_state) = &*child.get::<Option<AccessibilityState>>().unwrap() {
                        Some(accessibility_state.id)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<AccessibilityId>>()
    }
}

pub fn process_accessibility(
    layout: &Torin<NodeId>,
    rdom: &DioxusDOM,
    accessibility_manager: &mut AccessibilityManager,
) {
    rdom.traverse_depth_first_advanced(|node| {
        if !node.node_type().is_element() {
            return false;
        }

        let layout_node = layout.get(node.id()).unwrap();
        if let Some(accessibility_state) = &*node.get::<Option<AccessibilityState>>().unwrap() {
            accessibility_manager.add_node(
                &node,
                layout_node,
                accessibility_state
            );
        }

        if let Some(tag) = node.node_type().tag() {
            if *tag == TagName::Paragraph || *tag == TagName::Label {
                return false;
            }
        }

        true
    });
}
