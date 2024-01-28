use crate::accessibility::*;
use accesskit::{Node, NodeClassSet, NodeId as AccessibilityId};
use std::sync::{Arc, Mutex};

pub type SharedAccessibilityState = Arc<Mutex<AccessibilityState>>;

pub const ACCESSIBILITY_ROOT_ID: AccessibilityId = AccessibilityId(0);

/// Manages the Accessibility integration.
pub struct AccessibilityState {
    /// Accessibility Nodes
    pub nodes: Vec<(AccessibilityId, Node)>,

    /// Accessibility tree
    pub node_classes: NodeClassSet,

    /// Current focused Accessibility Node.
    pub focused_id: AccessibilityId,
}

impl AccessibilityState {
    pub fn new(focused_id: AccessibilityId) -> Self {
        Self {
            focused_id,
            node_classes: NodeClassSet::default(),
            nodes: Vec::default(),
        }
    }

    /// Wrap it in a `Arc<Mutex<T>>`.
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

    fn focus_id(&self) -> AccessibilityId {
        self.focused_id
    }

    fn set_focus(&mut self, new_focus_id: AccessibilityId) {
        self.focused_id = new_focus_id;
    }

    fn push_node(&mut self, id: AccessibilityId, node: Node) {
        self.nodes.push((id, node))
    }
}
