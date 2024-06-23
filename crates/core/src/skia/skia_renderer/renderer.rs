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
use torin::{
    geometry::Area,
    prelude::{
        LayoutNode,
        Torin,
    },
};

use super::{
    image::render_image,
    label::render_label,
    paragraph::render_paragraph,
    rect::render_rect,
    svg::render_svg,
    wireframe,
};
use crate::dom::DioxusNode;

fn clip_viewport(canvas: &Canvas, viewport: &Area) {
    canvas.clip_rect(
        Rect::new(
            viewport.min_x(),
            viewport.min_y(),
            viewport.max_x(),
            viewport.max_y(),
        ),
        ClipOp::Intersect,
        true,
    );
}

pub struct SkiaRenderer<'a> {
    pub canvas: &'a Canvas,
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
        layout_node: &LayoutNode,
        dioxus_node: &DioxusNode,
        render_wireframe: bool,
        layout: &Torin<NodeId>,
    ) {
        let area = layout_node.visible_area();
        let data = &layout_node.data;
        let node_type = &*dioxus_node.node_type();
        if let NodeType::Element(ElementNode { tag, .. }) = node_type {
            self.canvas.save();

            let node_transform = &*dioxus_node.get::<TransformState>().unwrap();
            let node_style = &*dioxus_node.get::<StyleState>().unwrap();

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

                self.matrices.push((matrix, vec![dioxus_node.id()]));
            }

            // Pass opacity effect to children
            if let Some(opacity) = node_style.opacity {
                self.opacities.push((opacity, vec![dioxus_node.id()]));
            }

            // Apply inherited matrices
            for (matrix, nodes) in self.matrices.iter_mut() {
                if nodes.contains(&dioxus_node.id()) {
                    self.canvas.concat(matrix);

                    nodes.extend(dioxus_node.child_ids());
                }
            }

            // Apply inherited opacity effects
            for (opacity, nodes) in self.opacities.iter_mut() {
                if nodes.contains(&dioxus_node.id()) {
                    self.canvas.save_layer_alpha_f(
                        Rect::new(area.min_x(), area.min_y(), area.max_x(), area.max_y()),
                        *opacity,
                    );

                    nodes.extend(dioxus_node.child_ids());
                }
            }

            // Clip all elements with their corresponding viewports
            let node_viewports = dioxus_node.get::<ViewportState>().unwrap();
            // Only clip the element iself when it's paragraph because
            // it will render the inner text spans on it's own, so if these spans overflow the paragraph,
            // It is the paragraph job to make sure they are clipped
            if !node_viewports.viewports.is_empty() && *tag == TagName::Paragraph {
                clip_viewport(self.canvas, &layout_node.visible_area());
            }

            for viewport_id in &node_viewports.viewports {
                let viewport = layout.get(*viewport_id).unwrap().visible_area();
                clip_viewport(self.canvas, &viewport);
            }

            match tag {
                TagName::Rect => {
                    render_rect(
                        &area,
                        dioxus_node,
                        self.canvas,
                        self.font_collection,
                        self.scale_factor,
                    );
                }
                TagName::Label => {
                    render_label(&area, data, dioxus_node, self.canvas);
                }
                TagName::Paragraph => {
                    render_paragraph(
                        &area,
                        data,
                        dioxus_node,
                        self.canvas,
                        self.font_collection,
                        self.default_fonts,
                        self.scale_factor,
                    );
                }
                TagName::Svg => {
                    render_svg(&area, dioxus_node, self.canvas, self.font_manager);
                }
                TagName::Image => {
                    render_image(&area, dioxus_node, self.canvas);
                }
                _ => {}
            }

            if render_wireframe {
                wireframe::render_wireframe(self.canvas, &area);
            }

            self.canvas.restore();
        }
    }
}
