use dioxus_native_core::real_dom::NodeImmutable;
use freya_layout::{DioxusDOM, DioxusNode, RenderData};
use freya_node_state::{ShadowSettings, Style};
use skia_safe::{BlurStyle, Canvas, MaskFilter, Paint, PaintStyle, Path, PathDirection, Rect};

/// Render a `rect` or a `container` element
pub fn render_rect_container(node: &RenderData, node_ref: DioxusNode, canvas: &mut Canvas) {
    let node_style = &*node_ref.get::<Style>().unwrap();

    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_style(PaintStyle::Fill);
    paint.set_color(node_style.background);

    let radius = node_style.radius;
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
        if node_style.shadow.intensity > 0 {
            let mut blur_paint = paint.clone();

            blur_paint.set_color(node_style.shadow.color);
            blur_paint.set_alpha(node_style.shadow.intensity);
            blur_paint.set_mask_filter(MaskFilter::blur(
                BlurStyle::Normal,
                node_style.shadow.size,
                false,
            ));
            canvas.draw_path(&path, &blur_paint);
        }
    }

    canvas.draw_path(&path, &paint);
}
