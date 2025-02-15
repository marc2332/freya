mod tree;
use freya_native_core::{
    node::NodeType,
    real_dom::NodeImmutable,
};
use itertools::Itertools;
pub use tree::*;

use crate::{
    dom::DioxusNode,
    states::AccessibilityNodeState,
    types::AccessibilityId,
};

/// Shortcut functions to retrieve Acessibility info from a Dioxus Node
pub trait NodeAccessibility {
    fn get_accessibility_id(&self) -> Option<AccessibilityId>;

    /// Return the first text node from this Node
    fn get_inner_texts(&self) -> Option<String>;

    /// Collect all the AccessibilityIDs from a Node's children
    fn get_accessibility_children(&self) -> Vec<AccessibilityId>;
}

impl NodeAccessibility for DioxusNode<'_> {
    fn get_accessibility_id(&self) -> Option<AccessibilityId> {
        if self.id() == self.real_dom().root_id() {
            Some(ACCESSIBILITY_ROOT_ID)
        } else {
            let node_accessibility = &*self.get::<AccessibilityNodeState>()?;
            node_accessibility.a11y_id
        }
    }

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

    /// Collect all descendant accessibility node ids
    fn get_accessibility_children(&self) -> Vec<AccessibilityId> {
        self.children()
            .into_iter()
            .filter_map(|child| child.get_accessibility_id())
            .collect_vec()
    }
}
