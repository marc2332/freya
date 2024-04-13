use freya_native_core::prelude::ElementNode;
use freya_native_core::real_dom::NodeImmutable;
use freya_native_core::NodeId;
use freya_native_core::{node::NodeType, tags::TagName};

use freya_core::dom::DioxusNode;
use freya_engine::prelude::*;
use freya_node_state::{Style, Transform, ViewportState};
use torin::{
    geometry::Area,
    prelude::{LayoutNode, Torin},
};

use crate::elements::{render_image, render_label, render_paragraph, render_rect, render_svg};

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

/// Render a node into the Skia canvas
#[allow(clippy::too_many_arguments)]
pub fn render_skia(
    canvas: &Canvas,
    layout_node: &LayoutNode,
    dioxus_node: &DioxusNode,
    font_collection: &mut FontCollection,
    font_manager: &FontMgr,
    render_wireframe: bool,
    matrices: &mut Vec<(Matrix, Vec<NodeId>)>,
    opacities: &mut Vec<(f32, Vec<NodeId>)>,
    default_fonts: &[String],
    layout: &Torin<NodeId>,
) {
    let area = layout_node.visible_area();
    let data = &layout_node.data;
    let node_type = &*dioxus_node.node_type();
    if let NodeType::Element(ElementNode { tag, .. }) = node_type {
        canvas.save();

        let node_transform = &*dioxus_node.get::<Transform>().unwrap();
        let node_style = &*dioxus_node.get::<Style>().unwrap();

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

            matrices.push((matrix, vec![dioxus_node.id()]));
        }

        // Pass opacity effect to children
        if let Some(opacity) = node_style.opacity {
            opacities.push((opacity, vec![dioxus_node.id()]));
        }

        // Apply inherited matrices
        for (matrix, nodes) in matrices.iter_mut() {
            if nodes.contains(&dioxus_node.id()) {
                canvas.concat(matrix);

                nodes.extend(dioxus_node.child_ids());
            }
        }

        // Apply inherited opacity effects
        for (opacity, nodes) in opacities.iter_mut() {
            if nodes.contains(&dioxus_node.id()) {
                canvas.save_layer_alpha_f(
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
            clip_viewport(canvas, &layout_node.visible_area());
        }

        for viewport_id in &node_viewports.viewports {
            let viewport = layout.get(*viewport_id).unwrap().visible_area();
            clip_viewport(canvas, &viewport);
        }

        match tag {
            TagName::Rect => {
                render_rect(&area, dioxus_node, canvas, font_collection);
            }
            TagName::Label => {
                render_label(&area, data, canvas);
            }
            TagName::Paragraph => {
                render_paragraph(
                    &area,
                    data,
                    dioxus_node,
                    canvas,
                    font_collection,
                    default_fonts,
                );
            }
            TagName::Svg => {
                render_svg(&area, dioxus_node, canvas, font_manager);
            }
            TagName::Image => {
                render_image(&area, dioxus_node, canvas);
            }
            _ => {}
        }

        if render_wireframe {
            crate::wireframe::render_wireframe(canvas, &area);
        }

        canvas.restore();
    }
}
