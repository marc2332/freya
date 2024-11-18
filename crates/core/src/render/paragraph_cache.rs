use std::borrow::Cow;

use freya_common::CachedParagraph;
use freya_engine::prelude::*;
use freya_node_state::{
    FontStyleState,
    TextOverflow,
};
use rustc_hash::FxBuildHasher;

pub struct ParagraphCache {
    cache: indexmap::IndexMap<u64, CachedParagraph, FxBuildHasher>,
    max_size: usize,
}

impl ParagraphCache {
    pub const MAX_SIZE: usize = 128;

    pub fn new(max_size: usize) -> Self {
        Self {
            cache: indexmap::IndexMap::with_hasher(FxBuildHasher),
            max_size,
        }
    }

    pub fn insert(&mut self, key: u64, paragraph: CachedParagraph) {
        self.cache.insert(key, paragraph);

        if self.cache.len() > self.max_size {
            let first = *self.cache.first().unwrap().0;
            self.cache.shift_remove(&first);

            #[cfg(debug_assertions)]
            tracing::info!(
                "Reached max size of paragraph cache ({}), removing first element",
                self.max_size
            );
        }
    }

    pub fn get(&self, key: &u64) -> Option<&CachedParagraph> {
        self.cache.get(key)
    }
}

#[derive(Hash)]
pub struct ParagraphCacheKey<'a> {
    pub color: (u8, u8, u8),
    pub font_family: &'a [String],
    pub font_size: u32,
    pub font_slant: Slant,
    pub font_weight: i32,
    pub font_width: i32,
    pub line_height: Option<u32>,
    pub word_spacing: u32,
    pub letter_spacing: u32,
    pub text_align: TextAlign,
    pub max_lines: Option<usize>,
    pub text_overflow: TextOverflow,
    pub text_height: TextHeightBehavior,
    pub text: Option<Cow<'a, str>>,
}

impl<'a> ParagraphCacheKey<'a> {
    pub fn new(
        font_style: &FontStyleState,
        font_family: &'a [String],
        text: Option<Cow<'a, str>>,
    ) -> Self {
        Self {
            color: (
                font_style.color.r(),
                font_style.color.g(),
                font_style.color.b(),
            ),
            font_family,
            font_size: font_style.font_size.to_bits(),
            font_slant: font_style.font_slant,
            font_weight: *font_style.font_weight,
            font_width: *font_style.font_width,
            line_height: font_style.line_height.map(|n| n.to_bits()),
            word_spacing: font_style.word_spacing.to_bits(),
            letter_spacing: font_style.letter_spacing.to_bits(),
            text_align: font_style.text_align,
            max_lines: font_style.max_lines,
            text_overflow: font_style.text_overflow.clone(),
            text_height: font_style.text_height,
            text,
        }
    }
}
