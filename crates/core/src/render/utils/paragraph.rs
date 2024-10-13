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
};
use torin::prelude::{
    Alignment,
    Area,
    Size2D,
};

use crate::dom::DioxusNode;

/// Compose a new SkParagraph
pub fn create_paragraph(
    node: &DioxusNode,
    area_size: &Size2D,
    font_collection: &FontCollection,
    is_rendering: bool,
    default_font_family: &[String],
    scale_factor: f32,
) -> Paragraph {
    let font_style = &*node.get::<FontStyleState>().unwrap();

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
