use std::sync::Arc;

use freya_common::CachedParagraph;
use freya_engine::prelude::*;
use freya_native_core::{
    real_dom::NodeImmutable,
    SendAnyMap,
};
use freya_node_state::CursorState;
use torin::geometry::Area;

use crate::{
    dom::DioxusNode,
    prelude::{
        align_highlights_and_cursor_paragraph,
        align_main_align_paragraph,
    },
    skia::create_paragraph,
};

/// Render a `paragraph` element
pub fn render_paragraph(
    area: &Area,
    data: &Option<Arc<SendAnyMap>>,
    dioxus_node: &DioxusNode,
    canvas: &Canvas,
    font_collection: &mut FontCollection,
    default_fonts: &[String],
    scale_factor: f32,
) {
    let node_cursor_state = &*dioxus_node.get::<CursorState>().unwrap();

    let paint = |paragraph: &Paragraph| {
        let x = area.min_x();
        let y = area.min_y() + align_main_align_paragraph(dioxus_node, area, paragraph);

        // Draw the highlights if specified
        draw_cursor_highlights(area, paragraph, canvas, dioxus_node);

        // Draw a cursor if specified
        draw_cursor(area, paragraph, canvas, dioxus_node);

        paragraph.paint(canvas, (x, y));
    };

    if node_cursor_state.position.is_some() {
        let paragraph = create_paragraph(
            dioxus_node,
            &area.size,
            font_collection,
            true,
            default_fonts,
            scale_factor,
        );
        paint(&paragraph);
    } else {
        let paragraph = &data.as_ref().unwrap().get::<CachedParagraph>().unwrap().0;
        paint(paragraph);
    };
}

fn draw_cursor_highlights(
    area: &Area,
    paragraph: &Paragraph,
    canvas: &Canvas,
    dioxus_node: &DioxusNode,
) -> Option<()> {
    let node_cursor_state = &*dioxus_node.get::<CursorState>().unwrap();

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
                dioxus_node,
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
    dioxus_node: &DioxusNode,
) -> Option<()> {
    let node_cursor_state = &*dioxus_node.get::<CursorState>().unwrap();

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
        align_highlights_and_cursor_paragraph(dioxus_node, area, paragraph, cursor_rect, Some(1.0));

    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_style(PaintStyle::Fill);
    paint.set_color(cursor_color);

    canvas.draw_rect(Rect::new(start.x, start.y, end.x, end.y), &paint);

    Some(())
}
