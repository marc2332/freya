use dioxus_native_core::node::NodeType;
use dioxus_native_core::prelude::TextNode;
use dioxus_native_core::real_dom::NodeImmutable;
use freya_dom::prelude::DioxusNode;
use freya_engine::prelude::*;
use freya_layout::create_text;
use torin::geometry::Area;

/// Render a `label` element
pub fn render_label(
    area: &Area,
    node_ref: &DioxusNode,
    canvas: &mut Canvas,
    font_collection: &mut FontCollection,
) {
    let node_children = node_ref.children();

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
        let paragraph = create_text(node_ref, area, font_collection, &text);

        let x = area.min_x();
        let y = area.min_y();

        paragraph.paint(canvas, (x, y));
    }
}
