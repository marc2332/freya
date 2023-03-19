use dioxus_core::Element;
use dioxus_native_core::prelude::{ElementNode, TextNode};
use dioxus_native_core::real_dom::NodeImmutable;
use dioxus_native_core::{node::NodeType, NodeId};
use freya_layout::{DioxusDOM, DioxusNode, RenderData};
use freya_node_state::{CursorSettings, FontStyle};
use skia_safe::{
    textlayout::{
        FontCollection, Paragraph, ParagraphBuilder, ParagraphStyle, RectHeightStyle,
        RectWidthStyle, TextHeightBehavior, TextStyle,
    },
    Canvas, Paint, PaintStyle, Rect,
};

/// Render a `paragraph` element
pub fn render_paragraph(
    node: &RenderData,
    node_ref: DioxusNode,
    canvas: &mut Canvas,
    font_collection: &mut FontCollection,
) {
    let node_font_style = &*node_ref.get::<FontStyle>().unwrap();
    let node_cursor_settings = &*node_ref.get::<CursorSettings>().unwrap();

    let texts = get_inner_texts(node_ref);

    let (x, y) = node.node_area.get_origin_points();

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

    paragraph.layout(node.node_area.width);

    paragraph.paint(canvas, (x, y));

    // Draw a cursor if specified
    draw_cursor(node, node_ref, paragraph, node_cursor_settings, canvas);
}

fn draw_cursor(
    node: &RenderData,
    node_ref: DioxusNode,
    paragraph: Paragraph,
    node_cursor_settings: &CursorSettings,
    canvas: &mut Canvas,
) -> Option<()> {
    let cursor = node_cursor_settings.position?;
    let cursor_color = node_cursor_settings.color;
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

fn get_inner_texts(node_ref: DioxusNode) -> Vec<(FontStyle, String)> {
    node_ref
        .children()
        .iter()
        .filter_map(|child| {
            if let NodeType::Element(ElementNode { tag, .. }) = &*child.node_type() {
                let children = child.children();
                if tag != "text" {
                    return None;
                }
                let child_text = children.get(0)?;
                if let NodeType::Text(TextNode { text, .. }) = &*child_text.node_type() {
                    let child_font_style = child_text.get::<FontStyle>().unwrap().clone();
                    Some((child_font_style, text.clone()))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<(FontStyle, String)>>()
}
