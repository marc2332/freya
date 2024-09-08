use freya_native_core::NodeId;
use rustc_hash::{
    FxHashMap,
    FxHashSet,
};

#[derive(Default)]
pub struct AccessibilityDirtyNodes {
    pub added_or_updated: FxHashSet<NodeId>,
    pub removed: FxHashMap<NodeId, NodeId>,
}

impl AccessibilityDirtyNodes {
    pub fn add_or_update(&mut self, node_id: NodeId) {
        self.added_or_updated.insert(node_id);
    }

    pub fn remove(&mut self, node_id: NodeId, ancestor_node_id: NodeId) {
        self.removed.insert(node_id, ancestor_node_id);
    }

    pub fn clear(&mut self) {
        self.added_or_updated.clear();
        self.removed.clear();
    }
}
