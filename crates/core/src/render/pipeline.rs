use freya_engine::prelude::{
    ClipOp,
    Color,
    FontCollection,
    FontMgr,
    Matrix,
    Paint,
    Point,
    Rect,
    SamplingOptions,
    SaveLayerRec,
    Surface,
};
use freya_native_core::{
    node::{
        ElementNode,
        NodeType,
    },
    real_dom::NodeImmutable,
    tags::TagName,
    NodeId,
};
use itertools::sorted;
use torin::prelude::{
    Area,
    LayoutNode,
    Torin,
};

use super::{
    wireframe_renderer,
    Compositor,
    CompositorCache,
    CompositorDirtyArea,
};
use crate::{
    dom::{
        CompositorDirtyNodes,
        DioxusDOM,
        DioxusNode,
        ImagesCache,
    },
    elements::{
        ElementUtils,
        ElementUtilsResolver,
    },
    layers::Layers,
    states::{
        TransformState,
        ViewportState,
    },
};

/// Runs the full rendering cycle.
pub struct RenderPipeline<'a> {
    pub rdom: &'a DioxusDOM,
    pub layers: &'a Layers,
    pub layout: &'a Torin<NodeId>,
    pub compositor_dirty_nodes: &'a mut CompositorDirtyNodes,
    pub compositor_dirty_area: &'a mut CompositorDirtyArea,
    pub compositor_cache: &'a mut CompositorCache,
    pub surface: &'a mut Surface,
    pub dirty_surface: &'a mut Surface,
    pub compositor: &'a mut Compositor,
    pub font_collection: &'a mut FontCollection,
    pub font_manager: &'a FontMgr,
    pub images_cache: &'a mut ImagesCache,
    pub canvas_area: Area,
    pub background: Color,
    pub scale_factor: f32,
    pub highlighted_node: Option<NodeId>,
    pub default_fonts: &'a [String],
}

