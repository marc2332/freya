use freya_native_core::real_dom::NodeImmutable;

use freya_core::dom::DioxusNode;
use freya_engine::prelude::*;
use freya_node_state::Style;
use torin::geometry::Area;

/// Render a `svg` element
pub fn render_svg(area: &Area, node_ref: &DioxusNode, canvas: &Canvas, font_manager: &FontMgr) {
    let node_style = &*node_ref.get::<Style>().unwrap();

    let x = area.min_x();
    let y = area.min_y();
    if let Some(svg_data) = &node_style.svg_data {
        let svg_dom = svg::Dom::from_bytes(svg_data.as_slice(), font_manager);
        if let Ok(mut svg_dom) = svg_dom {
            canvas.save();
            canvas.translate((x, y));
            svg_dom.set_container_size((area.width() as i32, area.height() as i32));
            svg_dom.render(canvas);
            canvas.restore();
        }
    }
}
