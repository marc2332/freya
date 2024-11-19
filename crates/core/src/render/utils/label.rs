use std::{
    cell::RefCell,
    rc::Rc,
    time::Instant,
};

use freya_common::CachedParagraph;
use freya_engine::prelude::*;
use freya_native_core::{
    prelude::NodeType,
    real_dom::NodeImmutable,
};
use freya_node_state::FontStyleState;
use rustc_hash::FxBuildHasher;
use torin::prelude::Size2D;

use crate::{
    dom::*,
    render::{
        ParagraphCache,
        ParagraphCacheKey,
        ParagraphCacheText,
    },
};

pub fn create_label(
    node: &DioxusNode,
    area_size: &Size2D,
    font_collection: &FontCollection,
    default_font_family: &[String],
    scale_factor: f32,
    paragraph_cache: &mut ParagraphCache,
) -> CachedParagraph {
    let font_style = &*node.get::<FontStyleState>().unwrap();

    let mut label_text = Vec::new();

    for child in node.children() {
        if let NodeType::Text(text) = &*child.node_type() {
            let span_text_hash = FxBuildHasher.hash_one(text);
            label_text.push(span_text_hash);
        }
    }

    let paragraph_cache_key = ParagraphCacheKey::new(
        font_style,
        default_font_family,
        Some(ParagraphCacheText::Hashes(label_text)),
    );

    use std::hash::BuildHasher;

    let hasher = FxBuildHasher;
    let paragraph_cache_key_hash = hasher.hash_one(paragraph_cache_key);

    let paragraph = paragraph_cache.get(&paragraph_cache_key_hash).cloned();

    let paragraph = paragraph.unwrap_or_else(|| {
        let mut paragraph_style = ParagraphStyle::default();
        paragraph_style.set_text_align(font_style.text_align);
        paragraph_style.set_max_lines(font_style.max_lines);
        paragraph_style.set_replace_tab_characters(true);
        paragraph_style.set_text_height_behavior(font_style.text_height);

        if let Some(ellipsis) = font_style.text_overflow.get_ellipsis() {
            paragraph_style.set_ellipsis(ellipsis);
        }

        let text_style =
            font_style.text_style(default_font_family, scale_factor, font_style.text_height);
        paragraph_style.set_text_style(&text_style);

        let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, font_collection);

        for child in node.children() {
            if let NodeType::Text(text) = &*child.node_type() {
                paragraph_builder.add_text(text);
            }
        }

        CachedParagraph(Rc::new(RefCell::new(paragraph_builder.build())))
    });
    paragraph.0.borrow_mut().layout(
        if font_style.max_lines == Some(1) && font_style.text_align == TextAlign::default() {
            f32::MAX
        } else {
            area_size.width + 1.0
        },
    );

    paragraph
}
