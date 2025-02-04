use std::ops::{
    Deref,
    DerefMut,
};

use freya_native_core::NodeId;
use rustc_hash::FxHashSet;

#[derive(Clone, Default, Debug)]
pub struct CompositorDirtyNodes(FxHashSet<NodeId>);

impl Deref for CompositorDirtyNodes {
    type Target = FxHashSet<NodeId>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CompositorDirtyNodes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl CompositorDirtyNodes {
    /// Mark a certain node as invalidated.
    pub fn invalidate(&mut self, node_id: NodeId) {
        self.0.insert(node_id);
    }
}
