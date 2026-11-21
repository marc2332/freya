use freya_engine::prelude::{
    BlendMode,
    Canvas,
    ClipOp,
    Colors,
    FontCollection,
    FontMgr,
    Gradient,
    Interpolation,
    Paint,
    SaveLayerRec,
    SkColor4f,
    SkMatrix,
    SkPoint,
    SkRect,
    TileMode,
    blur,
    shaders,
};
use rustc_hash::FxHashMap;
use torin::prelude::Area;

use crate::{
    data::EffectState,
    element::{
        ClipContext,
        RenderContext,
    },
    node_id::NodeId,
    prelude::Color,
    style::{
        render_callback::{
            RenderContext as FillRenderContext,
            RenderTarget,
        },
        shadow::ShadowPosition,
    },
    tree::Tree,
};

pub struct RenderPipeline<'a> {
    pub font_collection: &'a mut FontCollection,
    pub font_manager: &'a FontMgr,
    pub canvas: &'a Canvas,
    pub tree: &'a Tree,
    pub scale_factor: f64,
    pub background: Color,
}

impl RenderPipeline<'_> {
    fn scale_transformed_area(&self, mut area: Area, scale_node_ids: &[NodeId]) -> Area {
        for node_id in scale_node_ids {
            let layout_node = self.tree.layout.get(node_id).unwrap();
            let effect = self.tree.effect_state.get(node_id).unwrap();
            let node_area = layout_node.visible_area();
            let origin = effect.transform_origin.origin(&node_area);
            let scale = effect.scale.unwrap();

            area = area.translate(-origin.to_vector());
            area = area.scale(scale.x, scale.y);
            area = area.translate(origin.to_vector());
        }
        area
    }

    /// Whether `area` reaches into the faded edges of `fade_id`, rather than lying wholly in
    /// its opaque middle.
    fn reaches_fade_edge(&self, fade_id: NodeId, area: &Area) -> bool {
        let Some(fade) = self
            .tree
            .effect_state
            .get(&fade_id)
            .and_then(|effect| effect.fade)
        else {
            return false;
        };
        let bounds = self.tree.layout.get(&fade_id).unwrap().visible_area();
        let scale_factor = self.scale_factor as f32;
        let (start, end, from, to) = match fade.horizontal {
            false => (bounds.min_y(), bounds.max_y(), area.min_y(), area.max_y()),
            true => (bounds.min_x(), bounds.max_x(), area.min_x(), area.max_x()),
        };
        from < start + fade.start * scale_factor || to > end - fade.end * scale_factor
    }

    /// Masks what was drawn in the current layer with the fade of `fade_id`: transparent at
    /// its edges, opaque inside.
    fn apply_fade(&self, fade_id: NodeId) {
        let Some(fade) = self
            .tree
            .effect_state
            .get(&fade_id)
            .and_then(|effect| effect.fade)
        else {
            return;
        };
        let area = self.tree.layout.get(&fade_id).unwrap().visible_area();
        let scale_factor = self.scale_factor as f32;
        let (from, to, length) = match fade.horizontal {
            false => (
                SkPoint::new(area.min_x(), area.min_y()),
                SkPoint::new(area.min_x(), area.max_y()),
                area.height(),
            ),
            true => (
                SkPoint::new(area.min_x(), area.min_y()),
                SkPoint::new(area.max_x(), area.min_y()),
                area.width(),
            ),
        };
        if length <= 0. {
            return;
        }
        let start = (fade.start * scale_factor / length).clamp(0., 0.5);
        let end = (1. - fade.end * scale_factor / length).clamp(0.5, 1.);
        let clear = SkColor4f::new(0., 0., 0., 0.);
        let solid = SkColor4f::new(0., 0., 0., 1.);
        let colors = [clear, solid, solid, clear];
        let offsets = [0., start, end, 1.];
        let gradient = Gradient::new(
            Colors::new(&colors, Some(&offsets), TileMode::Clamp, None),
            Interpolation::default(),
        );
        let Some(shader) = shaders::linear_gradient((from, to), &gradient, None) else {
            return;
        };
        let mut paint = Paint::default();
        paint.set_shader(shader);
        paint.set_blend_mode(BlendMode::DstIn);
        self.canvas.draw_rect(
            self.canvas.local_clip_bounds().unwrap_or(SkRect::new(
                area.min_x(),
                area.min_y(),
                area.max_x(),
                area.max_y(),
            )),
            &paint,
        );
    }

    fn apply_rotations(&self, effect_state: &EffectState) {
        for node_id in effect_state.rotations.iter() {
            let Some(layout_node) = self.tree.layout.get(node_id) else {
                continue;
            };
            let Some(effect) = self.tree.effect_state.get(node_id) else {
                continue;
            };
            let Some(rotation) = effect.rotation else {
                continue;
            };

            let origin = effect.transform_origin.origin(&layout_node.visible_area());
            let mut matrix = SkMatrix::new_identity();
            matrix.set_rotate(rotation, Some(SkPoint::new(origin.x, origin.y)));
            self.canvas.concat(&matrix);
        }
    }

    fn apply_scales(&self, effect_state: &EffectState) {
        for node_id in effect_state.scales.iter() {
            let Some(layout_node) = self.tree.layout.get(node_id) else {
                continue;
            };
            let Some(effect) = self.tree.effect_state.get(node_id) else {
                continue;
            };
            let Some(scale) = effect.scale else { continue };

            let origin = effect.transform_origin.origin(&layout_node.visible_area());
            self.canvas.translate((origin.x, origin.y));
            self.canvas.scale((scale.x, scale.y));
            self.canvas.translate((-origin.x, -origin.y));
        }
    }

    fn apply_mask(&mut self, node_id: NodeId) {
        let Some(layout_node) = self.tree.layout.get(&node_id) else {
            return;
        };
        let Some(element) = self.tree.elements.get(&node_id) else {
            return;
        };
        let Some(effect) = element.effect() else {
            return;
        };
        let Some(mask) = &effect.mask else { return };

        let area = layout_node.visible_area();
        let bounds = SkRect::new(area.min_x(), area.min_y(), area.max_x(), area.max_y());
        let mut paint = Paint::default();
        paint.set_blend_mode(BlendMode::DstIn);

        let layer = self.canvas.save();
        self.canvas
            .save_layer(&SaveLayerRec::default().bounds(&bounds).paint(&paint));
        self.canvas.clip_rect(bounds, ClipOp::Intersect, true);

        if let Some(callback) = mask.render_callback() {
            if let Some(text_style_state) = self.tree.text_style_state.get(&node_id) {
                callback.call(&mut FillRenderContext {
                    target: RenderTarget::Background {
                        canvas: self.canvas,
                        font_collection: self.font_collection,
                    },
                    scale_factor: self.scale_factor,
                    origin: area.origin,
                    size: area.size,
                    text_style_state,
                });
            }
        } else {
            let mut paint = Paint::default();
            mask.apply_to_paint(&mut paint, area);
            self.canvas.draw_rect(bounds, &paint);
        }

        self.canvas.restore_to_count(layer);
    }

    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn render(mut self) {
        self.canvas.clear(self.background);
        let base_matrix = self.canvas.local_to_device();

        let mut clip_areas: FxHashMap<NodeId, Area> = FxHashMap::default();

        // TODO: Use incremental rendering
        for i16 in itertools::sorted(self.tree.layers.keys()) {
            let nodes = self.tree.layers.get(i16).unwrap();
            'rendering: for node_id in nodes {
                let layout_node = self.tree.layout.get(node_id).unwrap();

                if layout_node.hidden {
                    continue 'rendering;
                }

                let effect_state = self.tree.effect_state.get(node_id);
                let element = self.tree.elements.get(node_id).unwrap();
                if !element.paints_content()
                    && effect_state.is_none_or(|effect| effect.blur.is_none())
                {
                    continue 'rendering;
                }

                let mut visible_area = layout_node.visible_area();

                if let Some(effect_state) = effect_state {
                    visible_area = self.scale_transformed_area(visible_area, &effect_state.scales);

                    for clip_node_id in effect_state.clips.iter() {
                        let clip_area = *clip_areas.entry(*clip_node_id).or_insert_with(|| {
                            let clip_layout_node = self.tree.layout.get(clip_node_id).unwrap();
                            let clip_effect = self.tree.effect_state.get(clip_node_id).unwrap();
                            self.scale_transformed_area(
                                clip_layout_node.visible_area(),
                                &clip_effect.scales,
                            )
                        });

                        if !visible_area.intersects(&clip_area) {
                            continue 'rendering;
                        }
                    }
                }

                let layer = self.canvas.save();

                let text_style_state = self.tree.text_style_state.get(node_id).unwrap();

                if let Some(effect_state) = effect_state {
                    hotpath::measure_block!("Element Clipping", {
                        for clip_node_id in effect_state.clips.iter() {
                            let clip_element = self.tree.elements.get(clip_node_id).unwrap();
                            let clip_area = clip_areas.get(clip_node_id).unwrap();

                            let clip_context = ClipContext {
                                canvas: self.canvas,
                                visible_area: clip_area,
                                scale_factor: self.scale_factor,
                            };

                            clip_element.clip(clip_context);
                        }
                    });

                    self.apply_rotations(effect_state);

                    let scale_factor = self.scale_factor as f32;

                    // Blur the content underneath before applying opacity.
                    if let Some(blur_radius) = effect_state.blur {
                        let render_rect = element.render_rect(&visible_area, scale_factor);
                        let style = element.style();

                        let image_filter = blur(
                            (blur_radius * scale_factor, blur_radius * scale_factor),
                            None,
                            None,
                            render_rect.rect(),
                        );
                        if let Some(image_filter) = image_filter {
                            let rec = SaveLayerRec::default()
                                .bounds(render_rect.rect())
                                .backdrop(&image_filter);

                            let blur_layer = self.canvas.save();
                            if style.corner_radius.is_round() {
                                self.canvas.clip_rrect(render_rect, ClipOp::Intersect, true);
                            }
                            self.canvas.save_layer(&rec);
                            self.canvas.restore_to_count(blur_layer);
                        }
                    }

                    let opacity = effect_state
                        .opacities
                        .iter()
                        .fold(1., |acc, opacity| acc * opacity);
                    if opacity < 1. {
                        let render_rect = element.render_rect(&visible_area, scale_factor);
                        let mut layer_bounds = *render_rect.rect();
                        for shadow in element.style().shadows.iter() {
                            if shadow.position == ShadowPosition::Normal {
                                let outset_x = shadow.x.abs() + shadow.spread + shadow.blur;
                                let outset_y = shadow.y.abs() + shadow.spread + shadow.blur;
                                layer_bounds = layer_bounds.with_outset((
                                    outset_x * scale_factor,
                                    outset_y * scale_factor,
                                ));
                            }
                        }
                        self.canvas.save_layer_alpha_f(layer_bounds, opacity);
                    }

                    self.apply_scales(effect_state);
                }

                let masks = effect_state
                    .map(|effect| effect.masks.as_ref())
                    .unwrap_or_default();
                if !masks.is_empty() {
                    let render_rect =
                        element.render_rect(&layout_node.visible_area(), self.scale_factor as f32);
                    let mut bounds = *render_rect.rect();

                    for shadow in element.style().shadows.iter() {
                        if shadow.position == ShadowPosition::Normal {
                            let outset_x = shadow.x.abs() + shadow.spread + shadow.blur;
                            let outset_y = shadow.y.abs() + shadow.spread + shadow.blur;
                            bounds = bounds.with_outset((
                                outset_x * self.scale_factor as f32,
                                outset_y * self.scale_factor as f32,
                            ));
                        }
                    }

                    // Keep masks from erasing content behind this element.
                    self.canvas
                        .save_layer(&SaveLayerRec::default().bounds(&bounds));
                }

                // Under a fade, the element draws into a layer of its own that the fades' masks
                // then cut into.
                // Only the fades whose edges this element reaches into: one wholly inside a
                // fade's opaque middle needs no layer and no mask.
                let fades: Vec<NodeId> = effect_state
                    .filter(|_| element.paints_content())
                    .map(|effect_state| {
                        effect_state
                            .fades
                            .iter()
                            .copied()
                            .filter(|fade_id| self.reaches_fade_edge(*fade_id, &visible_area))
                            .collect()
                    })
                    .unwrap_or_default();
                if !fades.is_empty() && masks.is_empty() {
                    self.canvas.save_layer(&SaveLayerRec::default());
                }

                let render_context = RenderContext {
                    font_collection: self.font_collection,
                    canvas: self.canvas,
                    layout_node,
                    text_style_state,
                    scale_factor: self.scale_factor,
                };

                hotpath::measure_block!("Element Render", {
                    element.render(render_context);
                });

                for fade_id in fades.iter() {
                    self.apply_fade(*fade_id);
                }
                for mask_id in masks.iter() {
                    let mask_layer = self.canvas.save();
                    self.canvas.set_matrix(&base_matrix);

                    if let Some(effect) = self.tree.effect_state.get(mask_id) {
                        self.apply_rotations(effect);
                        self.apply_scales(effect);
                    }

                    self.apply_mask(*mask_id);
                    self.canvas.restore_to_count(mask_layer);
                }

                self.canvas.restore_to_count(layer);
            }
        }
    }
}
