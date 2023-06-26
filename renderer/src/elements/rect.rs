use dioxus_native_core::real_dom::NodeImmutable;
use freya_dom::prelude::DioxusNode;
use freya_node_state::{BorderAlignment, BorderStyle, Fill, References, ShadowPosition, Style};
use skia_safe::{
    textlayout::FontCollection, BlurStyle, Canvas, ClipOp, Color, MaskFilter, Paint, PaintStyle,
    Path, PathDirection, RRect, Rect,
};
use torin::prelude::Area;

/// Render a `rect` element
pub fn render_rect(
    area: &Area,
    node_ref: &DioxusNode,
    canvas: &mut Canvas,
    font_collection: &FontCollection,
) {
    let node_style = &*node_ref.get::<Style>().unwrap();

    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_style(PaintStyle::Fill);

    let area = area.to_f32();

    match &node_style.background {
        Fill::Color(color) => {
            paint.set_color(*color);
        }
        Fill::LinearGradient(gradient) => {
            paint.set_shader(gradient.into_shader(area));
        }
    }

    let radius = node_style.radius;
    let radius = &[
        (radius.top_left(), radius.top_left()).into(),
        (radius.top_right(), radius.top_right()).into(),
        (radius.bottom_right(), radius.bottom_right()).into(),
        (radius.bottom_left(), radius.bottom_left()).into(),
    ];

    let mut path = Path::new();
    let rounded_rect = RRect::new_rect_radii(
        Rect::new(area.min_x(), area.min_y(), area.max_x(), area.max_y()),
        radius,
    );

    path.add_rrect(rounded_rect, None);
    canvas.draw_path(&path, &paint);

    // Shadow effect
    // A box shadow is created by creating a copy of the drawn rectangle
    // and applying a blur filter and a clip.
    //
    // Before applying the filter, we can translate and scale the rectangle
    // to adjust intensity and blur position.
    //
    // If a shadow is inset, then we instead draw an inner stroke and blur that,
    // clipping whatever blur escapes the shadow's bounding
    for shadow in node_style.shadows.iter() {
        if shadow.fill != Fill::Color(Color::TRANSPARENT) {
            let mut blur_paint = paint.clone();
            let mut blur_rect = rounded_rect;

            blur_rect.offset((shadow.x, shadow.y));
            match &shadow.fill {
                Fill::Color(color) => {
                    blur_paint.set_color(*color);
                }
                Fill::LinearGradient(gradient) => {
                    blur_paint.set_shader(gradient.into_shader(area));
                }
            }

            if shadow.position == ShadowPosition::Inset {
                blur_paint.set_style(PaintStyle::Stroke);
                blur_paint.set_stroke_width(shadow.blur / 2.0 + shadow.spread);
                blur_rect.inset((shadow.spread / 2.0, shadow.spread / 2.0));
            } else {
                blur_rect.outset((shadow.spread, shadow.spread));
            }

            if shadow.blur > 0.0 {
                blur_paint.set_mask_filter(MaskFilter::blur(
                    BlurStyle::Normal,
                    shadow.blur / 2.0,
                    false,
                ));
            }

            path.rewind();

            path.add_rrect(blur_rect, Some((PathDirection::CW, 0)));

            // Exclude the original rect bounds from the shadow
            canvas.save();
            let clip_operation = if shadow.position == ShadowPosition::Inset {
                ClipOp::Intersect
            } else {
                ClipOp::Difference
            };
            canvas.clip_rrect(rounded_rect, clip_operation, true);
            canvas.draw_path(&path, &blur_paint);
            canvas.restore();
        }
    }

    // Borders
    if node_style.border.width > 0.0
        && node_style.border.fill != Fill::Color(Color::TRANSPARENT)
        && node_style.border.style != BorderStyle::None
    {
        let mut stroke_paint = paint.clone();
        let half_border_width = node_style.border.width / 2.0;

        stroke_paint.set_style(PaintStyle::Stroke);
        match &node_style.border.fill {
            Fill::Color(color) => {
                stroke_paint.set_color(*color);
            }
            Fill::LinearGradient(gradient) => {
                stroke_paint.set_shader(gradient.into_shader(area));
            }
        }
        stroke_paint.set_stroke_width(node_style.border.width);

        path.rewind();

        let mut border_rect = rounded_rect;

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
