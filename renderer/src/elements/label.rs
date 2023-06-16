use dioxus_native_core::node::NodeType;
use dioxus_native_core::prelude::TextNode;
use dioxus_native_core::real_dom::NodeImmutable;
use freya_dom::prelude::DioxusNode;
use freya_node_state::FontStyle;
use skia_safe::{
    textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle},
    Canvas,
};
use torin::geometry::Area;

/// Render a `label` element
pub fn render_label(
    area: &Area,
    node_ref: &DioxusNode,
    canvas: &mut Canvas,
    font_collection: &mut FontCollection,
) {
    let node_children = node_ref.children();
    let node_font_style = &*node_ref.get::<FontStyle>().unwrap();

    let child = node_children.first();

    let text = if let Some(child) = child {
        if let NodeType::Text(TextNode { text, .. }) = &*child.node_type() {
            Some(text.clone())
        } else {
            None
        }
    } else {
        None
    };

    if let Some(text) = text {
        let x = area.min_x();
        let y = area.min_y();

        let mut paragraph_style = ParagraphStyle::default();
        paragraph_style.set_text_align(node_font_style.align);
        paragraph_style.set_text_style(&node_font_style.into());

        let mut paragraph_builder =
            ParagraphBuilder::new(&paragraph_style, font_collection.clone());

        paragraph_builder.add_text(text);

        let mut paragraph = paragraph_builder.build();
        paragraph.layout(area.width() + 1.0);
        paragraph.paint(canvas, (x, y));
    }
}
