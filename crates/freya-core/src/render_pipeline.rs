use freya_engine::prelude::{
    BlendMode,
    Canvas,
    ClipOp,
    FontCollection,
    FontMgr,
    Paint,
    SaveLayerRec,
    SkMatrix,
    SkPoint,
    SkRect,
    blur,
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

                let element = self.tree.elements.get(node_id).unwrap();
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

                    let render_rect = element.render_rect(&visible_area, self.scale_factor as f32);

                    let mut layer_bounds = *render_rect.rect();
                    let scale_factor = self.scale_factor as f32;

                    for shadow in element.style().shadows.iter() {
                        if shadow.position == ShadowPosition::Normal {
                            let outset_x = shadow.x.abs() + shadow.spread + shadow.blur;
                            let outset_y = shadow.y.abs() + shadow.spread + shadow.blur;
                            layer_bounds = layer_bounds
                                .with_outset((outset_x * scale_factor, outset_y * scale_factor));
                        }
                    }

                    // Blur the content underneath before applying opacity.
                    if let Some(blur_radius) = effect_state.blur {
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
