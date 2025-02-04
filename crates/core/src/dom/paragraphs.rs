use std::{
    self,
    ops::{
        Deref,
        DerefMut,
    },
};

use freya_native_core::NodeId;
use rustc_hash::{
    FxHashMap,
    FxHashSet,
};
use uuid::Uuid;

#[derive(Default, Clone)]
pub struct ParagraphElements(FxHashMap<Uuid, FxHashSet<NodeId>>);

impl ParagraphElements {
    pub fn insert_paragraph(&mut self, node_id: NodeId, text_id: Uuid) {
        let text_group = self.0.entry(text_id).or_default();

        text_group.insert(node_id);
    }

    pub fn remove_paragraph(&mut self, node_id: NodeId, text_id: &Uuid) {
        let text_group = self.0.get_mut(text_id);

        if let Some(text_group) = text_group {
            text_group.retain(|id| *id != node_id);

            if text_group.is_empty() {
                self.0.remove(text_id);
            }
        }
    }
}

impl Deref for ParagraphElements {
    type Target = FxHashMap<Uuid, FxHashSet<NodeId>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ParagraphElements {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
