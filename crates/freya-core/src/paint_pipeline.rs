#[cfg(feature = "incremental-painting")]
use freya_engine::prelude::{
    BlendMode,
    PathBuilder,
    RenderRecorder,
};
use freya_engine::prelude::{
    Canvas,
    ClipOp,
    FontCollection,
    FontMgr,
    Recording,
    RenderCanvas,
    SkMatrix,
    SkPoint,
    blur,
};
use torin::prelude::{
    Area,
    LayoutNode,
};

#[cfg(feature = "incremental-painting")]
use crate::geometry::{
    AreaExt,
    SkRectExt,
};
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

pub struct PaintPipeline<'a> {
    pub font_collection: &'a mut FontCollection,
    pub font_manager: &'a FontMgr,
    pub canvas: &'a Canvas,
    pub tree: &'a mut Tree,
    pub scale_factor: f64,
    pub background: Color,
    pub force_full: bool,
}

impl PaintPipeline<'_> {
    /// Transform an area with the accumulated scale effects of the given nodes.
    fn scale_transformed_area(tree: &Tree, mut area: Area, scale_node_ids: &[NodeId]) -> Area {
        for node_id in scale_node_ids {
            let layout_node = tree.layout.get(node_id).unwrap();
            let effect = tree.effect_state.get(node_id).unwrap();
            let node_area = layout_node.visible_area();
            let origin = effect.transform_origin.origin(&node_area);
            let scale = effect.scale.unwrap();

            area = area.translate(-origin.to_vector());
            area = area.scale(scale.x, scale.y);
            area = area.translate(origin.to_vector());
        }
        area
    }

    #[cfg(not(feature = "incremental-painting"))]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn paint(self) {
        self.canvas.clear(self.background);
        for layer in itertools::sorted(self.tree.layers.keys()) {
            for node_id in &self.tree.layers[layer] {
                let layout_node = self.tree.layout.get(node_id).unwrap();
                if layout_node.hidden {
                    continue;
                }
                let checkpoint = self.canvas.save();
                Self::paint_node(
                    self.tree,
                    self.font_collection,
                    self.scale_factor,
                    node_id,
                    layout_node,
                    RenderCanvas::Direct(self.canvas),
                );
                self.canvas.restore_to_count(checkpoint);
            }
        }
    }

    #[cfg(feature = "incremental-painting")]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn paint(mut self) {
        // `FREYA_FULL_RENDER` disables incremental painting as an escape hatch
        if self.force_full || std::env::var_os("FREYA_FULL_RENDER").is_some() {
            self.tree.paint_state.damage.mark_full();
        }

        if !self.tree.paint_state.damage.is_full() {
            self.pre_pass();
        }

        if self.tree.paint_state.damage.is_full() {
            self.paint_full();
        } else if !self.tree.paint_state.damage.is_empty() {
            self.paint_partial();
        }

        self.tree.paint_state.finish_paint();
    }

    /// Repaint everything, re-recording only the dirty and volatile nodes.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    #[cfg(feature = "incremental-painting")]
    fn paint_full(&mut self) {
        self.canvas.clear(self.background);
        let refresh_all = self.force_full || std::env::var_os("FREYA_FULL_RENDER").is_some();
        let tree = &mut *self.tree;

        for layer in itertools::sorted(tree.layers.keys()) {
            let nodes = tree.layers.get(layer).unwrap();
            for node_id in nodes {
                let layout_node = tree.layout.get(node_id).unwrap();

                if layout_node.hidden {
                    tree.paint_state.cache.remove(node_id);
                    continue;
                }

                if !refresh_all
                    && !tree.paint_state.dirty.contains(node_id)
                    && !tree.paint_state.volatile.contains(node_id)
                    && !tree.paint_state.area_invalidated.contains(node_id)
                    && let Some(recording) = tree.paint_state.cache.get(node_id)
                {
                    recording.replay(self.canvas, self.font_collection);
                    continue;
                }

                tree.paint_state.cache.remove(node_id);
                if let Some(recording) = Self::paint_node(
                    tree,
                    self.font_collection,
                    self.scale_factor,
                    node_id,
                    layout_node,
                    RenderCanvas::Recording(RenderRecorder::default()),
                ) {
                    recording.replay(self.canvas, self.font_collection);
                    tree.paint_state.cache.insert(*node_id, recording);
                }
            }
        }
    }

    /// Refresh invalid recordings and collect repaint regions.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    #[cfg(feature = "incremental-painting")]
    fn pre_pass(&mut self) {
        let tree = &mut *self.tree;
        let dirty = &tree.paint_state.dirty;
        let volatile_nodes = tree
            .paint_state
            .volatile
            .iter()
            .filter(|node_id| !dirty.contains(node_id));

        let area_invalidated = tree.paint_state.area_invalidated.iter().filter(|node_id| {
            !dirty.contains(node_id) && !tree.paint_state.volatile.contains(node_id)
        });

        for node_id in dirty.iter().chain(volatile_nodes).chain(area_invalidated) {
            let damage_bounds =
                dirty.contains(node_id) || tree.paint_state.volatile.contains(node_id);
            let Some(layout_node) = tree.layout.get(node_id) else {
                continue;
            };
            if !tree.elements.contains_key(node_id) {
                continue;
            }

            if let Some(old_recording) = tree.paint_state.cache.remove(node_id)
                && damage_bounds
            {
                tree.paint_state.damage.push_bounds(old_recording.bounds);
            }

            if !layout_node.hidden
                && let Some(recording) = Self::paint_node(
                    tree,
                    self.font_collection,
                    self.scale_factor,
                    node_id,
                    layout_node,
                    RenderCanvas::Recording(RenderRecorder::default()),
                )
            {
                if damage_bounds {
                    tree.paint_state.damage.push_bounds(recording.bounds);
                }
                tree.paint_state.cache.insert(*node_id, recording);
            }
        }

        hotpath::measure_block!("Backdrop Damage", {
            let paint_state = &mut tree.paint_state;
            let mut backdrop_nodes: Vec<(i16, NodeId)> = paint_state
                .cache
                .iter()
                .filter(|(_, recording)| recording.samples_backdrop)
                .map(|(node_id, _)| {
                    let layer = tree
                        .layer_state
                        .get(node_id)
                        .map(|layer_state| layer_state.layer)
                        .unwrap_or_default();
                    (layer, *node_id)
                })
                .collect();
            backdrop_nodes.sort_unstable();

            // Expand until overlapping backdrop sampling regions stop growing the damage.
            loop {
                let mut expanded = false;
                for (_, node_id) in &backdrop_nodes {
                    match paint_state.cache[node_id].bounds {
                        Some(bounds) => {
                            if paint_state.damage.intersects(&bounds)
                                && !paint_state.damage.contains(&bounds)
                            {
                                paint_state.damage.push(bounds);
                                expanded = true;
                            }
                        }
                        None => {
                            paint_state.damage.mark_full();
                            return;
                        }
                    }
                }
                if !expanded {
                    break;
                }
            }
        });
    }

    /// Repaint only the damaged regions, replaying cached recordings.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    #[cfg(feature = "incremental-painting")]
    fn paint_partial(&mut self) {
        let paint_state = &self.tree.paint_state;
        let damage = &paint_state.damage;
        let checkpoint = self.canvas.save();

        if let [single_rect] = damage.rects() {
            self.canvas
                .clip_rect(single_rect.round_out().to_skia(), ClipOp::Intersect, false);
        } else {
            let mut damage_path = PathBuilder::new();
            for rect in damage.rects() {
                damage_path.add_rect(rect.round_out().to_skia(), None, None);
            }
            self.canvas
                .clip_path(&damage_path.detach(), ClipOp::Intersect, false);
        }

        // `canvas.clear` ignores clips, so the background is painted with a draw.
        self.canvas.draw_color(self.background, BlendMode::Src);

        for layer in itertools::sorted(self.tree.layers.keys()) {
            let nodes = self.tree.layers.get(layer).unwrap();
            for node_id in nodes {
                let layout_node = self.tree.layout.get(node_id).unwrap();
                if layout_node.hidden {
                    continue;
                }
                let Some(recording) = paint_state.cache.get(node_id) else {
                    continue;
                };
                let intersects = match recording.bounds {
                    Some(bounds) => damage.intersects(&bounds),
                    None => true,
                };
                if intersects {
                    recording.replay(self.canvas, self.font_collection);
                }
            }
        }

        self.canvas.restore_to_count(checkpoint);
    }

    /// Paint a node and its inherited effects unless it is fully clipped.
    fn paint_node(
        tree: &Tree,
        font_collection: &mut FontCollection,
        scale_factor: f64,
        node_id: &NodeId,
        layout_node: &LayoutNode,
        mut recorder: RenderCanvas<'_>,
    ) -> Option<Recording> {
        let element = tree.elements.get(node_id).unwrap();
        let text_style_state = tree.text_style_state.get(node_id).unwrap();
        let effect_state = tree.effect_state.get(node_id);

        if let Some(effect_state) = effect_state {
            let visible_area = Self::scale_transformed_area(
                tree,
                layout_node.visible_area(),
                &effect_state.scales,
            );

            let mut clipped_away = false;
            hotpath::measure_block!("Element Clipping", {
                for clip_node_id in effect_state.clips.iter() {
                    let clip_element = tree.elements.get(clip_node_id).unwrap();
                    let clip_layout_node = tree.layout.get(clip_node_id).unwrap();
                    let clip_effect = tree.effect_state.get(clip_node_id).unwrap();
                    let transformed_clip_area = Self::scale_transformed_area(
                        tree,
                        clip_layout_node.visible_area(),
                        &clip_effect.scales,
                    );

                    if !visible_area.intersects(&transformed_clip_area) {
                        clipped_away = true;
                        break;
                    }

                    clip_element.clip(ClipContext {
                        recorder: &mut recorder,
                        visible_area: &transformed_clip_area,
                        scale_factor,
                    });
                }
            });
            if clipped_away {
                return None;
            }

            for id in effect_state.rotations.iter() {
                let layout_node = tree.layout.get(id).unwrap();
                let effect = tree.effect_state.get(id).unwrap();
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
                recorder.concat(matrix);
            }

            let render_rect = element.render_rect(&visible_area, scale_factor as f32);
            let mut layer_bounds = *render_rect.rect();
            let scale_factor = scale_factor as f32;

            for shadow in element.style().shadows.iter() {
                if shadow.position == ShadowPosition::Normal {
                    let outset_x = shadow.x.abs() + shadow.spread + shadow.blur;
                    let outset_y = shadow.y.abs() + shadow.spread + shadow.blur;
                    layer_bounds = layer_bounds
                        .with_outset((outset_x * scale_factor, outset_y * scale_factor));
                }
            }

            // Composite backdrop blur before opacity so it samples the content underneath.
            if let Some(blur_radius) = effect_state.blur {
                let style = element.style();
                let blur_sigma = (blur_radius * scale_factor, blur_radius * scale_factor);
                let image_filter = blur(blur_sigma, None, None, render_rect.rect());
                if let Some(image_filter) = image_filter {
                    recorder.save();
                    if style.corner_radius.is_round() {
                        recorder.clip_rrect(render_rect, ClipOp::Intersect, true);
                    }
                    recorder.backdrop_blur(*render_rect.rect(), image_filter, blur_sigma);
                    recorder.restore();
                    recorder.restore();
                }
            }

            let opacity = effect_state
                .opacities
                .iter()
                .fold(1., |acc, opacity| acc * opacity);
            if opacity < 1. {
                recorder.save_layer(layer_bounds, Some(opacity));
            }

            for id in effect_state.scales.iter() {
                let layout_node = tree.layout.get(id).unwrap();
                let effect = tree.effect_state.get(id).unwrap();
                let area = layout_node.visible_area();
                let origin = effect.transform_origin.origin(&area);
                let scale = effect.scale.unwrap();

                recorder.translate(origin.x, origin.y);
                recorder.scale(scale.x, scale.y);
                recorder.translate(-origin.x, -origin.y);
            }
        }

        #[cfg(feature = "incremental-painting")]
        let layout_bounds = match &recorder {
            RenderCanvas::Recording(recorder) => Some(
                recorder
                    .transform()
                    .map_rect(layout_node.visible_area().round_out().to_skia())
                    .0,
            ),
            RenderCanvas::Direct(_) => None,
        };
        hotpath::measure_block!("Element Render", {
            element.render(RenderContext {
                font_collection,
                recorder: &mut recorder,
                layout_node,
                text_style_state,
                scale_factor,
            });
        });

        #[cfg(feature = "incremental-painting")]
        if let RenderCanvas::Recording(recorder) = recorder {
            let mut recording = recorder.finish();
            if recording.bounds.is_some_and(|bounds| bounds.is_empty())
                && let Some(layout_bounds) = layout_bounds
            {
                recording.bounds = Some(layout_bounds.to_torin().round_out());
            }
            return Some(recording);
        }
        None
    }
}
