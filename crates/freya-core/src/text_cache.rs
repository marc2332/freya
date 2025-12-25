use std::hash::Hash;

use freya_engine::prelude::SkParagraph;

use crate::{
    data::TextStyleState,
    lru_cache::LRUCache,
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

pub type TextCache = LRUCache<SkParagraph, NodeId>;
