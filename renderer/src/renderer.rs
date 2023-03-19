use dioxus_native_core::node::NodeType;
use dioxus_native_core::prelude::ElementNode;
use dioxus_native_core::real_dom::NodeImmutable;
use dioxus_native_core::NodeId;
use freya_core::ViewportsCollection;
use freya_layout::RenderData;
use freya_node_state::Transform;
use skia_safe::{textlayout::FontCollection, Canvas, ClipOp, Rect};
use skia_safe::{Matrix, Point};

use crate::elements::{
    render_image, render_label, render_paragraph, render_rect_container, render_svg,
};
use crate::DioxusDOM;

/// Render a node into the Skia canvas
pub fn render_skia(
    dom: &DioxusDOM,
    canvas: &mut Canvas,
    node: &RenderData,
    font_collection: &mut FontCollection,
    viewports_collection: &ViewportsCollection,
    render_wireframe: bool,
    matrices: &mut Vec<(Matrix, Vec<NodeId>)>,
) {
    let node_ref = node.get_node(dom);
    let node_type = &*node_ref.node_type();
    if let NodeType::Element(ElementNode { tag, .. }) = node_type {
        canvas.save();

        let node_transform = &*node_ref.get::<Transform>().unwrap();

        if let Some(rotate_degs) = node_transform.rotate_degs {
            let area = node.get_area();

            let mut matrix = Matrix::new_identity();
            matrix.set_rotate(
                rotate_degs,
                Some(Point {
                    x: area.x + area.width / 2.0,
                    y: area.y + area.height / 2.0,
                }),
            );

            matrices.push((matrix, node_ref.child_ids()));

            canvas.concat(&matrix);
        }

        for (matrix, nodes) in matrices.iter_mut() {
            if nodes.contains(&node.node_id) {
                canvas.concat(matrix);

                nodes.extend(node_ref.child_ids());
            }
        }

        let viewports = viewports_collection.get(node.get_id());

        // Clip all elements with their corresponding viewports
        if let Some((_, viewports)) = viewports {
            for viewport_id in viewports {
                let viewport = viewports_collection.get(viewport_id).unwrap().0;
                if let Some(viewport) = viewport {
                    canvas.clip_rect(
                        Rect::new(
                            viewport.x,
                            viewport.y,
                            viewport.x + viewport.width,
                            viewport.y + viewport.height,
                        ),
                        ClipOp::Intersect,
                        true,
                    );
                }
            }
        }

        match tag.as_str() {
            "rect" | "container" => {
                render_rect_container(node, node_ref, canvas);
            }
            "label" => {
                render_label(node, node_ref, canvas, font_collection);
            }
            "paragraph" => {
                render_paragraph(node, node_ref, canvas, font_collection);
            }
            "svg" => {
                render_svg(node, node_ref, canvas);
            }
            "image" => {
                render_image(node, node_ref, canvas);
            }
            _ => {}
        }

        if render_wireframe {
            crate::wireframe::render_wireframe(canvas, node);
        }

        canvas.restore();
    }
}
