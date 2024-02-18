use freya_core::layout::create_label;
use freya_dom::prelude::DioxusNode;
use freya_engine::prelude::*;
use torin::geometry::Area;

/// Render a `label` element
pub fn render_label(
    area: &Area,
    node_ref: &DioxusNode,
    canvas: &Canvas,
    font_collection: &mut FontCollection,
) {
    let paragraph = create_label(node_ref, area, font_collection);

    let x = area.min_x();
    let y = area.min_y();

    paragraph.paint(canvas, (x, y));
}
