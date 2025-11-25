use std::{
    hash::{
        Hash,
        Hasher,
    },
    rc::Rc,
};

use freya_engine::prelude::SkParagraph;
use rustc_hash::{
    FxHashMap,
    FxHasher,
};

use crate::{
    data::TextStyleState,
    node_id::NodeId,
    prelude::Span,
};

pub struct CachedParagraph<'a> {
    pub text_style_state: &'a TextStyleState,
    pub spans: &'a [Span<'a>],
    pub max_lines: Option<usize>,
    pub line_height: Option<f32>,
    pub width: f32,
}

impl Hash for CachedParagraph<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.text_style_state.hash(state);
        self.spans.hash(state);
        self.max_lines.hash(state);
        if let Some(v) = self.line_height {
            v.to_bits().hash(state)
        }
        self.width.to_bits().hash(state);
    }
}

#[derive(Default)]
pub struct TextCache {
    map: FxHashMap<u64, (i32, Rc<SkParagraph>)>,
    users: FxHashMap<NodeId, u64>,
}

impl TextCache {
    pub fn utilize(
        &mut self,
        node_id: NodeId,
        paragraph: &CachedParagraph,
    ) -> Option<Rc<SkParagraph>> {
        let mut hasher = FxHasher::default();
        paragraph.hash(&mut hasher);
        let hash = hasher.finish();
        let mut value = self.map.get_mut(&hash);

        if let Some(value) = &mut value {
            value.0 += 1;
        }

        value.map(|v| v.1.clone()).inspect(|_| {
            // Stop utilizing the old paragraph if it had one and its different
            let Some(old_hash) = self.users.insert(node_id, hash) else {
                return;
            };

            if hash == old_hash {
                return;
            }

            let Some(entry) = self.map.get_mut(&old_hash) else {
                return;
            };

            entry.0 -= 1;

            if entry.0 == 0 {
                self.map.remove(&hash);
            }
        })
    }

    pub fn insert(
        &mut self,
        node_id: NodeId,
        paragraph: &CachedParagraph,
        sk_paragraph: SkParagraph,
    ) -> Rc<SkParagraph> {
        let mut hasher = FxHasher::default();
        paragraph.hash(&mut hasher);
        let hash = hasher.finish();
        let sk_paragraph = Rc::new(sk_paragraph);

        self.map.insert(hash, (1, sk_paragraph.clone()));
        self.users.insert(node_id, hash);

        sk_paragraph
    }

    pub fn remove(&mut self, node_id: &NodeId) {
        let Some(hash) = self.users.remove(node_id) else {
            return;
        };
        let Some(entry) = self.map.get_mut(&hash) else {
            return;
        };

        entry.0 -= 1;

        if entry.0 == 0 {
            self.map.remove(&hash);
        }
    }

    pub fn reset(&mut self) {
        self.map.clear();
        self.users.clear();
    }

    pub fn print_metrics(&self) {
        println!("Cached Paragraphs {}", self.map.len());
        println!("Paragraphs Users {}", self.users.len());
    }
}
