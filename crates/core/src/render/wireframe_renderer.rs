use freya_engine::prelude::*;
use torin::geometry::Area;

/// Render a wireframe around the given node
pub fn render_wireframe(canvas: &Canvas, area: &Area) {
    let mut paint = Paint::default();

    paint.set_anti_alias(true);
    paint.set_style(PaintStyle::Fill);
    paint.set_color(Color::MAGENTA);

    let x = area.min_x();
    let y = area.min_y();

    let x2 = x + area.width();
    let y2 = if area.height() < 0.0 {
        y
    } else {
        y + area.height()
    };

    canvas.draw_line((x, y), (x2, y), &paint);
    canvas.draw_line((x2, y), (x2, y2), &paint);
    canvas.draw_line((x2, y2), (x, y2), &paint);
    canvas.draw_line((x, y2), (x, y), &paint);
}
