use dioxus_native_core::real_dom::NodeImmutable;
use freya_dom::prelude::DioxusNode;
use freya_node_state::{BorderAlignment, BorderStyle, References, Style};
use skia_safe::{
    textlayout::FontCollection, BlurStyle, Canvas, MaskFilter, Paint, PaintStyle, Path,
    PathDirection, RRect, Rect,
};
use torin::prelude::Area;

/// Render a `rect` or a `container` element
pub fn render_rect_container(
    area: &Area,
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
    let radius = &[
        (radius.top_left(), radius.top_left()).into(),
        (radius.top_right(), radius.top_right()).into(),
        (radius.bottom_right(), radius.bottom_right()).into(),
        (radius.bottom_left(), radius.bottom_left()).into(),
    ];

    let area = area.to_f32();

    let mut path = Path::new();
    let rect = Rect::new(area.min_x(), area.min_y(), area.max_x(), area.max_y());
    let rounded_rect = RRect::new_rect_radii(rect, radius);
    path.add_rrect(rounded_rect, None);
    path.close();

    // Shadow effect
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

    canvas.draw_path(&path, &paint);

    // Borders
    if node_style.border.width > 0.0 && node_style.border.style != BorderStyle::None {
        let mut stroke_paint = paint.clone();
        let half_border_width = node_style.border.width / 2.0;

        stroke_paint.set_style(PaintStyle::Stroke);
        stroke_paint.set_color(node_style.border.color);
        stroke_paint.set_stroke_width(node_style.border.width);

        path.rewind();

        let mut border_rect = RRect::new_rect_radii(rect, radius);

        match node_style.border.alignment {
            BorderAlignment::Inner => {
                border_rect.inset((half_border_width, half_border_width));
            }
            BorderAlignment::Outer => {
                border_rect.outset((half_border_width, half_border_width));
            }
            BorderAlignment::Center => (),
        }

        path.add_rrect(border_rect, Some((PathDirection::CW, 0)));

        canvas.draw_path(&path, &stroke_paint);
    }

    let references = node_ref.get::<References>().unwrap();

    if let Some(canvas_ref) = &references.canvas_ref {
        (canvas_ref.runner)(canvas, font_collection, area);
    }
}
