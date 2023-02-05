use dioxus_native_core::tree::TreeView;
use dioxus_native_core::{node::NodeType, NodeId};
use freya_core::SharedRealDOM;
use freya_layout::RenderData;
use skia_safe::{
    textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle},
    Canvas, Paint, PaintStyle,
};

/// Render a `label` element
pub fn render_label(
    dom: &SharedRealDOM,
    canvas: &mut Canvas,
    font_collection: &mut FontCollection,
    node: &RenderData,
    children: &[NodeId],
) {
    let font_size = node.get_state().font_style.font_size;
    let font_family = &node.get_state().font_style.font_family;
    let font_color = node.get_state().font_style.color;
    let align = node.get_state().font_style.align;
    let font_style = node.get_state().font_style.font_style;

    let mut paint = Paint::default();

    paint.set_anti_alias(true);
    paint.set_style(PaintStyle::StrokeAndFill);
    paint.set_color(node.get_state().font_style.color);

    let child_id = children.get(0);

    let text = if let Some(child_id) = child_id {
        let dom = dom.lock().unwrap();
        if let Some(child) = dom.get(*child_id) {
            if let NodeType::Text { text } = &child.node_data.node_type {
                Some(text.clone())
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    if let Some(text) = text {
        let x = node.node_area.x;
        let y = node.node_area.y;

        let mut paragraph_style = ParagraphStyle::default();
        paragraph_style.set_text_align(align);
        paragraph_style.set_text_style(
            TextStyle::new()
                .set_font_style(font_style)
                .set_color(font_color)
                .set_font_size(font_size)
                .set_font_families(&[font_family]),
        );
        let mut paragraph_builder =
            ParagraphBuilder::new(&paragraph_style, font_collection.clone());

        paragraph_builder.add_text(text);

        let mut paragraph = paragraph_builder.build();

        paragraph.layout(node.node_area.width + 1.0);

        paragraph.paint(canvas, (x, y));
    }
}
