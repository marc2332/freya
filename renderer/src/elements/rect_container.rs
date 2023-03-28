use freya_dom::FreyaDOM;
use freya_layout::RenderData;
use skia_safe::{BlurStyle, Canvas, MaskFilter, Paint, PaintStyle, Path, PathDirection, Rect};

/// Render a `rect` or a `container` element
pub fn render_rect_container(canvas: &mut Canvas, node: &RenderData, rdom: &FreyaDOM) {
    let dioxus_node = node.get_node(rdom);
    let shadow = &dioxus_node.state.style.shadow;

    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_style(PaintStyle::Fill);
    paint.set_color(dioxus_node.state.style.background);

    let radius = dioxus_node.state.style.radius;
    let radius = if radius < 0.0 { 0.0 } else { radius };

    let ((x, y), (x2, y2)) = node.node_area.get_rect();

    let mut path = Path::new();
    path.add_round_rect(
        Rect::new(x as f32, y as f32, x2 as f32, y2 as f32),
        (radius, radius),
        PathDirection::CW,
    );
    path.close();

    // Shadow effect
    {
        if shadow.intensity > 0 {
            let mut blur_paint = paint.clone();

            blur_paint.set_color(shadow.color);
            blur_paint.set_alpha(shadow.intensity);
            blur_paint.set_mask_filter(MaskFilter::blur(BlurStyle::Normal, shadow.size, false));
            canvas.draw_path(&path, &blur_paint);
        }
    }

    canvas.draw_path(&path, &paint);
}
