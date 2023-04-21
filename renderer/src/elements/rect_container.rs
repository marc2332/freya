use freya_dom::DioxusNode;
use freya_layout::RenderData;
use skia_safe::{BlurStyle, Canvas, MaskFilter, Paint, PaintStyle, Path, PathDirection, Rect};

/// Render a `rect` or a `container` element
pub fn render_rect_container(
    canvas: &mut Canvas,
    render_node: &RenderData,
    dioxus_node: &DioxusNode,
) {
    let shadow = &dioxus_node.state.style.shadow;

    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_style(PaintStyle::Fill);
    paint.set_color(dioxus_node.state.style.background);

    let radius = dioxus_node.state.style.radius;
    let radius = if radius < 0.0 { 0.0 } else { radius };

    let area = render_node.node_area.to_f32();

    let mut path = Path::new();
    path.add_round_rect(
        Rect::new(area.min_x(), area.min_y(), area.max_x(), area.max_y()),
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

    if let Some(canvas_ref) = &dioxus_node.state.references.canvas_ref {
        (canvas_ref.runner)(canvas, area);
    }
}
