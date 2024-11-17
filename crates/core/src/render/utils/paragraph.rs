use std::{
    hash::Hash,
    rc::Rc,
    vec,
};

use freya_common::CachedParagraph;
use freya_engine::prelude::*;
use freya_native_core::{
    node::ElementNode,
    prelude::NodeType,
    real_dom::NodeImmutable,
    tags::TagName,
};
use freya_node_state::{
    CursorState,
    FontStyleState,
    HighlightMode,
    LayoutState,
    TextOverflow,
};
use rustc_hash::FxBuildHasher;
use torin::prelude::{
    Alignment,
    Area,
    Size2D,
};

use crate::{
    dom::DioxusNode,
    render::ParagraphCache,
};
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
    pub text: Option<String>,
}

/// Compose a new SkParagraph
pub fn create_paragraph(
    node: &DioxusNode,
    area_size: &Size2D,
    font_collection: &FontCollection,
    is_rendering: bool,
    default_font_family: &[String],
    scale_factor: f32,
    paragraph_cache: &mut ParagraphCache,
) -> CachedParagraph {
    let font_style = &*node.get::<FontStyleState>().unwrap();

    let mut paragraph_cache_key: (u32, ParagraphCacheKey, Vec<ParagraphCacheKey>) = (
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
            text: None,
        },
        vec![],
    );

    for text_span in node.children() {
        if let NodeType::Element(ElementNode {
            tag: TagName::Text, ..
        }) = &*text_span.node_type()
        {
            let text_nodes = text_span.children();
            let text_node = *text_nodes.first().unwrap();
            let text_node_type = &*text_node.node_type();
            let text_font_style = text_span.get::<FontStyleState>().unwrap();
            let mut key = ParagraphCacheKey {
                color: (
                    text_font_style.color.r(),
                    font_style.color.g(),
                    font_style.color.b(),
                ),
                font_family: default_font_family,
                font_size: text_font_style.font_size.to_bits(),
                font_slant: text_font_style.font_slant,
                font_weight: *text_font_style.font_weight,
                font_width: *text_font_style.font_width,
                line_height: text_font_style.line_height.map(|n| n.to_bits()),
                word_spacing: text_font_style.word_spacing.to_bits(),
                letter_spacing: text_font_style.letter_spacing.to_bits(),
                text_align: text_font_style.text_align,
                max_lines: text_font_style.max_lines,
                text_overflow: text_font_style.text_overflow.clone(),
                text_height: text_font_style.text_height,
                text: None,
            };

            if let NodeType::Text(text) = text_node_type {
                key.text = Some(text.clone());
            }

            paragraph_cache_key.2.push(key);
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

    let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, font_collection);

    let text_style =
        font_style.text_style(default_font_family, scale_factor, font_style.text_height);
    paragraph_builder.push_style(&text_style);

    for text_span in node.children() {
        if let NodeType::Element(ElementNode {
            tag: TagName::Text, ..
        }) = &*text_span.node_type()
        {
            let text_nodes = text_span.children();
            let text_node = *text_nodes.first().unwrap();
            let text_node_type = &*text_node.node_type();
            let text_font_style = text_span.get::<FontStyleState>().unwrap();
            let text_style = text_font_style.text_style(
                default_font_family,
                scale_factor,
                font_style.text_height,
            );
            paragraph_builder.push_style(&text_style);

            if let NodeType::Text(text) = text_node_type {
                paragraph_builder.add_text(text);
            }
        }
    }

    if is_rendering {
        // This is very tricky, but it works! It allows freya to render the cursor at the end of a line.
        paragraph_builder.add_text(" ");
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

pub fn draw_cursor_highlights(
    area: &Area,
    paragraph: &Paragraph,
    canvas: &Canvas,
    node_ref: &DioxusNode,
) -> Option<()> {
    let node_cursor_state = &*node_ref.get::<CursorState>().unwrap();

    let highlights = node_cursor_state.highlights.as_ref()?;
    let highlight_color = node_cursor_state.highlight_color;

    for (from, to) in highlights.iter() {
        let (from, to) = {
            if from < to {
                (from, to)
            } else {
                (to, from)
            }
        };
        let cursor_rects = paragraph.get_rects_for_range(
            *from..*to,
            RectHeightStyle::Tight,
            RectWidthStyle::Tight,
        );
        for cursor_rect in cursor_rects {
            let rect = align_highlights_and_cursor_paragraph(
                node_ref,
                area,
                paragraph,
                &cursor_rect,
                None,
            );

            let mut paint = Paint::default();
            paint.set_anti_alias(true);
            paint.set_style(PaintStyle::Fill);
            paint.set_color(highlight_color);

            canvas.draw_rect(rect, &paint);
        }
    }

    Some(())
}

pub fn draw_cursor(
    area: &Area,
    paragraph: &Paragraph,
    canvas: &Canvas,
    node_ref: &DioxusNode,
) -> Option<()> {
    let node_cursor_state = &*node_ref.get::<CursorState>().unwrap();

    let cursor = node_cursor_state.position?;
    let cursor_color = node_cursor_state.color;
    let cursor_position = cursor as usize;

    let cursor_rects = paragraph.get_rects_for_range(
        cursor_position..cursor_position + 1,
        RectHeightStyle::Tight,
        RectWidthStyle::Tight,
    );
    let cursor_rect = cursor_rects.first()?;

    let rect =
        align_highlights_and_cursor_paragraph(node_ref, area, paragraph, cursor_rect, Some(1.0));

    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_style(PaintStyle::Fill);
    paint.set_color(cursor_color);

    canvas.draw_rect(rect, &paint);

    Some(())
}

/// Align the Y axis of the highlights and cursor of a paragraph
pub fn align_highlights_and_cursor_paragraph(
    node: &DioxusNode,
    area: &Area,
    paragraph: &Paragraph,
    cursor_rect: &TextBox,
    width: Option<f32>,
) -> Rect {
    let cursor_state = node.get::<CursorState>().unwrap();

    let left = area.min_x() + cursor_rect.rect.left;
    let right = left + width.unwrap_or(cursor_rect.rect.right - cursor_rect.rect.left);

    match cursor_state.highlight_mode {
        HighlightMode::Fit => {
            let top = (area.min_y()
                + align_main_align_paragraph(node, area, paragraph)
                + cursor_rect.rect.top)
                .clamp(area.min_y(), area.max_y());
            let bottom = (top + (cursor_rect.rect.bottom - cursor_rect.rect.top))
                .clamp(area.min_y(), area.max_y());

            Rect::new(left, top, right, bottom)
        }
        HighlightMode::Expanded => {
            let top = area.min_y();
            let bottom = area.max_y();

            Rect::new(left, top, right, bottom)
        }
    }
}

/// Align the main alignment of a paragraph
pub fn align_main_align_paragraph(node: &DioxusNode, area: &Area, paragraph: &Paragraph) -> f32 {
    let layout = node.get::<LayoutState>().unwrap();

    match layout.main_alignment {
        Alignment::Start => 0.,
        Alignment::Center => (area.height() / 2.0) - (paragraph.height() / 2.0),
        Alignment::End => area.height() - paragraph.height(),
        Alignment::SpaceBetween => 0.,
        Alignment::SpaceEvenly => 0.,
        Alignment::SpaceAround => 0.,
    }
}
