use freya_engine::prelude::*;
use freya_native_core::real_dom::NodeImmutable;
use freya_node_state::{
    CanvasRunnerContext,
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
use crate::{
    dom::DioxusNode,
    render::{
        border_shape,
        render_border,
        render_shadow,
        BorderShape,
        ParagraphCache,
    },
};

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
        _paragraph_cache: &mut ParagraphCache,
    ) {
        let node_style = &*node_ref.get::<StyleState>().unwrap();

        let area = layout_node.visible_area().to_f32();
        let mut path = Path::new();
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Fill);

        node_style.background.apply_to_paint(&mut paint, area);

        let mut corner_radius = node_style.corner_radius;
        corner_radius.scale(scale_factor);

        // Container
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

                render_shadow(
                    canvas,
                    node_style,
                    &mut path,
                    rounded_rect,
                    area,
                    shadow,
                    corner_radius,
                );
            }
        }

        // Borders
        for mut border in node_style.borders.clone().into_iter() {
            if border.is_visible() {
                border.scale(scale_factor);

                render_border(canvas, rounded_rect, area, &border, corner_radius);
            }
        }

        // Layout references
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

    #[inline]
    fn element_needs_cached_area(&self, node_ref: &DioxusNode) -> bool {
        let node_style = &*node_ref.get::<StyleState>().unwrap();

        !node_style.borders.is_empty() || !node_style.shadows.is_empty()
    }

    fn element_drawing_area(
        &self,
        layout_node: &LayoutNode,
        node_ref: &DioxusNode,
        scale_factor: f32,
    ) -> Area {
        let node_style = &*node_ref.get::<StyleState>().unwrap();
        let mut area = layout_node.visible_area();

        if node_style.borders.is_empty() && node_style.shadows.is_empty() {
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

        for mut border in node_style.borders.clone().into_iter() {
            if border.is_visible() {
                border.scale(scale_factor);

                let border_shape =
                    border_shape(*rounded_rect.rect(), node_style.corner_radius, &border);
                let border_bounds = match border_shape {
                    BorderShape::DRRect(ref outer, _) => outer.bounds(),
                    BorderShape::Path(ref path) => path.bounds(),
                };
                let border_area = Area::new(
                    Point2D::new(border_bounds.x(), border_bounds.y()),
                    Size2D::new(border_bounds.width(), border_bounds.height()),
                );

                area = area.union(&border_area.round_out());
            }
        }

        area
    }
}
