use accesskit::{Node, NodeClassSet, NodeId as AccessibilityId};
use dioxus_native_core::{
    prelude::{NodeType, TextNode},
    real_dom::NodeImmutable,
};
use freya_core::accessibility::AccessibilityProvider;
use freya_dom::prelude::DioxusNode;
use freya_node_state::AccessibilitySettings;
use std::{
    num::NonZeroU128,
    sync::{Arc, Mutex},
};

pub type SharedAccessibilityState = Arc<Mutex<AccessibilityState>>;

pub const WINDOW_ID: AccessibilityId = AccessibilityId(unsafe { NonZeroU128::new_unchecked(1) });

/// Manages the Accessibility integration.
#[derive(Default)]
pub struct AccessibilityState {
    /// Accessibility Nodes
    pub nodes: Vec<(AccessibilityId, Node)>,

    /// Accessibility tree
    pub node_classes: NodeClassSet,

    /// Current focused Accessibility Node.
    pub focus: Option<AccessibilityId>,
}

impl AccessibilityState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Wrap it in a Arc<Mutex<T>>.
    pub fn wrap(self) -> SharedAccessibilityState {
        Arc::new(Mutex::new(self))
    }

    /// Clear the Accessibility Nodes.
    pub fn clear(&mut self) {
        self.nodes.clear();
    }
}

impl AccessibilityProvider for AccessibilityState {
    fn node_classes(&mut self) -> &mut NodeClassSet {
        &mut self.node_classes
    }

    fn nodes(&self) -> std::slice::Iter<(AccessibilityId, Node)> {
        self.nodes.iter()
    }

    fn focus_id(&self) -> Option<AccessibilityId> {
        self.focus
    }

    fn set_focus(&mut self, new_focus_id: Option<AccessibilityId>) {
        self.focus = new_focus_id;
    }

    fn push_node(&mut self, id: AccessibilityId, node: Node) {
        self.nodes.push((id, node))
    }
}

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
                let node_accessibility = &*child.get::<AccessibilitySettings>().unwrap();
                node_accessibility.focus_id
            })
            .collect::<Vec<AccessibilityId>>()
    }
}
