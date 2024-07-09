use freya_engine::prelude::*;
use freya_native_core::real_dom::NodeImmutable;
use freya_node_state::{
    BorderAlignment,
    BorderStyle,
    Fill,
    ReferencesState,
    ShadowPosition,
    StyleState,
};
use torin::{
    prelude::{
        CursorPoint,
        LayoutNode,
    },
    scaled::Scaled,
};

use super::utils::ElementUtils;
use crate::dom::DioxusNode;

pub struct RectElement;

impl RectElement {
    fn get_rounded_rect(
        &self,
        layout_node: &LayoutNode,
        node_ref: &DioxusNode,
        scale_factor: f32,
    ) -> RRect {
        let area = layout_node.visible_area().to_f32();
        let node_style = &*node_ref.get::<StyleState>().unwrap();
        let mut radius = node_style.corner_radius;
        radius.scale(scale_factor);

        RRect::new_rect_radii(
            Rect::new(area.min_x(), area.min_y(), area.max_x(), area.max_y()),
            &[
                (radius.top_left, radius.top_left).into(),
                (radius.top_right, radius.top_right).into(),
                (radius.bottom_right, radius.bottom_right).into(),
                (radius.bottom_left, radius.bottom_left).into(),
            ],
        )
    }
}

impl ElementUtils for RectElement {
    fn is_point_inside_area(
        &self,
        point: &CursorPoint,
        node_ref: &DioxusNode,
        layout_node: &LayoutNode,
        scale_factor: f32,
    ) -> bool {
        let rounded_rect = self.get_rounded_rect(layout_node, node_ref, scale_factor);
        let point = point.to_f32();
        rounded_rect.contains(Rect::new(point.x, point.y, point.x + 1., point.y + 1.))
    }

    fn clip(
        &self,
        layout_node: &LayoutNode,
        node_ref: &DioxusNode,
        canvas: &Canvas,
        scale_factor: f32,
    ) {
        let rounded_rect = self.get_rounded_rect(layout_node, node_ref, scale_factor);

        canvas.clip_rrect(rounded_rect, ClipOp::Intersect, true);
    }

    fn render(
        self,
        layout_node: &LayoutNode,
        node_ref: &DioxusNode,
        canvas: &Canvas,
        font_collection: &mut FontCollection,
        _font_manager: &FontMgr,
        _default_fonts: &[String],
        scale_factor: f32,
    ) {
        let node_style = &*node_ref.get::<StyleState>().unwrap();

        let mut paint = Paint::default();
        let mut path = Path::new();
        let area = layout_node.visible_area().to_f32();

        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Fill);

        match &node_style.background {
            Fill::Color(color) => {
                paint.set_color(*color);
            }
            Fill::LinearGradient(gradient) => {
                paint.set_shader(gradient.into_shader(area));
            }
            Fill::RadialGradient(gradient) => {
                paint.set_shader(gradient.into_shader(area));
            }
            Fill::ConicGradient(gradient) => {
                paint.set_shader(gradient.into_shader(area));
            }
        }

        let mut radius = node_style.corner_radius;
        radius.scale(scale_factor);

        let rounded_rect = RRect::new_rect_radii(
            Rect::new(area.min_x(), area.min_y(), area.max_x(), area.max_y()),
            &[
                (radius.top_left, radius.top_left).into(),
                (radius.top_right, radius.top_right).into(),
                (radius.bottom_right, radius.bottom_right).into(),
                (radius.bottom_left, radius.bottom_left).into(),
            ],
        );

        if radius.smoothing > 0.0 {
            path.add_path(
                &radius.smoothed_path(rounded_rect),
                (area.min_x(), area.min_y()),
                None,
            );
        } else {
            path.add_rrect(rounded_rect, None);
        }

        canvas.draw_path(&path, &paint);

        // Shadows
        for mut shadow in node_style.shadows.clone().into_iter() {
            if shadow.fill != Fill::Color(Color::TRANSPARENT) {
                shadow.scale(scale_factor);
                let mut shadow_paint = paint.clone();
                let mut shadow_path = Path::new();

                match &shadow.fill {
                    Fill::Color(color) => {
                        shadow_paint.set_color(*color);
                    }
                    Fill::LinearGradient(gradient) => {
                        shadow_paint.set_shader(gradient.into_shader(area));
                    }
                    Fill::RadialGradient(gradient) => {
                        shadow_paint.set_shader(gradient.into_shader(area));
                    }
                    Fill::ConicGradient(gradient) => {
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
                if radius.smoothing > 0.0 {
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
            let mut border_with = node_style.border.width;
            border_with *= scale_factor;

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
                Fill::RadialGradient(gradient) => {
                    border_paint.set_shader(gradient.into_shader(area));
                }
                Fill::ConicGradient(gradient) => {
                    border_paint.set_shader(gradient.into_shader(area));
                }
            }
            border_paint.set_stroke_width(border_with);

            // Skia draws strokes centered on the edge of the path. This means that half of the stroke is inside the path, and half outside.
            // For Inner and Outer borders, we need to grow or shrink the stroke path by half the border width.
            let outset = Point::new(border_with / 2.0, border_with / 2.0)
                * match node_style.border.alignment {
                    BorderAlignment::Center => 0.0,
                    BorderAlignment::Inner => -1.0,
                    BorderAlignment::Outer => 1.0,
                };

            // Add either the RRect or smoothed path based on whether smoothing is used.
            if radius.smoothing > 0.0 {
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

        let references = node_ref.get::<ReferencesState>().unwrap();

        if let Some(canvas_ref) = &references.canvas_ref {
            (canvas_ref.runner)(canvas, font_collection, area, scale_factor);
        }
    }
}
