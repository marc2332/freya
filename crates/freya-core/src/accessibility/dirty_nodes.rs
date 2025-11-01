use rustc_hash::{
    FxHashMap,
    FxHashSet,
};

use crate::{
    node_id::NodeId,
    prelude::AccessibilityFocusStrategy,
};

#[derive(Default)]
pub struct AccessibilityDirtyNodes {
    pub requested_focus: Option<AccessibilityFocusStrategy>,
    pub added_or_updated: FxHashSet<NodeId>,
    pub removed: FxHashMap<NodeId, NodeId>,
}

impl AccessibilityDirtyNodes {
    pub fn request_focus(&mut self, strategy: AccessibilityFocusStrategy) {
        self.requested_focus = Some(strategy);
    }

    pub fn add_or_update(&mut self, node_id: NodeId) {
        self.added_or_updated.insert(node_id);
    }

    pub fn remove(&mut self, node_id: NodeId, parent_id: NodeId) {
        self.removed.insert(node_id, parent_id);
    }

    pub fn clear(&mut self) {
        self.requested_focus.take();
        self.added_or_updated.clear();
        self.removed.clear();
    }
}
