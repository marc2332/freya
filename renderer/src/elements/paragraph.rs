use freya_dom::{DioxusNode, FreyaDOM};
use freya_layout::{get_inner_texts, RenderData};
use skia_safe::{
    textlayout::{
        FontCollection, Paragraph, ParagraphBuilder, ParagraphStyle, RectHeightStyle,
        RectWidthStyle, TextHeightBehavior, TextStyle,
    },
    Canvas, Paint, PaintStyle, Rect,
};

/// Render a `paragraph` element
pub fn render_paragraph(
    render_node: &RenderData,
    dioxus_node: &DioxusNode,
    font_collection: &mut FontCollection,
    dom: &FreyaDOM,
    canvas: &mut Canvas,
) {
    let align = dioxus_node.state.font_style.align;
    let max_lines = dioxus_node.state.font_style.max_lines;

    let texts = get_inner_texts(dom, &render_node.node_id);

    let (x, y) = render_node.node_area.get_origin_points();

    let mut paragraph_style = ParagraphStyle::default();
    paragraph_style.set_max_lines(max_lines);
    paragraph_style.set_text_align(align);
    paragraph_style.set_replace_tab_characters(true);
    paragraph_style.set_text_height_behavior(TextHeightBehavior::DisableAll);

    let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, font_collection.clone());

    for (style, text) in &texts {
        paragraph_builder.push_style(
            TextStyle::new()
                .set_font_style(style.font_style)
                .set_height_override(true)
                .set_height(style.line_height)
                .set_color(style.color)
                .set_font_size(style.font_size)
                .set_font_families(&style.font_family),
        );
        paragraph_builder.add_text(text.clone());
    }

    if dioxus_node.state.cursor_settings.position.is_some() {
        // This is very tricky, but it works! It allows freya to render the cursor at the end of a line.
        paragraph_builder.add_text(" ");
    }

    let mut paragraph = paragraph_builder.build();

    paragraph.layout(render_node.node_area.width);

    // Draw the highlights if specified
    draw_cursor_highlights(render_node, &paragraph, canvas, dioxus_node);

    // Draw a cursor if specified
    draw_cursor(render_node, &paragraph, canvas, dioxus_node);

    paragraph.paint(canvas, (x, y));
}

fn draw_cursor_highlights(
    render_node: &RenderData,
    paragraph: &Paragraph,
    canvas: &mut Canvas,
    dioxus_node: &DioxusNode,
) -> Option<()> {
    let highlights = dioxus_node.state.cursor_settings.highlights.as_ref()?;
    let highlight_color = dioxus_node.state.cursor_settings.highlight_color;

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
            let x = render_node.node_area.x + cursor_rect.rect.left;
            let y = render_node.node_area.y + cursor_rect.rect.top;

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
    render_node: &RenderData,
    paragraph: &Paragraph,
    canvas: &mut Canvas,
    dioxus_node: &DioxusNode,
) -> Option<()> {
    let cursor = dioxus_node.state.cursor_settings.position?;
    let cursor_color = dioxus_node.state.cursor_settings.color;
    let cursor_position = cursor as usize;

    let cursor_rects = paragraph.get_rects_for_range(
        cursor_position..cursor_position + 1,
        RectHeightStyle::Tight,
        RectWidthStyle::Tight,
    );
    let cursor_rect = cursor_rects.first()?;

    let x = render_node.node_area.x + cursor_rect.rect.left;
    let y = render_node.node_area.y + cursor_rect.rect.top;

    let x2 = x + 1.0;
    let y2 = y + (cursor_rect.rect.bottom - cursor_rect.rect.top);

    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_style(PaintStyle::Fill);
    paint.set_color(cursor_color);

    canvas.draw_rect(Rect::new(x, y, x2, y2), &paint);

    Some(())
}
