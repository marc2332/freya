use dioxus_native_core::real_dom::NodeImmutable;
use freya_dom::prelude::DioxusNode;
use freya_node_state::{BorderAlignment, BorderStyle, References, ShadowPosition, Style};
use skia_safe::{
    textlayout::FontCollection, BlurStyle, Canvas, ClipOp, Color, MaskFilter, Paint, PaintStyle,
    Path, RRect, Rect,
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
    paint.set_color(node_style.background);

    let radius = node_style.corner_radius;
    let radius = &[
        (radius.top_left, radius.top_left).into(),
        (radius.top_right, radius.top_right).into(),
        (radius.bottom_right, radius.bottom_right).into(),
        (radius.bottom_left, radius.bottom_left).into(),
    ];
    
    let mut path = Path::new();
    let area = area.to_f32();

    let smoothing_path = node_style.corner_radius.clone().smoothed_path(area);
    let rounded_rect = RRect::new_rect_radii(
        Rect::new(area.min_x(), area.min_y(), area.max_x(), area.max_y()),
        radius,
    );

    if node_style.corner_radius.smoothing > 0.0 {
        path.add_path(&smoothing_path, (area.min_x(), area.min_y()), None);
    } else {
        path.add_rrect(&rounded_rect, None);
    }

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
        if shadow.color != Color::TRANSPARENT {
            let mut shadow_paint = paint.clone();
            let mut shadow_path = path.clone();
            
            shadow_path.offset((shadow.x, shadow.y));

            shadow_paint.set_color(shadow.color);
            shadow_paint.set_stroke_width(shadow.spread);

            match shadow.position {
                ShadowPosition::Normal => shadow_paint.set_style(PaintStyle::StrokeAndFill),
                ShadowPosition::Inset => shadow_paint.set_style(PaintStyle::Stroke),
            };

            if shadow.blur > 0.0 {
                shadow_paint.set_mask_filter(MaskFilter::blur(
                    BlurStyle::Normal,
                    shadow.blur / 2.0,
                    false,
                ));
            }

            // Exclude the original path bounds from the shadow
            canvas.save();
            let clip_operation = match shadow.position {
                ShadowPosition::Normal => ClipOp::Difference,
                ShadowPosition::Inset => ClipOp::Intersect,
            };
            canvas.clip_path(&path, clip_operation, true);
            canvas.draw_path(&shadow_path, &shadow_paint);
            canvas.restore();
        }
    }

    // Borders
    if node_style.border.width > 0.0 && node_style.border.style != BorderStyle::None {
        let mut border_paint = paint.clone();

        border_paint.set_style(PaintStyle::Stroke);
        border_paint.set_color(node_style.border.color);
        border_paint.set_stroke_width(
            if node_style.border.alignment == BorderAlignment::Center {
                node_style.border.width
            } else {
                node_style.border.width * 2.0
            }
        );

        match node_style.border.alignment {
            BorderAlignment::Outer => {
                canvas.clip_path(&path, ClipOp::Difference, true);
            },
            BorderAlignment::Inner => {
                canvas.clip_path(&path, ClipOp::Intersect, true);
            },
            _ => {},
        }
        canvas.draw_path(&path, &border_paint);
    }

    let references = node_ref.get::<References>().unwrap();

    if let Some(canvas_ref) = &references.canvas_ref {
        (canvas_ref.runner)(canvas, font_collection, area);
    }
}
