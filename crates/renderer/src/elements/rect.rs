use freya_native_core::real_dom::NodeImmutable;

use freya_core::dom::DioxusNode;
use freya_engine::prelude::*;
use freya_node_state::{BorderAlignment, BorderStyle, Fill, References, ShadowPosition, Style};
use torin::prelude::Area;

/// Render a `rect` element
pub fn render_rect(
    area: &Area,
    node_ref: &DioxusNode,
    canvas: &Canvas,
    font_collection: &mut FontCollection,
) {
    let node_style = &*node_ref.get::<Style>().unwrap();

    let mut paint = Paint::default();
    let mut path = Path::new();
    let area = area.to_f32();

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

    let radius = node_style.corner_radius;
    let rounded_rect = RRect::new_rect_radii(
        Rect::new(area.min_x(), area.min_y(), area.max_x(), area.max_y()),
        &[
            (radius.top_left, radius.top_left).into(),
            (radius.top_right, radius.top_right).into(),
            (radius.bottom_right, radius.bottom_right).into(),
            (radius.bottom_left, radius.bottom_left).into(),
        ],
    );

    if node_style.corner_radius.smoothing > 0.0 {
        path.add_path(
            &node_style.corner_radius.smoothed_path(rounded_rect),
            (area.min_x(), area.min_y()),
            None,
        );
    } else {
        path.add_rrect(rounded_rect, None);
    }

    canvas.draw_path(&path, &paint);

    // Shadows
    for shadow in node_style.shadows.iter() {
        if shadow.fill != Fill::Color(Color::TRANSPARENT) {
            let mut shadow_paint = paint.clone();
            let mut shadow_path = Path::new();

            match &shadow.fill {
                Fill::Color(color) => {
                    shadow_paint.set_color(*color);
                }
                Fill::LinearGradient(gradient) => {
                    shadow_paint.set_shader(gradient.into_shader(area));
                }
            }

            // Shadows can be either outset or inset
            // If they are outset, we fill a copy of the path outset by spread_radius, and blur it.
            // Otherwise, we draw a stroke with the inner portion being spread_radius width, and the outer portion being blur_radius width.
            let outset: Point = match shadow.position {
                ShadowPosition::Normal => {
                    shadow_paint.set_style(PaintStyle::Fill);
                    (shadow.spread, shadow.spread).into()
                }
                ShadowPosition::Inset => {
                    shadow_paint.set_style(PaintStyle::Stroke);
                    shadow_paint.set_stroke_width(shadow.blur / 2.0 + shadow.spread);
                    (-shadow.spread / 2.0, -shadow.spread / 2.0).into()
                }
            };

            // Apply gassuan blur to the copied path.
            if shadow.blur > 0.0 {
                shadow_paint.set_mask_filter(MaskFilter::blur(
                    BlurStyle::Normal,
                    shadow.blur / 2.0,
                    false,
                ));
            }

            // Add either the RRect or smoothed path based on whether smoothing is used.
            if node_style.corner_radius.smoothing > 0.0 {
                shadow_path.add_path(
                    &node_style
                        .corner_radius
                        .smoothed_path(rounded_rect.with_outset(outset)),
                    Point::new(area.min_x(), area.min_y()) - outset,
                    None,
                );
            } else {
                shadow_path.add_rrect(rounded_rect.with_outset(outset), None);
            }

            // Offset our path by the shadow's x and y coordinates.
            shadow_path.offset((shadow.x, shadow.y));

            // Exclude the original path bounds from the shadow using a clip, then draw the shadow.
            canvas.save();
            canvas.clip_path(
                &path,
                match shadow.position {
                    ShadowPosition::Normal => ClipOp::Difference,
                    ShadowPosition::Inset => ClipOp::Intersect,
                },
                true,
            );
            canvas.draw_path(&shadow_path, &shadow_paint);
            canvas.restore();
        }
    }

    // Borders
    if node_style.border.width > 0.0 && node_style.border.style != BorderStyle::None {
        // Create a new paint and path
        let mut border_paint = paint.clone();
        let mut border_path = Path::new();

        // Setup paint params
        border_paint.set_anti_alias(true);
        border_paint.set_style(PaintStyle::Stroke);
        match &node_style.border.fill {
            Fill::Color(color) => {
                border_paint.set_color(*color);
            }
            Fill::LinearGradient(gradient) => {
                border_paint.set_shader(gradient.into_shader(area));
            }
        }
        border_paint.set_stroke_width(node_style.border.width);

        // Skia draws strokes centered on the edge of the path. This means that half of the stroke is inside the path, and half outside.
        // For Inner and Outer borders, we need to grow or shrink the stroke path by half the border width.
        let outset = Point::new(node_style.border.width / 2.0, node_style.border.width / 2.0)
            * match node_style.border.alignment {
                BorderAlignment::Center => 0.0,
                BorderAlignment::Inner => -1.0,
                BorderAlignment::Outer => 1.0,
            };

        // Add either the RRect or smoothed path based on whether smoothing is used.
        if node_style.corner_radius.smoothing > 0.0 {
            border_path.add_path(
                &node_style
                    .corner_radius
                    .smoothed_path(rounded_rect.with_outset(outset)),
                Point::new(area.min_x(), area.min_y()) - outset,
                None,
            );
        } else {
            border_path.add_rrect(rounded_rect.with_outset(outset), None);
        }

        canvas.draw_path(&border_path, &border_paint);
    }

    let references = node_ref.get::<References>().unwrap();

    if let Some(canvas_ref) = &references.canvas_ref {
        (canvas_ref.runner)(canvas, font_collection, area);
    }
}
