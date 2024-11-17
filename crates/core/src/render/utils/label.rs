use std::rc::Rc;

use freya_common::CachedParagraph;
use freya_engine::prelude::*;
use freya_native_core::{
    prelude::NodeType,
    real_dom::NodeImmutable,
};
use freya_node_state::FontStyleState;
use rustc_hash::FxBuildHasher;
use torin::prelude::Size2D;

use super::ParagraphCacheKey;
use crate::{
    dom::*,
    render::ParagraphCache,
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

    let mut paragraph_cache_key: (u32, ParagraphCacheKey) = (
        area_size.width.to_bits(),
        ParagraphCacheKey {
            color: (
                font_style.color.r(),
                font_style.color.g(),
                font_style.color.b(),
            ),
            font_family: default_font_family,
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
            text: Some("".to_string()),
        },
    );

    for child in node.children() {
        if let NodeType::Text(text) = &*child.node_type() {
            paragraph_cache_key.1.text.as_mut().unwrap().push_str(text);
        }
    }

    use std::hash::BuildHasher;

    let hasher = FxBuildHasher;
    let paragraph_cache_key_hash = hasher.hash_one(paragraph_cache_key);

    let paragraph = paragraph_cache.get(&paragraph_cache_key_hash).cloned();
    if let Some(paragraph) = paragraph {
        return paragraph;
    }

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

    let mut paragraph = paragraph_builder.build();
    paragraph.layout(
        if font_style.max_lines == Some(1) && font_style.text_align == TextAlign::default() {
            f32::MAX
        } else {
            area_size.width + 1.0
        },
    );

    let paragraph = CachedParagraph(Rc::new(paragraph));

    paragraph_cache.insert(paragraph_cache_key_hash, paragraph.clone());

    if paragraph_cache.len() > 128 {
        let first = *paragraph_cache.first().unwrap().0;
        paragraph_cache.shift_remove(&first);
    }

    paragraph
}
