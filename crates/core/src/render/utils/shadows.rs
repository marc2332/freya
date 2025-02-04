use freya_engine::prelude::*;
use torin::prelude::Area;

use crate::{
    states::StyleState,
    values::{
        CornerRadius,
        Shadow,
        ShadowPosition,
    },
};

pub fn render_shadow(
    canvas: &Canvas,
    node_style: &StyleState,
    path: &mut Path,
    rounded_rect: RRect,
    area: Area,
    shadow: Shadow,
    corner_radius: CornerRadius,
) {
    let mut shadow_path = Path::new();
    let mut shadow_paint = Paint::default();
    shadow_paint.set_anti_alias(true);

    shadow.fill.apply_to_paint(&mut shadow_paint, area);

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
    if corner_radius.smoothing > 0.0 {
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
        path,
        match shadow.position {
            ShadowPosition::Normal => ClipOp::Difference,
            ShadowPosition::Inset => ClipOp::Intersect,
        },
        true,
    );
    canvas.draw_path(&shadow_path, &shadow_paint);
    canvas.restore();
}
