use freya_engine::prelude::*;
use freya_native_core::{real_dom::NodeImmutable, NodeId};
use freya_node_state::ViewportState;
use itertools::sorted;
use torin::prelude::{LayoutNode, Torin};

use crate::dom::FreyaDOM;

/// Call the render function for the nodes that should be rendered.
pub fn process_render(
    fdom: &FreyaDOM,
    font_collection: &mut FontCollection,
    mut render_fn: impl FnMut(&FreyaDOM, &NodeId, &LayoutNode, &mut FontCollection, &Torin<NodeId>),
) {
    let layout = fdom.layout();
    let rdom = fdom.rdom();
    let layers = fdom.layers();

    // Render all the layers from the bottom to the top
    for (_, layer) in sorted(layers.layers().iter()) {
        'elements: for node_id in layer {
            let node = rdom.get(*node_id).unwrap();
            let node_viewports = node.get::<ViewportState>().unwrap();

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
                render_fn(fdom, node_id, layout_node, font_collection, &layout)
            }
        }
    }
}
