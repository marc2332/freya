use freya_dom::FreyaDOM;
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
    dom: &FreyaDOM,
    canvas: &mut Canvas,
    font_collection: &mut FontCollection,
    node: &RenderData,
) {
    let dioxus_node = node.get_node(dom);
    let align = dioxus_node.state.font_style.align;
    let max_lines = dioxus_node.state.font_style.max_lines;

    let texts = get_inner_texts(dom, &node.node_id);

    let (x, y) = node.node_area.get_origin_points();

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

    paragraph.layout(node.node_area.width);

    paragraph.paint(canvas, (x, y));

    // Draw a cursor if specified
    draw_cursor(node, paragraph, canvas, dom);
}

fn draw_cursor(
    node: &RenderData,
    paragraph: Paragraph,
    canvas: &mut Canvas,
    rdom: &FreyaDOM,
) -> Option<()> {
    let dioxus_node = node.get_node(rdom);
    let cursor = dioxus_node.state.cursor_settings.position?;
    let cursor_color = dioxus_node.state.cursor_settings.color;
    let cursor_position = cursor as usize;

    let cursor_rects = paragraph.get_rects_for_range(
        cursor_position..cursor_position + 1,
        RectHeightStyle::Tight,
        RectWidthStyle::Tight,
    );
    let cursor_rect = cursor_rects.first()?;

    let x = node.node_area.x + cursor_rect.rect.left;
    let y = node.node_area.y + cursor_rect.rect.top;

    let x2 = x + 1.0;
    let y2 = y + (cursor_rect.rect.bottom - cursor_rect.rect.top);

    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_style(PaintStyle::Fill);
    paint.set_color(cursor_color);

    canvas.draw_rect(Rect::new(x, y, x2, y2), &paint);

    Some(())
}
