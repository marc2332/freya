use crate::accessibility::AccessibilityProvider;
use accesskit::{Node, NodeClassSet, NodeId as AccessibilityId};
use std::{
    num::NonZeroU128,
    sync::{Arc, Mutex},
};

pub type SharedAccessibilityState = Arc<Mutex<AccessibilityState>>;

pub const ROOT_ID: AccessibilityId = AccessibilityId(unsafe { NonZeroU128::new_unchecked(1) });

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
