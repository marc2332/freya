use freya_native_core::NodeId;
use rustc_hash::FxHashMap;
use std::sync::{Arc, Mutex, MutexGuard};
use uuid::Uuid;

#[derive(Default, Clone)]
pub struct ParagraphElements {
    pub paragraphs: Arc<Mutex<FxHashMap<Uuid, Vec<NodeId>>>>,
}

impl ParagraphElements {
    pub fn insert_paragraph(&self, node_id: NodeId, text_id: Uuid) {
        let mut paragraphs = self.paragraphs.lock().unwrap();
        let text_group = paragraphs.entry(text_id).or_default();

        text_group.push(node_id);
    }

    pub fn paragraphs(&self) -> MutexGuard<FxHashMap<Uuid, Vec<NodeId>>> {
        self.paragraphs.lock().unwrap()
    }

    pub fn remove_paragraph(&self, node_id: NodeId, text_id: &Uuid) {
        let mut paragraphs = self.paragraphs.lock().unwrap();
        let text_group = paragraphs.get_mut(text_id);

        if let Some(text_group) = text_group {
            text_group.retain(|id| *id != node_id);

            if text_group.is_empty() {
                paragraphs.remove(text_id);
            }
        }
    }

    pub fn len_paragraphs(&self) -> usize {
        self.paragraphs.lock().unwrap().len()
    }
}
