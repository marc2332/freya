use dioxus_native_core::real_dom::NodeImmutable;
use freya_dom::DioxusNode;
use freya_layout::RenderData;
use freya_node_state::{References, Style};
use skia_safe::{
    textlayout::FontCollection, BlurStyle, Canvas, MaskFilter, Paint, PaintStyle, Path,
    PathDirection, Rect,
};

/// Render a `rect` or a `container` element
pub fn render_rect_container(
    render_node: &RenderData,
    node_ref: &DioxusNode,
    canvas: &mut Canvas,
    font_collection: &FontCollection,
) {
    let node_style = &*node_ref.get::<Style>().unwrap();

    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_style(PaintStyle::Fill);
    paint.set_color(node_style.background);

    let radius = node_style.radius;
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

    let references = node_ref.get::<References>().unwrap();

    if let Some(canvas_ref) = &references.canvas_ref {
        (canvas_ref.runner)(canvas, font_collection, area);
    }
}
