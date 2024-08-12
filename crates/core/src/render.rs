use freya_common::{
    Compositor,
    LayerState,
};
use freya_engine::prelude::{
    Canvas,
    Color,
    IPoint,
    SamplingOptions,
};
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
    canvas: &Canvas,
    compositor: &Compositor,
    mut render_fn: impl FnMut(&FreyaDOM, &NodeId, &LayoutNode, &Torin<NodeId>, &Canvas),
) {
    let layout = fdom.layout();
    let rdom = fdom.rdom();
    let layers = fdom.layers();
    let compositor_dirty_nodes = fdom.compositor_dirty_nodes();

    compositor.run(
        compositor_dirty_nodes,
        canvas,
        |node, try_traverse_children| {
            let node = rdom.get(node);
            if let Some(node) = node {
                let traverse_children = node
                    .node_type()
                    .tag()
                    .map(|tag| !tag.contains_text())
                    .unwrap_or_default();
                let mut affected = if traverse_children && try_traverse_children {
                    node.child_ids()
                } else {
                    Vec::new()
                };

                if !node.node_type().is_visible_element() {
                    if let Some(parent_id) = node.parent_id() {
                        affected.push(parent_id);
                    }
                }
                affected
            } else {
                Vec::new()
            }
        },
        layers,
    );

    canvas.clear(Color::WHITE);

    let mut rendering_layers = compositor.rendering_layers.borrow_mut();

    let layers = layers.layers();

    // Render all the layers from the bottom to the top
    for layer in sorted(rendering_layers.keys().copied().collect::<Vec<i16>>()) {
        let (ref mut layer_surface, state) = rendering_layers.get_mut(&layer).unwrap();
        let layer_canvas = layer_surface.canvas();

        if *state == LayerState::NeedsRender {
            // Clear the this layer canvas
            layer_canvas.clear(Color::TRANSPARENT);

            let nodes = layers.get(&layer).unwrap();
            'elements: for node_id in nodes {
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
                    render_fn(fdom, node_id, layout_node, &layout, layer_canvas);
                }
            }
        }
        layer_surface.draw(canvas, IPoint::new(0, 0), SamplingOptions::default(), None);
    }

    drop(rendering_layers);
    compositor.reset_invalidated_layers();
}
