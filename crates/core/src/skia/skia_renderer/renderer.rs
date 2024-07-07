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
    LayoutNode,
    Torin,
};

use super::{
    image::ImageElement,
    label::LabelElement,
    paragraph::ParagraphElement,
    rect::RectElement,
    svg::SvgElement,
    wireframe,
};
use crate::{
    dom::DioxusNode,
    prelude::DioxusDOM,
};

pub trait ElementRenderer {
    fn clip(
        &self,
        _layout_node: &LayoutNode,
        _node_ref: &DioxusNode,
        _canvas: &Canvas,
        _scale_factor: f32,
    ) {
    }

    #[allow(clippy::too_many_arguments)]
    fn render(
        self: Box<Self>,
        layout_node: &LayoutNode,
        node_ref: &DioxusNode,
        canvas: &Canvas,
        font_collection: &mut FontCollection,
        font_manager: &FontMgr,
        default_fonts: &[String],
        scale_factor: f32,
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
        rdom: &DioxusDOM,
        layout_node: &LayoutNode,
        node_ref: &DioxusNode,
        render_wireframe: bool,
        layout: &Torin<NodeId>,
    ) {
        let area = layout_node.visible_area();
        let node_type = &*node_ref.node_type();
        if let NodeType::Element(ElementNode { tag, .. }) = node_type {
            let get_renderer_by_tag = |tag: &TagName| -> Option<Box<dyn ElementRenderer>> {
                match tag {
                    TagName::Rect => Some(Box::new(RectElement)),
                    TagName::Svg => Some(Box::new(SvgElement)),
                    TagName::Paragraph => Some(Box::new(ParagraphElement)),
                    TagName::Image => Some(Box::new(ImageElement)),
                    TagName::Label => Some(Box::new(LabelElement)),
                    _ => None,
                }
            };

            let Some(element_renderer) = get_renderer_by_tag(tag) else {
                return;
            };

            self.canvas.save();

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
                    self.canvas.concat(matrix);

                    nodes.extend(node_ref.child_ids());
                }
            }

            // Apply inherited opacity effects
            for (opacity, nodes) in self.opacities.iter_mut() {
                if nodes.contains(&node_ref.id()) {
                    self.canvas.save_layer_alpha_f(
                        Rect::new(area.min_x(), area.min_y(), area.max_x(), area.max_y()),
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
                element_renderer.clip(layout_node, node_ref, self.canvas, self.scale_factor);
            }

            for node_id in &node_viewports.viewports {
                let node_ref = rdom.get(*node_id).unwrap();
                let node_type = node_ref.node_type();
                let Some(tag) = node_type.tag() else {
                    continue;
                };
                let layout_node = layout.get(*node_id).unwrap();
                let Some(element_renderer) = get_renderer_by_tag(tag) else {
                    continue;
                };
                element_renderer.clip(layout_node, &node_ref, self.canvas, self.scale_factor);
            }

            element_renderer.render(
                layout_node,
                node_ref,
                self.canvas,
                self.font_collection,
                self.font_manager,
                self.default_fonts,
                self.scale_factor,
            );

            if render_wireframe {
                wireframe::render_wireframe(self.canvas, &area);
            }

            self.canvas.restore();
        }
    }
}
