use freya_engine::prelude::{
    BlendMode,
    Canvas,
    Colors,
    Gradient,
    Interpolation,
    SkColor4f,
    Paint,
    SkRect,
    TileMode,
    shaders,
    ClipOp,
    FontCollection,
    FontMgr,
    SaveLayerRec,
    SkMatrix,
    SkPoint,
    blur,
};
use rustc_hash::FxHashMap;
use torin::prelude::Area;

use crate::{
    element::{
        ClipContext,
        RenderContext,
    },
    node_id::NodeId,
    prelude::Color,
    style::shadow::ShadowPosition,
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
    /// Transform an area with the accumulated scale effects of the given nodes.
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
        self.canvas.draw_rect(self.canvas.local_clip_bounds().unwrap_or(SkRect::new(
            area.min_x(),
            area.min_y(),
            area.max_x(),
            area.max_y(),
        )), &paint);
    }

    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn render(self) {
        self.canvas.clear(self.background);

        // Clip areas are deterministic no matter what node inherits them
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

                    // No need to render this element if it is completely clipped
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

                    // Pass rotate effect to children
                    for id in effect_state.rotations.iter() {
                        let layout_node = self.tree.layout.get(id).unwrap();
                        let effect = self.tree.effect_state.get(id).unwrap();
                        let area = layout_node.visible_area();
                        let origin = effect.transform_origin.origin(&area);
                        let mut matrix = SkMatrix::new_identity();
                        matrix.set_rotate(
                            effect.rotation.unwrap(),
                            Some(SkPoint {
                                x: origin.x,
                                y: origin.y,
                            }),
                        );
                        self.canvas.concat(&matrix);
                    }

                    let render_rect = element.render_rect(&visible_area, self.scale_factor as f32);

                    // Apply inherited opacity effects with bounds expanded
                    // to accommodate outset shadows
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

                    // Composite the blur before the opacity layers so it samples the real content underneath.
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

                    // Transform the canvas area given the scale effects
                    for id in effect_state.scales.iter() {
                        let layout_node = self.tree.layout.get(id).unwrap();
                        let effect = self.tree.effect_state.get(id).unwrap();
                        let area = layout_node.visible_area();
                        let origin = effect.transform_origin.origin(&area);
                        let scale = effect.scale.unwrap();

                        self.canvas.translate((origin.x, origin.y));
                        self.canvas.scale((scale.x, scale.y));
                        self.canvas.translate((-origin.x, -origin.y));
                    }
                }

                // Under a fade, the element draws into a layer of its own that the fades' masks
                // then cut into.
                let fades = effect_state
                    .map(|effect_state| effect_state.fades.clone())
                    .unwrap_or_default();
                if !fades.is_empty() {
                    self.canvas.save_layer(&SaveLayerRec::default());
                }

                let render_context = RenderContext {
                    font_collection: self.font_collection,
                    canvas: self.canvas,
                    layout_node,
                    tree: self.tree,
                    text_style_state,
                    scale_factor: self.scale_factor,
                };

                hotpath::measure_block!("Element Render", {
                    element.render(render_context);
                });

                for fade_id in fades.iter() {
                    self.apply_fade(*fade_id);
                }

                self.canvas.restore_to_count(layer);
            }
        }
    }
}
