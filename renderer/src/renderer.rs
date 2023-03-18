use dioxus_native_core::node::NodeType;
use dioxus_native_core::prelude::ElementNode;
use dioxus_native_core::real_dom::NodeImmutable;
use freya_core::ViewportsCollection;
use freya_layout::RenderData;
use skia_safe::{textlayout::FontCollection, Canvas, ClipOp, Rect};

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
) {
    let node_ref = node.get_node(dom);
    let node_type = &*node_ref.node_type();
    if let NodeType::Element(ElementNode { tag, .. }) = node_type {
        let children = node_ref.children();
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
                render_label(dom, canvas, font_collection, node, children);
            }
            "paragraph" => {
                render_paragraph(dom, canvas, font_collection, node, children);
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
    }
}
