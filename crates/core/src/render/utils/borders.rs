use freya_engine::prelude::*;
use freya_node_state::{
    Border,
    BorderAlignment,
    CornerRadius,
};
use torin::prelude::Area;

pub enum BorderShape {
    DRRect(RRect, RRect),
    Path(Path),
}

pub fn render_border(
    canvas: &Canvas,
    rounded_rect: RRect,
    area: Area,
    border: &Border,
    corner_radius: CornerRadius,
) {
    // Create a new paint
    let mut border_paint = Paint::default();
    border_paint.set_style(PaintStyle::Fill);
    border_paint.set_anti_alias(true);

    border.fill.apply_to_paint(&mut border_paint, area);

    match border_shape(*rounded_rect.rect(), corner_radius, border) {
        BorderShape::DRRect(outer, inner) => {
            canvas.draw_drrect(outer, inner, &border_paint);
        }
        BorderShape::Path(path) => {
            canvas.draw_path(&path, &border_paint);
        }
    }
}

/// Returns a `Path` that will draw a [`Border`] around a base rectangle.
///
/// We don't use Skia's stroking API here, since we might need different widths for each side.
pub fn border_shape(
    base_rect: Rect,
    base_corner_radius: CornerRadius,
    border: &Border,
) -> BorderShape {
    let border_alignment = border.alignment;
    let border_width = border.width;

    // First we create a path that is outset from the rect by a certain amount on each side.
    //
    // Let's call this the outer border path.
    let (outer_rrect, outer_corner_radius) = {
        // Calculuate the outer corner radius for the border.
        let corner_radius = CornerRadius {
            top_left: outer_border_path_corner_radius(
                border_alignment,
                base_corner_radius.top_left,
                border_width.top,
                border_width.left,
            ),
            top_right: outer_border_path_corner_radius(
                border_alignment,
                base_corner_radius.top_right,
                border_width.top,
                border_width.right,
            ),
            bottom_left: outer_border_path_corner_radius(
                border_alignment,
                base_corner_radius.bottom_left,
                border_width.bottom,
                border_width.left,
            ),
            bottom_right: outer_border_path_corner_radius(
                border_alignment,
                base_corner_radius.bottom_right,
                border_width.bottom,
                border_width.right,
            ),
            smoothing: base_corner_radius.smoothing,
        };

        let rrect = RRect::new_rect_radii(
            {
                let mut rect = base_rect;
                let alignment_scale = match border_alignment {
                    BorderAlignment::Outer => 1.0,
                    BorderAlignment::Center => 0.5,
                    BorderAlignment::Inner => 0.0,
                };

                rect.left -= border_width.left * alignment_scale;
                rect.top -= border_width.top * alignment_scale;
                rect.right += border_width.right * alignment_scale;
                rect.bottom += border_width.bottom * alignment_scale;

                rect
            },
            &[
                (corner_radius.top_left, corner_radius.top_left).into(),
                (corner_radius.top_right, corner_radius.top_right).into(),
                (corner_radius.bottom_right, corner_radius.bottom_right).into(),
                (corner_radius.bottom_left, corner_radius.bottom_left).into(),
            ],
        );

        (rrect, corner_radius)
    };

    // After the outer path, we will then move to the inner bounds of the border.
    let (inner_rrect, inner_corner_radius) = {
        // Calculuate the inner corner radius for the border.
        let corner_radius = CornerRadius {
            top_left: inner_border_path_corner_radius(
                border_alignment,
                base_corner_radius.top_left,
                border_width.top,
                border_width.left,
            ),
            top_right: inner_border_path_corner_radius(
                border_alignment,
                base_corner_radius.top_right,
                border_width.top,
                border_width.right,
            ),
            bottom_left: inner_border_path_corner_radius(
                border_alignment,
                base_corner_radius.bottom_left,
                border_width.bottom,
                border_width.left,
            ),
            bottom_right: inner_border_path_corner_radius(
                border_alignment,
                base_corner_radius.bottom_right,
                border_width.bottom,
                border_width.right,
            ),
            smoothing: base_corner_radius.smoothing,
        };

        let rrect = RRect::new_rect_radii(
            {
                let mut rect = base_rect;
                let alignment_scale = match border_alignment {
                    BorderAlignment::Outer => 0.0,
                    BorderAlignment::Center => 0.5,
                    BorderAlignment::Inner => 1.0,
                };

                rect.left += border_width.left * alignment_scale;
                rect.top += border_width.top * alignment_scale;
                rect.right -= border_width.right * alignment_scale;
                rect.bottom -= border_width.bottom * alignment_scale;

                rect
            },
            &[
                (corner_radius.top_left, corner_radius.top_left).into(),
                (corner_radius.top_right, corner_radius.top_right).into(),
                (corner_radius.bottom_right, corner_radius.bottom_right).into(),
                (corner_radius.bottom_left, corner_radius.bottom_left).into(),
            ],
        );

        (rrect, corner_radius)
    };

    if base_corner_radius.smoothing > 0.0 {
        let mut path = Path::new();
        path.set_fill_type(PathFillType::EvenOdd);

        path.add_path(
            &outer_corner_radius.smoothed_path(outer_rrect),
            Point::new(outer_rrect.rect().x(), outer_rrect.rect().y()),
            None,
        );

        path.add_path(
            &inner_corner_radius.smoothed_path(inner_rrect),
            Point::new(inner_rrect.rect().x(), inner_rrect.rect().y()),
            None,
        );

        BorderShape::Path(path)
    } else {
        BorderShape::DRRect(outer_rrect, inner_rrect)
    }
}

fn outer_border_path_corner_radius(
    alignment: BorderAlignment,
    corner_radius: f32,
    width_1: f32,
    width_2: f32,
) -> f32 {
    if alignment == BorderAlignment::Inner || corner_radius == 0.0 {
        return corner_radius;
    }

    let mut offset = if width_1 == 0.0 {
        width_2
    } else if width_2 == 0.0 {
        width_1
    } else {
        width_1.min(width_2)
    };

    if alignment == BorderAlignment::Center {
        offset *= 0.5;
    }

    corner_radius + offset
}

fn inner_border_path_corner_radius(
    alignment: BorderAlignment,
    corner_radius: f32,
    width_1: f32,
    width_2: f32,
) -> f32 {
    if alignment == BorderAlignment::Outer || corner_radius == 0.0 {
        return corner_radius;
    }

    let mut offset = if width_1 == 0.0 {
        width_2
    } else if width_2 == 0.0 {
        width_1
    } else {
        width_1.min(width_2)
    };

    if alignment == BorderAlignment::Center {
        offset *= 0.5;
    }

    corner_radius - offset
}
