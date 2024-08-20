use freya_common::Layers;
use freya_engine::prelude::{
    Canvas,
    ClipOp,
    Color,
    FontCollection,
    FontMgr,
    Matrix,
    Point,
    Rect,
    SamplingOptions,
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
use freya_node_state::{
    TransformState,
    ViewportState,
};
use itertools::sorted;
use torin::prelude::{
    Area,
    LayoutNode,
    Torin,
};

use super::wireframe_renderer;
use crate::{
    dom::{
        DioxusDOM,
        DioxusNode,
        FreyaDOM,
    },
    prelude::{
        Compositor,
        ElementUtils,
        ElementUtilsResolver,
    },
};

pub struct RenderPipeline<'a> {
    pub canvas_area: Area,
    pub fdom: &'a FreyaDOM,
    pub background: Color,
    pub surface: &'a mut Surface,
    pub dirty_surface: &'a mut Surface,
    pub compositor: &'a mut Compositor,
    pub scale_factor: f32,
    pub selected_node: Option<NodeId>,
    pub font_collection: &'a mut FontCollection,
    pub font_manager: &'a FontMgr,
    pub default_fonts: &'a [String],
}

impl RenderPipeline<'_> {
    pub fn run(&mut self) {
        let canvas = self.surface.canvas();
        let dirty_canvas = self.dirty_surface.canvas();

        let layout = self.fdom.layout();
        let rdom = self.fdom.rdom();
        let layers = self.fdom.layers();
        let mut compositor_dirty_area = self.fdom.compositor_dirty_area();
        let mut compositor_dirty_nodes = self.fdom.compositor_dirty_nodes();
        let mut compositor_cache = self.fdom.compositor_cache();

        let mut dirty_layers = Layers::default();

        // Process what nodes need to be rendered
        let rendering_layers = self.compositor.run(
            &mut compositor_dirty_nodes,
            &mut compositor_dirty_area,
            &mut compositor_cache,
            &layers,
            &mut dirty_layers,
            &layout,
            rdom,
            self.scale_factor,
        );

        dirty_canvas.save();

        compositor_dirty_area.round_out();

        if let Some(dirty_area) = compositor_dirty_area.take() {
            #[cfg(debug_assertions)]
            tracing::info!("Marked {dirty_area:?} as dirty area");

            // Clear using the the background only, but only the dirty
            // area in which it will render the intersected nodes again
            dirty_canvas.clip_rect(
                Rect::new(
                    dirty_area.min_x(),
                    dirty_area.min_y(),
                    dirty_area.max_x(),
                    dirty_area.max_y(),
                ),
                ClipOp::Intersect,
                false,
            );
            dirty_canvas.clear(self.background);
        }

        #[cfg(debug_assertions)]
        let mut painted = 0;

        // Render the layers
        for (_, nodes) in sorted(rendering_layers.iter()) {
            'elements: for node_id in nodes {
                let node_ref = rdom.get(*node_id).unwrap();
                let node_viewports = node_ref.get::<ViewportState>().unwrap();

                let layout_node = layout.get(*node_id);

                if let Some(layout_node) = layout_node {
                    // Skip elements that are completely out of any their parent's viewport
                    for viewport_id in &node_viewports.viewports {
                        let viewport = layout.get(*viewport_id).unwrap().visible_area();
                        if !viewport.intersects(&layout_node.area) {
                            continue 'elements;
                        }
                    }

                    // Render the element
                    Self::render(
                        self.canvas_area,
                        self.font_collection,
                        self.font_manager,
                        self.default_fonts,
                        self.scale_factor,
                        rdom,
                        layout_node,
                        &node_ref,
                        Some(node_id) == self.selected_node.as_ref(),
                        &layout,
                        canvas,
                    );

                    #[cfg(debug_assertions)]
                    {
                        painted += 1;
                    }
                }
            }
        }

        #[cfg(debug_assertions)]
        {
            if painted > 0 {
                tracing::info!("Painted {painted} nodes");
            }
        }

        dirty_canvas.restore();
        canvas.clear(self.background);
        self.dirty_surface
            .draw(canvas, (0, 0), SamplingOptions::default(), None);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn render(
        canvas_area: Area,
        font_collection: &mut FontCollection,
        font_manager: &FontMgr,
        default_fonts: &[String],
        scale_factor: f32,
        rdom: &DioxusDOM,
        layout_node: &LayoutNode,
        node_ref: &DioxusNode,
        render_wireframe: bool,
        layout: &Torin<NodeId>,
        canvas: &Canvas,
    ) {
        let area = layout_node.visible_area();
        let node_type = &*node_ref.node_type();
        if let NodeType::Element(ElementNode { tag, .. }) = node_type {
            let Some(element_utils) = tag.utils() else {
                return;
            };

            let initial_layer = canvas.save();
            let node_transform = &*node_ref.get::<TransformState>().unwrap();

            // Pass rotate effect to children
            for (id, rotate_degs) in &node_transform.rotations {
                let layout_node = layout.get(*id).unwrap();
                let area = layout_node.visible_area();
                let mut matrix = Matrix::new_identity();
                matrix.set_rotate(
                    *rotate_degs,
                    Some(Point {
                        x: area.min_x() + area.width() / 2.0,
                        y: area.min_y() + area.height() / 2.0,
                    }),
                );
                canvas.concat(&matrix);
            }

            // Apply inherited opacity effects
            for opacity in &node_transform.opacities {
                canvas.save_layer_alpha_f(
                    Rect::new(
                        canvas_area.min_x(),
                        canvas_area.min_y(),
                        canvas_area.max_x(),
                        canvas_area.max_y(),
                    ),
                    *opacity,
                );
            }

            // Clip all elements with their corresponding viewports
            let node_viewports = node_ref.get::<ViewportState>().unwrap();
            // Only clip the element iself when it's paragraph because
            // it will render the inner text spans on it's own, so if these spans overflow the paragraph,
            // It is the paragraph job to make sure they are clipped
            if !node_viewports.viewports.is_empty() && *tag == TagName::Paragraph {
                element_utils.clip(layout_node, node_ref, canvas, scale_factor);
            }

            for node_id in &node_viewports.viewports {
                let node_ref = rdom.get(*node_id).unwrap();
                let node_type = node_ref.node_type();
                let Some(element_utils) = node_type.tag().and_then(|tag| tag.utils()) else {
                    continue;
                };
                let layout_node = layout.get(*node_id).unwrap();
                element_utils.clip(layout_node, &node_ref, canvas, scale_factor);
            }

            element_utils.render(
                layout_node,
                node_ref,
                canvas,
                font_collection,
                font_manager,
                default_fonts,
                scale_factor,
            );

            if render_wireframe {
                wireframe_renderer::render_wireframe(canvas, &area);
            }

            canvas.restore_to_count(initial_layer);
        }
    }
}
