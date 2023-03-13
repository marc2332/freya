use dioxus_native_core::node::NodeType;
use dioxus_native_core::NodeId;
use freya_core::ViewportsCollection;
use freya_layout::RenderData;
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
    matrixs: &mut Vec<(Matrix, Vec<NodeId>)>,
) {
    if let NodeType::Element { tag, .. } = &node.get_node(dom).node_data.node_type {
        if let Some(rotate_degs) = node.get_node(dom).state.transform.rotate_degs {
            let area = node.get_area();

            let mut matrix = Matrix::new_identity();
            matrix.set_rotate(
                rotate_degs,
                Some(Point {
                    x: area.x + area.width / 2.0,
                    y: area.y + area.height / 2.0,
                }),
            );

            let mut nodes = vec![node.node_id];
            nodes.extend(node.get_children().clone().unwrap_or_default());
            matrixs.push((matrix, nodes));
        }

        canvas.save();

        if let Some((matrix, _)) = matrixs
            .iter()
            .find(|(_, nodes)| nodes.contains(&node.node_id))
        {
            canvas.concat(matrix);
        }

        let children = node.children.as_ref();
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
                render_rect_container(canvas, node, dom);
            }
            "label" => {
                if let Some(children) = children {
                    render_label(dom, canvas, font_collection, node, children);
                }
            }
            "paragraph" => {
                if let Some(children) = children {
                    render_paragraph(dom, canvas, font_collection, node, children);
                }
            }
            "svg" => {
                render_svg(canvas, node, dom);
            }
            "image" => {
                render_image(canvas, node, dom);
            }
            _ => {}
        }

        if render_wireframe {
            crate::wireframe::render_wireframe(canvas, node);
        }

        canvas.restore();
    }
}
