use freya_common::CachedParagraph;
use freya_engine::prelude::*;
use freya_native_core::{
    prelude::{
        ElementNode,
        NodeType,
    },
    real_dom::NodeImmutable,
    tags::TagName,
};
use freya_node_state::{
    CursorState,
    FontStyleState,
};
use torin::{
    geometry::Area,
    prelude::{
        AreaModel,
        LayoutNode,
        Length,
        Size2D,
    },
};

use super::utils::ElementUtils;
use crate::{
    dom::DioxusNode,
    prelude::{
        align_highlights_and_cursor_paragraph,
        align_main_align_paragraph,
    },
    skia::create_paragraph,
};

pub struct ParagraphElement;

impl ElementUtils for ParagraphElement {
    fn render(
        self,
        layout_node: &torin::prelude::LayoutNode,
        node_ref: &DioxusNode,
        canvas: &Canvas,
        font_collection: &mut FontCollection,
        _font_manager: &FontMgr,
        default_fonts: &[String],
        scale_factor: f32,
    ) {
        let area = layout_node.visible_area();
        let node_cursor_state = &*node_ref.get::<CursorState>().unwrap();

        let paint = |paragraph: &Paragraph| {
            let x = area.min_x();
            let y = area.min_y() + align_main_align_paragraph(node_ref, &area, paragraph);

            // Draw the highlights if specified
            draw_cursor_highlights(&area, paragraph, canvas, node_ref);

            // Draw a cursor if specified
            draw_cursor(&area, paragraph, canvas, node_ref);

            paragraph.paint(canvas, (x, y));
        };

        if node_cursor_state.position.is_some() {
            let paragraph = create_paragraph(
                node_ref,
                &area.size,
                font_collection,
                true,
                default_fonts,
                scale_factor,
            );
            paint(&paragraph);
        } else {
            let paragraph = &layout_node
                .data
                .as_ref()
                .unwrap()
                .get::<CachedParagraph>()
                .unwrap()
                .0;
            paint(paragraph);
        };
    }

    fn needs_cached_area(&self, node_ref: &DioxusNode) -> bool {
        for text_span in node_ref.children() {
            if let NodeType::Element(ElementNode {
                tag: TagName::Text, ..
            }) = &*text_span.node_type()
            {
                let font_style = text_span.get::<FontStyleState>().unwrap();

                if !font_style.text_shadows.is_empty() {
                    return true;
                }
            }
        }

        false
    }

    fn drawing_area(
        &self,
        layout_node: &LayoutNode,
        node_ref: &DioxusNode,
        scale_factor: f32,
    ) -> Area {
        let mut area = layout_node.visible_area();

        // Iterate over all the text spans inside this paragraph and if any of them
        // has a shadow at all, apply this shadow to the general paragraph.
        // Is this fully correct? Not really.
        // Best thing would be to know if any of these text spans withs shadow actually increase
        // the paragraph area, but I honestly don't know how to properly know the layout of X
        // text span with shadow.
        // Therefore I simply assume that the shadow of any text span is referring to the paragraph.
        // Better to have a big dirty area rather than smaller than what is supposed to be rendered again.

        for text_span in node_ref.children() {
            if let NodeType::Element(ElementNode {
                tag: TagName::Text, ..
            }) = &*text_span.node_type()
            {
                let font_style = text_span.get::<FontStyleState>().unwrap();

                let mut text_shadow_area = area;

                for text_shadow in &font_style.text_shadows {
                    if text_shadow.color != Color::TRANSPARENT {
                        text_shadow_area.move_with_offsets(
                            &Length::new(text_shadow.offset.x),
                            &Length::new(text_shadow.offset.y),
                        );

                        let expanded_size = text_shadow.blur_sigma.ceil() as f32 * scale_factor;

                        text_shadow_area.expand(&Size2D::new(expanded_size, expanded_size))
                    }
                }

                area = area.union(&text_shadow_area);
            }
        }

        area
    }
}

fn draw_cursor_highlights(
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
            let (start, end) = align_highlights_and_cursor_paragraph(
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

            canvas.draw_rect(Rect::new(start.x, start.y, end.x, end.y), &paint);
        }
    }

    Some(())
}

fn draw_cursor(
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

    let (start, end) =
        align_highlights_and_cursor_paragraph(node_ref, area, paragraph, cursor_rect, Some(1.0));

    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_style(PaintStyle::Fill);
    paint.set_color(cursor_color);

    canvas.draw_rect(Rect::new(start.x, start.y, end.x, end.y), &paint);

    Some(())
}
