use freya_layout::{DioxusDOM, RenderData};
use skia_safe::{svg, Canvas};

/// Render a `svg` element
pub fn render_svg(canvas: &mut Canvas, node: &RenderData, dom: &DioxusDOM) {
    let dioxus_node = node.get_node(dom);
    let x = node.node_area.x;
    let y = node.node_area.y;
    if let Some(svg_data) = &dioxus_node.state.style.svg_data {
        let svg_dom = svg::Dom::from_bytes(svg_data);
        if let Ok(mut svg_dom) = svg_dom {
            canvas.save();
            canvas.translate((x, y));
            svg_dom.set_container_size((node.node_area.width as i32, node.node_area.height as i32));
            svg_dom.render(canvas);
            canvas.restore();
        }
    }
}
