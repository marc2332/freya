use freya_native_core::{
    real_dom::NodeImmutable,
    NodeId,
};
use freya_node_state::ViewportState;
use itertools::sorted;
use torin::prelude::{
    LayoutNode,
    Torin,
};

use crate::dom::FreyaDOM;

pub fn process_render(
    fdom: &FreyaDOM,
    full_render: bool,
    mut render_fn: impl FnMut(&FreyaDOM, &NodeId, &LayoutNode, &Torin<NodeId>),
) {
    let layout = fdom.layout();
    let rdom = fdom.rdom();
    let layers = fdom.layers();
    let multi_layer_renderer = fdom.multi_layer_renderer();

    let rendering_layers = if full_render {
        layers
    } else {
        &multi_layer_renderer.run(
            |node| {
                let node = rdom.get(node).unwrap();

                let traverse_children = node
                    .node_type()
                    .tag()
                    .map(|tag| !tag.contains_text())
                    .unwrap_or_default();
                if traverse_children {
                    node.child_ids()
                } else {
                    Vec::new()
                }
            },
            layers,
            |node| layout.get(node).map(|node| node.area),
        )
    };

    let mut total = 0;
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

                total += 1;
            }
        }
    }

    // Render all the layers from the bottom to the top
    let mut painted = 0;
    for (_, layer) in sorted(rendering_layers.layers().iter()) {
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
                render_fn(fdom, node_id, layout_node, &layout);
                painted += 1;
            }
        }
    }

    println!("PAINTED -> {painted}/{}", total)
}
