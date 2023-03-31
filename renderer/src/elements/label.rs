use dioxus_native_core::node::NodeType;
use dioxus_native_core::tree::TreeView;
use freya_dom::{DioxusNode, FreyaDOM};
use freya_layout::RenderData;
use skia_safe::{
    textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle},
    Canvas, Paint, PaintStyle,
};

/// Render a `label` element
pub fn render_label(
    render_node: &RenderData,
    dioxus_node: &DioxusNode,
    dom: &FreyaDOM,
    canvas: &mut Canvas,
    font_collection: &mut FontCollection,
) {
    let font_size = dioxus_node.state.font_style.font_size;
    let font_family = &dioxus_node.state.font_style.font_family;
    let font_color = dioxus_node.state.font_style.color;
    let align = dioxus_node.state.font_style.align;
    let font_style = dioxus_node.state.font_style.font_style;

    let mut paint = Paint::default();

    paint.set_anti_alias(true);
    paint.set_style(PaintStyle::StrokeAndFill);
    paint.set_color(dioxus_node.state.font_style.color);

    let children = dom.dom().tree.children(render_node.node_id);

    let text = if let Some(children) = children {
        children
            .filter_map(|child| {
                if let NodeType::Text { text } = &child.node_data.node_type {
                    Some(text)
                } else {
                    None
                }
            })
            .next()
    } else {
        None
    };

    if let Some(text) = text {
        let x = render_node.node_area.x;
        let y = render_node.node_area.y;

        let mut paragraph_style = ParagraphStyle::default();
        paragraph_style.set_text_align(align);
        paragraph_style.set_text_style(
            TextStyle::new()
                .set_font_style(font_style)
                .set_color(font_color)
                .set_font_size(font_size)
                .set_font_families(font_family),
        );
        let mut paragraph_builder =
            ParagraphBuilder::new(&paragraph_style, font_collection.clone());

        paragraph_builder.add_text(text);

        let mut paragraph = paragraph_builder.build();

        paragraph.layout(render_node.node_area.width + 1.0);

        paragraph.paint(canvas, (x, y));
    }
}
