use dioxus_native_core::real_dom::NodeImmutable;
use freya_dom::DioxusNode;
use freya_node_state::Style;
use skia_safe::{svg, Canvas};
use torin::geometry::Area;

/// Render a `svg` element
pub fn render_svg(area: &Area, node_ref: &DioxusNode, canvas: &mut Canvas) {
    let node_style = &*node_ref.get::<Style>().unwrap();

    let x = area.min_x();
    let y = area.min_y();
    if let Some(svg_data) = &node_style.svg_data {
        let svg_dom = svg::Dom::from_bytes(svg_data);
        if let Ok(mut svg_dom) = svg_dom {
            canvas.save();
            canvas.translate((x, y));
            svg_dom.set_container_size((area.width() as i32, area.height() as i32));
            svg_dom.render(canvas);
            canvas.restore();
        }
    }
}
