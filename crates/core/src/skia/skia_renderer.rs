use freya_engine::prelude::*;
use freya_native_core::{
    node::NodeType,
    prelude::ElementNode,
    real_dom::NodeImmutable,
    tags::TagName,
    NodeId,
};
use freya_node_state::{
    StyleState,
    TransformState,
    ViewportState,
};
use torin::prelude::{
    Area,
    LayoutNode,
    Torin,
};

use super::wireframe_renderer;
use crate::{
    dom::DioxusNode,
    elements::{
        ElementUtils,
        ElementUtilsResolver,
    },
    prelude::DioxusDOM,
};

pub struct SkiaRenderer<'a> {
    pub canvas_area: Area,
    pub font_collection: &'a mut FontCollection,
    pub font_manager: &'a FontMgr,
    pub matrices: Vec<(Matrix, Vec<NodeId>)>,
    pub opacities: Vec<(f32, Vec<NodeId>)>,
    pub default_fonts: &'a [String],
    pub scale_factor: f32,
}

impl SkiaRenderer<'_> {
    /// Render a node into the Skia canvas
    pub fn render(
        &mut self,
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
            let node_style = &*node_ref.get::<StyleState>().unwrap();

            // Pass rotate effect to children
            if let Some(rotate_degs) = node_transform.rotate_degs {
                let mut matrix = Matrix::new_identity();
                matrix.set_rotate(
                    rotate_degs,
                    Some(Point {
                        x: area.min_x() + area.width() / 2.0,
                        y: area.min_y() + area.height() / 2.0,
                    }),
                );

                self.matrices.push((matrix, vec![node_ref.id()]));
            }

            // Pass opacity effect to children
            if let Some(opacity) = node_style.opacity {
                self.opacities.push((opacity, vec![node_ref.id()]));
            }

            // Apply inherited matrices
            for (matrix, nodes) in self.matrices.iter_mut() {
                if nodes.contains(&node_ref.id()) {
                    canvas.concat(matrix);

                    nodes.extend(node_ref.child_ids());
                }
            }

            // Apply inherited opacity effects
            for (opacity, nodes) in self.opacities.iter_mut() {
                if nodes.contains(&node_ref.id()) {
                    canvas.save_layer_alpha_f(
                        Rect::new(
                            self.canvas_area.min_x(),
                            self.canvas_area.min_y(),
                            self.canvas_area.max_x(),
                            self.canvas_area.max_y(),
                        ),
                        *opacity,
                    );

                    nodes.extend(node_ref.child_ids());
                }
            }

            // Clip all elements with their corresponding viewports
            let node_viewports = node_ref.get::<ViewportState>().unwrap();
            // Only clip the element iself when it's paragraph because
            // it will render the inner text spans on it's own, so if these spans overflow the paragraph,
            // It is the paragraph job to make sure they are clipped
            if !node_viewports.viewports.is_empty() && *tag == TagName::Paragraph {
                element_utils.clip(layout_node, node_ref, canvas, self.scale_factor);
            }

            for node_id in &node_viewports.viewports {
                let node_ref = rdom.get(*node_id).unwrap();
                let node_type = node_ref.node_type();
                let Some(element_utils) = node_type.tag().and_then(|tag| tag.utils()) else {
                    continue;
                };
                let layout_node = layout.get(*node_id).unwrap();
                element_utils.clip(layout_node, &node_ref, canvas, self.scale_factor);
            }

            element_utils.render(
                layout_node,
                node_ref,
                canvas,
                self.font_collection,
                self.font_manager,
                self.default_fonts,
                self.scale_factor,
            );

            if render_wireframe {
                wireframe_renderer::render_wireframe(canvas, &area);
            }

            canvas.restore_to_count(initial_layer);
        }
    }
}
