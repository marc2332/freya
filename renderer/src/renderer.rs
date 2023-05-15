use dioxus_native_core::node::NodeType;
use dioxus_native_core::prelude::ElementNode;
use dioxus_native_core::real_dom::NodeImmutable;
use dioxus_native_core::NodeId;
use freya_core::ViewportsCollection;
use freya_dom::DioxusNode;
use freya_layout::RenderData;
use freya_node_state::Transform;
use skia_safe::{textlayout::FontCollection, Canvas, ClipOp, Rect};
use skia_safe::{Matrix, Point};

use crate::elements::{
    render_image, render_label, render_paragraph, render_rect_container, render_svg,
};

/// Render a node into the Skia canvas
#[allow(clippy::too_many_arguments)]
pub fn render_skia(
    canvas: &mut Canvas,
    render_node: &RenderData,
    dioxus_node: &DioxusNode,
    font_collection: &mut FontCollection,
    viewports_collection: &ViewportsCollection,
    render_wireframe: bool,
    matrices: &mut Vec<(Matrix, Vec<NodeId>)>,
) {
    let node_type = &*dioxus_node.node_type();
    if let NodeType::Element(ElementNode { tag, .. }) = node_type {
        canvas.save();

        let node_transform = &*dioxus_node.get::<Transform>().unwrap();

        if let Some(rotate_degs) = node_transform.rotate_degs {
            let area = render_node.get_area();

            let mut matrix = Matrix::new_identity();
            matrix.set_rotate(
                rotate_degs,
                Some(Point {
                    x: area.min_x() + area.width() / 2.0,
                    y: area.min_y() + area.height() / 2.0,
                }),
            );

            matrices.push((matrix, dioxus_node.child_ids()));

            canvas.concat(&matrix);
        }

        for (matrix, nodes) in matrices.iter_mut() {
            if nodes.contains(&render_node.node_id) {
                canvas.concat(matrix);

                nodes.extend(dioxus_node.child_ids());
            }
        }

        let viewports = viewports_collection.get(&dioxus_node.id());

        // Clip all elements with their corresponding viewports
        if let Some((_, viewports)) = viewports {
            for viewport_id in viewports {
                let viewport = viewports_collection.get(viewport_id).unwrap().0;
                if let Some(viewport) = viewport {
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
            }
        }

        match tag.as_str() {
            "rect" | "container" => {
                render_rect_container(render_node, dioxus_node, canvas, font_collection);
            }
            "label" => {
                render_label(render_node, dioxus_node, canvas, font_collection);
            }
            "paragraph" => {
                render_paragraph(render_node, dioxus_node, canvas, font_collection);
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