impl RenderPipeline<'_> {
    pub fn run(&mut self) {
        let mut dirty_layers = Layers::default();

        // Process what nodes need to be rendered
        let rendering_layers = self.compositor.run(
            self.compositor_dirty_nodes,
            self.compositor_dirty_area,
            self.compositor_cache,
            self.layers,
            &mut dirty_layers,
            self.layout,
            self.rdom,
            self.scale_factor,
        );

        #[cfg(feature = "fade-cached-incremental-areas")]
        {
            // Slowly fade into white non-rerendered areas
            if self.compositor_dirty_area.is_some() {
                use freya_engine::prelude::{
                    Paint,
                    PaintStyle,
                };
                let rect = Rect::new(
                    self.canvas_area.min_x(),
                    self.canvas_area.min_y(),
                    self.canvas_area.max_x(),
                    self.canvas_area.max_y(),
                );
                let mut paint = Paint::default();
                paint.set_color(Color::from_argb(10, 245, 245, 245));
                paint.set_anti_alias(true);
                paint.set_style(PaintStyle::Fill);
                self.dirty_surface.canvas().draw_rect(rect, &paint);
            }
        }

        self.dirty_surface.canvas().save();

        // Round the area out to prevent float pixels issues
        self.compositor_dirty_area.round_out();

        // Clear using the the background only, but only the dirty
        // area in which it will render the intersected nodes again
        if let Some(dirty_area) = self.compositor_dirty_area.take() {
            #[cfg(debug_assertions)]
            tracing::info!("Marked {dirty_area:?} as dirty area");

            self.dirty_surface.canvas().clip_rect(
                Rect::new(
                    dirty_area.min_x(),
                    dirty_area.min_y(),
                    dirty_area.max_x(),
                    dirty_area.max_y(),
                ),
                ClipOp::Intersect,
                false,
            );
            self.dirty_surface.canvas().clear(self.background);
        }

        #[cfg(debug_assertions)]
        // Counter of painted nodes for debugging purposes
        let mut painted = 0;

        // Render the dirty nodes
        for (_, nodes) in sorted(rendering_layers.iter()) {
            'elements: for node_id in sorted(nodes) {
                let node_ref = self.rdom.get(*node_id).unwrap();
                let node_viewports = node_ref.get::<ViewportState>().unwrap();
                let layout_node = self.layout.get(*node_id);

                if let Some(layout_node) = layout_node {
                    // Skip elements that are completely out of any their parent's viewport
                    for viewport_id in &node_viewports.viewports {
                        let viewport = self.layout.get(*viewport_id).unwrap().visible_area();
                        if !viewport.intersects(&layout_node.area) {
                            continue 'elements;
                        }
                    }

                    // Render the element
                    self.render(node_ref, layout_node);

                    #[cfg(debug_assertions)]
                    {
                        painted += 1;
                    }
                }
            }
        }

        if let Some(highlighted_node) = &self.highlighted_node {
            if let Some(layout_node) = self.layout.get(*highlighted_node) {
                wireframe_renderer::render_wireframe(
                    self.dirty_surface.canvas(),
                    &layout_node.visible_area(),
                );
            }
        }

        #[cfg(debug_assertions)]
        {
            if painted > 0 {
                tracing::info!("Painted {painted} nodes");
            }
        }

        // Copy the dirty canvas into the main canvas
        self.dirty_surface.canvas().restore();
        self.surface.canvas().clear(self.background);
        self.dirty_surface.draw(
            self.surface.canvas(),
            (0, 0),
            SamplingOptions::default(),
            None,
        );

        self.compositor_dirty_nodes.clear();
    }

    pub fn render(&mut self, node_ref: DioxusNode, layout_node: &LayoutNode) {
        let dirty_canvas = self.dirty_surface.canvas();
        let node_type = &*node_ref.node_type();
        if let NodeType::Element(ElementNode { tag, .. }) = node_type {
            let Some(element_utils) = tag.utils() else {
                return;
            };

            let initial_layer = dirty_canvas.save();

            let area = layout_node.visible_area();
            let rect = Rect::new(area.min_x(), area.min_y(), area.max_x(), area.max_y());

            let node_transform = &*node_ref.get::<TransformState>().unwrap();
            let node_viewports = node_ref.get::<ViewportState>().unwrap();

            for node_id in &node_viewports.viewports {
                let node_ref = self.rdom.get(*node_id).unwrap();
                let node_type = node_ref.node_type();
                let Some(element_utils) = node_type.tag().and_then(|tag| tag.utils()) else {
                    continue;
                };
                let layout_node = self.layout.get(*node_id).unwrap();
                element_utils.clip(layout_node, &node_ref, dirty_canvas, self.scale_factor);
            }

            // Apply inherited scale effects
            for (id, scale_x, scale_y) in &node_transform.scales {
                let layout_node = self.layout.get(*id).unwrap();
                let area = layout_node.visible_area();
                let center = area.center();
                dirty_canvas.translate((center.x, center.y));
                dirty_canvas.scale((*scale_x, *scale_y));
                dirty_canvas.translate((-center.x, -center.y));
            }

            // Pass rotate effect to children
            for (id, rotate_degs) in &node_transform.rotations {
                let layout_node = self.layout.get(*id).unwrap();
                let area = layout_node.visible_area();
                let mut matrix = Matrix::new_identity();
                matrix.set_rotate(
                    *rotate_degs,
                    Some(Point {
                        x: area.min_x() + area.width() / 2.0,
                        y: area.min_y() + area.height() / 2.0,
                    }),
                );
                dirty_canvas.concat(&matrix);
            }

            // Apply blend mode
            if let Some(blend) = node_transform.blend_mode {
                let mut paint = Paint::default();
                paint.set_blend_mode(blend);

                let layer_rec = SaveLayerRec::default().bounds(&rect).paint(&paint);
                dirty_canvas.save_layer(&layer_rec);
            }

            // Apply inherited opacity effects
            for opacity in &node_transform.opacities {
                dirty_canvas.save_layer_alpha_f(rect, *opacity);
            }

            // Clip the element itself if non-children content can overflow, like an image in case of `image`
            // or text in the case of `label` or `paragraph`
            if *tag == TagName::Paragraph || *tag == TagName::Label || *tag == TagName::Image {
                element_utils.clip(layout_node, &node_ref, dirty_canvas, self.scale_factor);
            }

            element_utils.render(
                layout_node,
                &node_ref,
                dirty_canvas,
                self.font_collection,
                self.font_manager,
                self.default_fonts,
                self.images_cache,
                self.scale_factor,
            );

            dirty_canvas.restore_to_count(initial_layer);
        }
    }
}
