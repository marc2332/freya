use std::sync::Arc;

use freya_common::CachedParagraph;
use freya_core::{dom::DioxusNode, layout::create_paragraph};
use freya_native_core::{real_dom::NodeImmutable, SendAnyMap};

use freya_engine::prelude::*;
use freya_node_state::CursorSettings;
use torin::geometry::Area;

/// Render a `paragraph` element
pub fn render_paragraph(
    area: &Area,
    data: &Option<Arc<SendAnyMap>>,
    dioxus_node: &DioxusNode,
    canvas: &Canvas,
    font_collection: &mut FontCollection,
    default_fonts: &[String],
) {
    let (x, y) = area.origin.to_tuple();
    let node_cursor_settings = &*dioxus_node.get::<CursorSettings>().unwrap();

    let paint = |paragraph: &Paragraph| {
        // Draw the highlights if specified
        draw_cursor_highlights(area, paragraph, canvas, dioxus_node);

        // Draw a cursor if specified
        draw_cursor(area, paragraph, canvas, dioxus_node);

        paragraph.paint(canvas, (x, y));
    };

    if node_cursor_settings.position.is_some() {
        let paragraph = create_paragraph(
            dioxus_node,
            &area.size,
            font_collection,
            true,
            default_fonts,
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
    let node_cursor_settings = &*dioxus_node.get::<CursorSettings>().unwrap();

    let highlights = node_cursor_settings.highlights.as_ref()?;
    let highlight_color = node_cursor_settings.highlight_color;

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
            let x = area.min_x() + cursor_rect.rect.left;
            let y = area.min_y() + cursor_rect.rect.top;

            let x2 = x + (cursor_rect.rect.right - cursor_rect.rect.left);
            let y2 = y + (cursor_rect.rect.bottom - cursor_rect.rect.top);

            let mut paint = Paint::default();
            paint.set_anti_alias(true);
            paint.set_style(PaintStyle::Fill);
            paint.set_color(highlight_color);

            canvas.draw_rect(Rect::new(x, y, x2, y2), &paint);
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
    let node_cursor_settings = &*dioxus_node.get::<CursorSettings>().unwrap();

    let cursor = node_cursor_settings.position?;
    let cursor_color = node_cursor_settings.color;
    let cursor_position = cursor as usize;

    let cursor_rects = paragraph.get_rects_for_range(
        cursor_position..cursor_position + 1,
        RectHeightStyle::Tight,
        RectWidthStyle::Tight,
    );
    let cursor_rect = cursor_rects.first()?;

    let x = area.min_x() + cursor_rect.rect.left;
    let y = area.min_y() + cursor_rect.rect.top;

    let x2 = x + 1.0;
    let y2 = y + (cursor_rect.rect.bottom - cursor_rect.rect.top);

    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_style(PaintStyle::Fill);
    paint.set_color(cursor_color);

    canvas.draw_rect(Rect::new(x, y, x2, y2), &paint);

    Some(())
}
