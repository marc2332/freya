use dioxus_native_core::node::NodeType;
use dioxus_native_core::NodeId;
use freya_core::ViewportsCollection;
use freya_dom::{DioxusNode, FreyaDOM};
use freya_layout::RenderData;
use skia_safe::{textlayout::FontCollection, Canvas, ClipOp, Rect};
use skia_safe::{Matrix, Point};

use crate::elements::{
    render_image, render_label, render_paragraph, render_rect_container, render_svg,
};

/// Render a node into the Skia canvas
#[allow(clippy::too_many_arguments)]
pub fn render_skia(
    dom: &FreyaDOM,
    canvas: &mut Canvas,
    render_node: &RenderData,
    dioxus_node: &DioxusNode,
    font_collection: &mut FontCollection,
    viewports_collection: &ViewportsCollection,
    render_wireframe: bool,
    matrices: &mut Vec<(Matrix, Vec<NodeId>)>,
) {
    if let NodeType::Element { tag, .. } = &dioxus_node.node_data.node_type {
        canvas.save();

        if let Some(rotate_degs) = dioxus_node.state.transform.rotate_degs {
            let area = render_node.get_area();

            let mut matrix = Matrix::new_identity();
            matrix.set_rotate(
                rotate_degs,
                Some(Point {
                    x: area.x + area.width / 2.0,
                    y: area.y + area.height / 2.0,
                }),
            );

            if let Some(children) = render_node.get_children() {
                matrices.push((matrix, children.clone()));
            }

            canvas.concat(&matrix);
        }

        for (matrix, nodes) in matrices.iter_mut() {
            if nodes.contains(&render_node.node_id) {
                canvas.concat(matrix);

                if let Some(children) = render_node.get_children() {
                    nodes.extend(children)
                }
            }
        }

        let viewports = viewports_collection.get(render_node.get_id());

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
                render_rect_container(canvas, render_node, dioxus_node);
            }
            "label" => {
                render_label(render_node, dioxus_node, dom, canvas, font_collection);
            }
            "paragraph" => {
                render_paragraph(render_node, dioxus_node, font_collection, dom, canvas);
            }
            "svg" => {
                render_svg(render_node, dioxus_node, canvas);
            }
            "image" => {
                render_image(render_node, dioxus_node, canvas);
            }
            _ => {}
        }

        if render_wireframe {
            crate::wireframe::render_wireframe(canvas, render_node);
        }

        canvas.restore();
    }
}
