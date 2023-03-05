use dioxus_native_core::tree::TreeView;
use dioxus_native_core::{node::NodeType, NodeId};
use freya_core::SharedRealDOM;
use freya_layout::{DioxusNode, RenderData};
use freya_node_state::NodeState;
use skia_safe::{
    textlayout::{
        FontCollection, Paragraph, ParagraphBuilder, ParagraphStyle, RectHeightStyle,
        RectWidthStyle, TextHeightBehavior, TextStyle,
    },
    Canvas, Paint, PaintStyle, Rect,
};

/// Render a `paragraph` element
pub fn render_paragraph(
    dom: &SharedRealDOM,
    canvas: &mut Canvas,
    font_collection: &mut FontCollection,
    node: &RenderData,
    children: &[NodeId],
) {
    let align = node.get_state().font_style.align;
    let max_lines = node.get_state().font_style.max_lines;

    let texts = get_inner_texts(children, dom);

    let (x, y) = node.node_area.get_origin_points();

    let mut paragraph_style = ParagraphStyle::default();
    paragraph_style.set_max_lines(max_lines);
    paragraph_style.set_text_align(align);
    paragraph_style.set_replace_tab_characters(true);
    paragraph_style.set_text_height_behavior(TextHeightBehavior::DisableAll);

    let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, font_collection.clone());

    for node_text in &texts {
        paragraph_builder.push_style(
            TextStyle::new()
                .set_font_style(node_text.0.font_style.font_style)
                .set_height_override(true)
                .set_height(node_text.0.font_style.line_height)
                .set_color(node_text.0.font_style.color)
                .set_font_size(node_text.0.font_style.font_size)
                .set_font_families(&node_text.0.font_style.font_family),
        );
        paragraph_builder.add_text(node_text.1.clone());
    }

    if node.get_state().cursor_settings.position.is_some() {
        // This is very tricky, but it works! It allows freya to render the cursor at the end of a line.
        paragraph_builder.add_text(" ");
    }

    let mut paragraph = paragraph_builder.build();

    paragraph.layout(node.node_area.width);

    paragraph.paint(canvas, (x, y));

    // Draw a cursor if specified
    draw_cursor(node, paragraph, canvas);
}

fn draw_cursor(node: &RenderData, paragraph: Paragraph, canvas: &mut Canvas) -> Option<()> {
    let cursor = node.get_state().cursor_settings.position?;
    let cursor_color = node.get_state().cursor_settings.color;
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

fn get_inner_texts(children: &[NodeId], dom: &SharedRealDOM) -> Vec<(NodeState, String)> {
    children
        .iter()
        .filter_map(|child_id| {
            let (child, children): (DioxusNode, Vec<NodeId>) = {
                let dom = dom.lock().unwrap();
                let children = dom.tree.children_ids(*child_id).map(|v| v.to_vec());
                (dom.get(*child_id).cloned()?, children?)
            };

            if let NodeType::Element { tag, .. } = child.node_data.node_type {
                if tag != "text" {
                    return None;
                }
                let child_text_id = children.get(0)?;
                let child_text: DioxusNode = {
                    let dom = dom.lock().unwrap();
                    dom.get(*child_text_id).cloned()
                }?;
                if let NodeType::Text { text } = &child_text.node_data.node_type {
                    Some((child.state, text.clone()))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<(NodeState, String)>>()
}
