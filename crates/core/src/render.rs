use freya_native_core::{
    real_dom::NodeImmutable, tree::TreeRef, NodeId
};
use freya_node_state::ViewportState;
use itertools::sorted;
use torin::prelude::{
    LayoutNode,
    Torin,
};

use crate::dom::FreyaDOM;


/// 1. What elements have changed? Affected children
/// 2. What elements are affected by rerendering (layering)
/// 3. Render those affected elements
/// 
/// Call the render function for the nodes that should be rendered.
pub fn process_render(
    fdom: &FreyaDOM,
    full_render: bool,
    mut render_fn: impl FnMut(&FreyaDOM, &NodeId, &LayoutNode, &Torin<NodeId>),
) {
    let layout = fdom.layout();
    let rdom = fdom.rdom();
    let layers = fdom.layers();
    let multi_layer_renderer = fdom.multi_layer_renderer();
    println!(">{full_render}");
    let rendering_layers = if full_render {
        layers
    } else {
        &multi_layer_renderer.run(|node| {
            let node = rdom.get(node).unwrap();
    
            let traverse_children = node
                .node_type()
                .tag()
                .map(|tag| tag.has_children_with_intrinsic_layout())
                .unwrap_or_default();
            if traverse_children {
                node.child_ids()
            } else {
                Vec::new()
            }
        }, layers, |node| {
            layout.get(node).map(|node| node.area)
        })
    };

    // Render all the layers from the bottom to the top
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
                render_fn(fdom, node_id, layout_node, &layout)
            }
        }
    }
}
