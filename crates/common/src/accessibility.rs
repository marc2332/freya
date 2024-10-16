use std::sync::atomic::{
    AtomicU64,
    Ordering,
};

use freya_native_core::NodeId;
use rustc_hash::{
    FxHashMap,
    FxHashSet,
};

#[derive(Default)]
pub struct AccessibilityDirtyNodes {
    pub requested_focus: Option<NodeId>,
    pub added_or_updated: FxHashSet<NodeId>,
    pub removed: FxHashMap<NodeId, NodeId>,
}

impl AccessibilityDirtyNodes {
    pub fn request_focus(&mut self, node_id: NodeId) {
        self.requested_focus = Some(node_id);
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

pub struct AccessibilityGenerator {
    counter: AtomicU64,
}

impl Default for AccessibilityGenerator {
    fn default() -> Self {
        Self {
            counter: AtomicU64::new(1), // Must start at 1 because 0 is reserved for the Root
        }
    }
}

impl AccessibilityGenerator {
    pub fn new_id(&self) -> u64 {
        self.counter.fetch_add(1, Ordering::Relaxed)
    }
}
