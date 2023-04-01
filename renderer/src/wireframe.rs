use freya_layout::RenderData;
use skia_safe::{Canvas, Color, Paint, PaintStyle};

/// Render a wireframe around the given node
pub fn render_wireframe(canvas: &mut Canvas, render_node: &RenderData) {
    let mut paint = Paint::default();

    paint.set_anti_alias(true);
    paint.set_style(PaintStyle::Fill);
    paint.set_color(Color::MAGENTA);

    let x = render_node.node_area.x;
    let y = render_node.node_area.y;

    let x2 = x + render_node.node_area.width;
    let y2 = if render_node.node_area.height < 0.0 {
        y
    } else {
        y + render_node.node_area.height
    };

    canvas.draw_line((x, y), (x2, y), &paint);
    canvas.draw_line((x2, y), (x2, y2), &paint);
    canvas.draw_line((x2, y2), (x, y2), &paint);
    canvas.draw_line((x, y2), (x, y), &paint);
}
