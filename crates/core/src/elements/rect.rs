use freya_engine::prelude::*;
use freya_native_core::real_dom::NodeImmutable;
use freya_node_state::{
    Border,
    BorderAlignment,
    CanvasRunnerContext,
    CornerRadius,
    Fill,
    ReferencesState,
    ShadowPosition,
    StyleState,
};
use torin::{
    prelude::{
        Area,
        CursorPoint,
        LayoutNode,
        Point2D,
        Size2D,
    },
    scaled::Scaled,
};

use super::utils::ElementUtils;
use crate::dom::DioxusNode;

pub struct RectElement;

impl RectElement {
    fn rrect(&self, layout_node: &LayoutNode, node_ref: &DioxusNode, scale_factor: f32) -> RRect {
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

    fn border_path_corner_radius_offset(alignment: BorderAlignment, width_1: f32, width_2: f32) -> f32 {
        if alignment == BorderAlignment::Inner {
            return 0.0;
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

        offset
    }

    fn border_clip_path_corner_radius_offset(alignment: BorderAlignment, width_1: f32, width_2: f32) -> f32 {
        if alignment == BorderAlignment::Outer {
            return 0.0;
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

        -offset
    }

    /// Returns the outer path to draw to create a border.
    fn border_path(base_rect: Rect, base_corner_radius: CornerRadius, border: &Border) -> Path {
        let border_alignment = border.alignment;
        let border_width = border.width;
        let border_radius = CornerRadius {
            top_left: base_corner_radius.top_left
                + Self::border_path_corner_radius_offset(
                    border_alignment,
                    border_width.top,
                    border_width.left,
                ),
            top_right: base_corner_radius.top_right
                + Self::border_path_corner_radius_offset(
                    border_alignment,
                    border_width.top,
                    border_width.right,
                ),
            bottom_left: base_corner_radius.bottom_left
                + Self::border_path_corner_radius_offset(
                    border_alignment,
                    border_width.bottom,
                    border_width.left,
                ),
            bottom_right: base_corner_radius.bottom_right
                + Self::border_path_corner_radius_offset(
                    border_alignment,
                    border_width.bottom,
                    border_width.right,
                ),
            smoothing: base_corner_radius.smoothing,
        };

        let alignment_scale = match border_alignment {
            BorderAlignment::Outer => 1.0,
            BorderAlignment::Center => 0.5,
            BorderAlignment::Inner => 0.0,
        };

        let mut path = Path::new();
        let rrect = RRect::new_rect_radii(
            {
                let mut rect = base_rect;

                rect.left -= border_width.left * alignment_scale;
                rect.top -= border_width.top * alignment_scale;
                rect.right += border_width.right * alignment_scale;
                rect.bottom += border_width.bottom * alignment_scale;

                rect
            },
            &[
                (border_radius.top_left, border_radius.top_left).into(),
                (border_radius.top_right, border_radius.top_right).into(),
                (border_radius.bottom_right, border_radius.bottom_right).into(),
                (border_radius.bottom_left, border_radius.bottom_left).into(),
            ],
        );

        if border_radius.smoothing > 0.0 {
            path.add_path(
                &border_radius.smoothed_path(rrect),
                Point::new(rrect.rect().x(), rrect.rect().y()),
                None,
            );
        } else {
            path.add_rrect(rrect, None);
        }

        path
    }

    /// Returns the inner path to clip from the outer path to create a border.
    fn border_clip_path(
        base_rect: Rect,
        base_corner_radius: CornerRadius,
        border: &Border,
    ) -> Path {
        let border_alignment = border.alignment;
        let border_width = border.width;
        let border_radius = CornerRadius {
            top_left: base_corner_radius.top_left
                + Self::border_clip_path_corner_radius_offset(
                    border_alignment,
                    border_width.top,
                    border_width.left,
                ),
            top_right: base_corner_radius.top_right
                + Self::border_clip_path_corner_radius_offset(
                    border_alignment,
                    border_width.top,
                    border_width.right,
                ),
            bottom_left: base_corner_radius.bottom_left
                + Self::border_clip_path_corner_radius_offset(
                    border_alignment,
                    border_width.bottom,
                    border_width.left,
                ),
            bottom_right: base_corner_radius.bottom_right
                + Self::border_clip_path_corner_radius_offset(
                    border_alignment,
                    border_width.bottom,
                    border_width.right,
                ),
            smoothing: base_corner_radius.smoothing,
        };

        let alignment_scale = match border_alignment {
            BorderAlignment::Outer => 0.0,
            BorderAlignment::Center => 0.5,
            BorderAlignment::Inner => 1.0,
        };

        let mut path = Path::new();
        let rrect = RRect::new_rect_radii(
            {
                let mut rect = base_rect;

                rect.left += border_width.left * alignment_scale;
                rect.top += border_width.top * alignment_scale;
                rect.right -= border_width.right * alignment_scale;
                rect.bottom -= border_width.bottom * alignment_scale;

                rect
            },
            &[
                (border_radius.top_left, border_radius.top_left).into(),
                (border_radius.top_right, border_radius.top_right).into(),
                (border_radius.bottom_right, border_radius.bottom_right).into(),
                (border_radius.bottom_left, border_radius.bottom_left).into(),
            ],
        );

        if border_radius.smoothing > 0.0 {
            path.add_path(
                &border_radius.smoothed_path(rrect),
                Point::new(rrect.rect().x(), rrect.rect().y()),
                None,
            );
        } else {
            path.add_rrect(rrect, None);
        }

        path
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
        let rounded_rect = self.rrect(layout_node, node_ref, scale_factor);
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
        let rounded_rect = self.rrect(layout_node, node_ref, scale_factor);

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

        let mut corner_radius = node_style.corner_radius;
        corner_radius.scale(scale_factor);

        let rounded_rect = RRect::new_rect_radii(
            Rect::new(area.min_x(), area.min_y(), area.max_x(), area.max_y()),
            &[
                (corner_radius.top_left, corner_radius.top_left).into(),
                (corner_radius.top_right, corner_radius.top_right).into(),
                (corner_radius.bottom_right, corner_radius.bottom_right).into(),
                (corner_radius.bottom_left, corner_radius.bottom_left).into(),
            ],
        );

        if corner_radius.smoothing > 0.0 {
            path.add_path(
                &corner_radius.smoothed_path(rounded_rect),
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
        if node_style.border.is_visible() {
            let mut border = node_style.border.clone();
            border.scale(scale_factor);

            // Create a new paint
            let mut border_paint = paint.clone();
            border_paint.set_anti_alias(true);
            border_paint.set_style(PaintStyle::Fill);
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

            let border_path = Self::border_path(*rounded_rect.rect(), corner_radius, &border);

            let border_clip_path =
                Self::border_clip_path(*rounded_rect.rect(), corner_radius, &border);

            canvas.save();
            canvas.clip_path(&border_clip_path, ClipOp::Difference, true);
            canvas.draw_path(&border_path, &border_paint);
            canvas.restore();
        }

        let references = node_ref.get::<ReferencesState>().unwrap();

        if let Some(canvas_ref) = &references.canvas_ref {
            let mut ctx = CanvasRunnerContext {
                canvas,
                font_collection,
                area,
                scale_factor,
            };
            (canvas_ref.runner)(&mut ctx);
        }
    }

    fn element_drawing_area(
        &self,
        layout_node: &LayoutNode,
        node_ref: &DioxusNode,
        scale_factor: f32,
    ) -> Area {
        let node_style = &*node_ref.get::<StyleState>().unwrap();
        let mut area = layout_node.visible_area();

        if !node_style.border.is_visible() && node_style.shadows.is_empty() {
            return area;
        }

        let mut path = Path::new();

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

        // Shadows
        for mut shadow in node_style.shadows.clone().into_iter() {
            if shadow.fill != Fill::Color(Color::TRANSPARENT) {
                shadow.scale(scale_factor);

                let mut shadow_path = Path::new();

                let outset: Option<Point> = match shadow.position {
                    ShadowPosition::Normal => Some(
                        (
                            shadow.spread.max(shadow.blur),
                            shadow.spread.max(shadow.blur),
                        )
                            .into(),
                    ),
                    ShadowPosition::Inset => None, // No need to consider inset shadows for the drawing area as they will always be smaller.
                };

                if let Some(outset) = outset {
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
                }

                shadow_path.offset((shadow.x, shadow.y));

                let shadow_bounds = shadow_path.bounds();
                let shadow_area = Area::new(
                    Point2D::new(shadow_bounds.x(), shadow_bounds.y()),
                    Size2D::new(shadow_bounds.width(), shadow_bounds.height()),
                );
                area = area.union(&shadow_area);
            }
        }

        if node_style.border.is_visible() {
            let mut border = node_style.border.clone();
            border.scale(scale_factor);

            let border_path =
                Self::border_path(*rounded_rect.rect(), node_style.corner_radius, &border);

            let border_bounds = border_path.bounds();
            let border_area = Area::new(
                Point2D::new(border_bounds.x(), border_bounds.y()),
                Size2D::new(border_bounds.width(), border_bounds.height()),
            );

            area = area.union(&border_area.round_out());
        }

        area
    }
}
