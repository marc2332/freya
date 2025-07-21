use freya_engine::prelude::*;
use freya_native_core::real_dom::NodeImmutable;
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
use tracing::error;

use super::utils::ElementUtils;
use crate::{
    custom_attributes::CanvasRunnerContext,
    dom::{
        DioxusNode,
        ImagesCache,
    },
    render::{
        border_shape,
        render_border,
        render_shadow,
        BorderShape,
    },
    states::{
        CanvasState,
        StyleState,
        TransformState,
    },
    values::{
        Fill,
        ShadowPosition,
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
        let radius = node_style.corner_radius.with_scale(scale_factor);

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

    pub fn has_blur(&self, node_transform: &TransformState, node_style: &StyleState) -> bool {
        if node_transform.backdrop_blur != 0.0 {
            // If we can guarantee that the node entirely draws over it's backdrop,
            // we can avoid this whole (possibly intense) process, since the node's
            // backdrop is never visible.
            //
            // There's probably more that can be done in this area, like checking individual gradient
            // stops but I'm not completely sure of the diminishing returns here.
            //
            // Currently we verify the following:
            // - Node has a single solid color background.
            // - The background has 100% opacity.
            // - The node has no parents with the `opacity` attribute applied.
            if let Fill::Color(color) = node_style.background {
                color.a() != u8::MAX
                    || node_transform.blend_mode.is_some()
                    || !node_transform.opacities.is_empty()
            } else {
                true
            }
        } else {
            false
        }
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
        _fallback_fonts: &[String],
        _images_cache: &mut ImagesCache,
        scale_factor: f32,
    ) {
        let node_style = &*node_ref.get::<StyleState>().unwrap();
        let node_transform = &*node_ref.get::<TransformState>().unwrap();

        let area = layout_node.visible_area().to_f32();
        let mut path = Path::new();
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Fill);

        node_style.background.apply_to_paint(&mut paint, area);

        let corner_radius = node_style.corner_radius.with_scale(scale_factor);

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

        if self.has_blur(node_transform, node_style) {
            let blur_filter = blur(
                (
                    node_transform.backdrop_blur * scale_factor,
                    node_transform.backdrop_blur * scale_factor,
                ),
                None,
                None,
                rounded_rect.rect(),
            );

            if let Some(blur_filter) = blur_filter {
                let layer_rec = SaveLayerRec::default()
                    .bounds(rounded_rect.rect())
                    .backdrop(&blur_filter);

                // Depending on if the rect is rounded or not, we might need to clip the blur
                // layer to the shape of the rounded rect.
                if corner_radius.is_round() {
                    canvas.save();
                    canvas.clip_rrect(rounded_rect, ClipOp::Intersect, true);
                    canvas.save_layer(&layer_rec);
                    canvas.restore();
                    canvas.restore();
                } else {
                    canvas.save_layer(&layer_rec);
                    canvas.restore();
                }
            } else {
                error!("Unable to create blur filter.");
            }
        }

        // Paint the rect's background.
        canvas.draw_path(&path, &paint);

        // Shadows
        for shadow in node_style.shadows.iter() {
            if shadow.fill != Fill::Color(Color::TRANSPARENT) {
                let shadow = shadow.with_scale(scale_factor);

                render_shadow(
                    canvas,
                    node_style,
                    &mut path,
                    rounded_rect,
                    area,
                    &shadow,
                    &corner_radius,
                );
            }
        }

        // Borders
        for border in node_style.borders.iter() {
            if border.is_visible() {
                let border = border.with_scale(scale_factor);
                let rect = rounded_rect.rect().round_in().into();
                render_border(canvas, rect, area, &border, &corner_radius);
            }
        }

        // Canvas reference
        let references = node_ref.get::<CanvasState>().unwrap();
        if let Some(canvas_ref) = &references.canvas_ref {
            let mut ctx = CanvasRunnerContext {
                canvas,
                font_collection,
                area,
                scale_factor,
            };
            (canvas_ref.runner.lock().unwrap())(&mut ctx);
        }
    }

    #[inline]
    fn element_needs_cached_area(&self, _node_ref: &DioxusNode, style_state: &StyleState) -> bool {
        !style_state.borders.is_empty() || !style_state.shadows.is_empty()
    }

    fn element_drawing_area(
        &self,
        layout_node: &LayoutNode,
        _node_ref: &DioxusNode,
        scale_factor: f32,
        node_style: &StyleState,
    ) -> Area {
        let mut area = layout_node.visible_area();

        if node_style.borders.is_empty() && node_style.shadows.is_empty() {
            return area;
        }

        let mut path = Path::new();

        let corner_radius = node_style.corner_radius.with_scale(scale_factor);

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

        // Shadows
        for shadow in node_style.shadows.iter() {
            if shadow.fill != Fill::Color(Color::TRANSPARENT) {
                let shadow = shadow.with_scale(scale_factor);

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
                    if corner_radius.smoothing > 0.0 {
                        shadow_path.add_path(
                            &corner_radius.smoothed_path(rounded_rect.with_outset(outset)),
                            Point::new(area.min_x(), area.min_y()) - outset,
                            None,
                        );
                    } else {
                        shadow_path.add_rrect(rounded_rect.with_outset(outset), None);
                    }
                }

                shadow_path.offset((shadow.x, shadow.y));

                // Why 3? Because it seems to be used by skia internally
                let good_enough_blur = shadow.blur * 3.;

                let shadow_bounds = *shadow_path.bounds();
                let shadow_rect = shadow_bounds.with_outset((good_enough_blur, good_enough_blur));
                let shadow_area = Area::new(
                    Point2D::new(shadow_rect.x(), shadow_rect.y()),
                    Size2D::new(shadow_rect.width(), shadow_rect.height()),
                );

                area = area.union(&shadow_area);
            }
        }

        for border in node_style.borders.iter() {
            if border.is_visible() {
                let border = border.with_scale(scale_factor);

                let border_shape = border_shape(*rounded_rect.rect(), &corner_radius, &border);
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
