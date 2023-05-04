use dioxus_native_core::real_dom::NodeImmutable;
use freya_dom::{get_inner_texts, DioxusNode};
use freya_node_state::{CursorSettings, FontStyle};
use skia_safe::{
    textlayout::{
        FontCollection, Paragraph, ParagraphBuilder, ParagraphStyle, RectHeightStyle,
        RectWidthStyle, TextHeightBehavior, TextStyle,
    },
    Canvas, Paint, PaintStyle, Rect,
};
use torin::geometry::Area;

/// Render a `paragraph` element
pub fn render_paragraph(
    area: &Area,
    dioxus_node: &DioxusNode,
    canvas: &mut Canvas,
    font_collection: &mut FontCollection,
) {
    let node_font_style = &*dioxus_node.get::<FontStyle>().unwrap();
    let node_cursor_settings = &*dioxus_node.get::<CursorSettings>().unwrap();

    let texts = get_inner_texts(dioxus_node);

    let (x, y) = area.origin.to_tuple();

    let mut paragraph_style = ParagraphStyle::default();
    paragraph_style.set_max_lines(node_font_style.max_lines);
    paragraph_style.set_text_align(node_font_style.align);
    paragraph_style.set_replace_tab_characters(true);
    paragraph_style.set_text_height_behavior(TextHeightBehavior::DisableAll);

    let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, font_collection.clone());

    for (font_style, text) in &texts {
        paragraph_builder.push_style(
            TextStyle::new()
                .set_font_style(font_style.font_style)
                .set_height_override(true)
                .set_height(font_style.line_height)
                .set_color(font_style.color)
                .set_font_size(font_style.font_size)
                .set_font_families(&font_style.font_family),
        );
        paragraph_builder.add_text(text);
    }

    if node_cursor_settings.position.is_some() {
        // This is very tricky, but it works! It allows freya to render the cursor at the end of a line.
        paragraph_builder.add_text(" ");
    }

    let mut paragraph = paragraph_builder.build();

    paragraph.layout(area.width() + 1.0); // `+ 1.0`: Skia won't render the last word properly otherwise

    // Draw the highlights if specified
    draw_cursor_highlights(area, &paragraph, canvas, dioxus_node);

    // Draw a cursor if specified
    draw_cursor(area, &paragraph, canvas, dioxus_node);

    paragraph.paint(canvas, (x, y));
}

fn draw_cursor_highlights(
    area: &Area,
    paragraph: &Paragraph,
    canvas: &mut Canvas,
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
    canvas: &mut Canvas,
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
