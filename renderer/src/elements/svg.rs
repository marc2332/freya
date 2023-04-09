use freya_dom::DioxusNode;
use freya_layout::RenderData;
use skia_safe::{svg, Canvas};

/// Render a `svg` element
pub fn render_svg(render_node: &RenderData, dioxus_node: &DioxusNode, canvas: &mut Canvas) {
    let x = render_node.node_area.min_x();
    let y = render_node.node_area.min_y();
    if let Some(svg_data) = &dioxus_node.state.style.svg_data {
        let svg_dom = svg::Dom::from_bytes(svg_data);
        if let Ok(mut svg_dom) = svg_dom {
            canvas.save();
            canvas.translate((x, y));
            svg_dom.set_container_size((
                render_node.node_area.width() as i32,
                render_node.node_area.height() as i32,
            ));
            svg_dom.render(canvas);
            canvas.restore();
        }
    }
}
