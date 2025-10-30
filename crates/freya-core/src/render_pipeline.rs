use freya_engine::prelude::{
    Canvas,
    FontCollection,
    FontMgr,
    SkMatrix,
    SkPoint,
    SkRect,
};

use crate::{
    element::{
        ClipContext,
        RenderContext,
    },
    prelude::Color,
    tree::Tree,
};

pub struct RenderPipeline<'a> {
    pub font_collection: &'a mut FontCollection,
    pub font_manager: &'a FontMgr,
    pub canvas: &'a Canvas,
    pub tree: &'a Tree,
    pub scale_factor: f64,
}

impl RenderPipeline<'_> {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn render(self) {
        self.canvas.clear(Color::WHITE);

        // TODO: Use incremental rendering
        for i16 in itertools::sorted(self.tree.layers.keys()) {
            let nodes = self.tree.layers.get(i16).unwrap();
            'rendering: for node_id in nodes {
                let layer = self.canvas.save();

                let element = self.tree.elements.get(node_id).unwrap();
                let text_style_state = self.tree.text_style_state.get(node_id).unwrap();
                let layout_node = self.tree.layout.get(node_id).unwrap();
                let effect_state = self.tree.effect_state.get(node_id);

                if let Some(effect_state) = effect_state {
                    hotpath::measure_block!("Element Clipping", {
                        for clip_node_id in effect_state.clips.iter() {
                            let clip_element = self.tree.elements.get(clip_node_id).unwrap();
                            let clip_layout_node = self.tree.layout.get(clip_node_id).unwrap();

                            // No need to render this node as its completely clipped
                            if !layout_node.area.intersects(&clip_layout_node.area) {
                                self.canvas.restore_to_count(layer);
                                continue 'rendering;
                            }

                            let clip_context = ClipContext {
                                canvas: self.canvas,
                                layout_node: clip_layout_node,
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
                        let mut matrix = SkMatrix::new_identity();
                        matrix.set_rotate(
                            effect.rotation.unwrap(),
                            Some(SkPoint {
                                x: area.min_x() + area.width() / 2.0,
                                y: area.min_y() + area.height() / 2.0,
                            }),
                        );
                        self.canvas.concat(&matrix);
                    }

                    // Apply inherited scale effects
                    for id in effect_state.scales.iter() {
                        let layout_node = self.tree.layout.get(id).unwrap();
                        let effect = self.tree.effect_state.get(id).unwrap();
                        let area = layout_node.visible_area();
                        let center = area.center();
                        let scale = effect.scale.unwrap();

                        self.canvas.translate((center.x, center.y));
                        self.canvas.scale((scale.x, scale.y));
                        self.canvas.translate((-center.x, -center.y));
                    }

                    // Apply inherited opacity effects
                    let area = layout_node.visible_area();
                    let rect = SkRect::new(area.min_x(), area.min_y(), area.max_x(), area.max_y());
                    for opacity in effect_state.opacities.iter() {
                        self.canvas.save_layer_alpha_f(rect, *opacity);
                    }
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

                self.canvas.restore_to_count(layer);
            }
        }
    }
}
