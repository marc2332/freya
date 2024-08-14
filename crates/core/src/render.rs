use freya_common::Compositor;
use freya_engine::prelude::{
    Canvas,
    ClipOp,
    Color,
    Rect,
    SamplingOptions,
    Surface,
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
    dirty_surface: &mut Surface,
    compositor: &Compositor,
    mut render_fn: impl FnMut(&FreyaDOM, &NodeId, &LayoutNode, &Torin<NodeId>, &Canvas),
) {
    let dirty_canvas = dirty_surface.canvas();
    let layout = fdom.layout();
    let rdom = fdom.rdom();
    let layers = fdom.layers();
    let compositor_dirty_nodes = fdom.compositor_dirty_nodes();

    let (dirty_layers, dirty_area) = compositor.run(
        compositor_dirty_nodes,
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
        |node| layout.get(node).map(|node| node.area),
        layers,
    );

    let dirty_layers = dirty_layers.layers();

    dirty_canvas.save();
    if let Some(dirty_area) = dirty_area {
        dirty_canvas.clip_rect(
            Rect::new(
                dirty_area.min_x(),
                dirty_area.min_y(),
                dirty_area.max_x(),
                dirty_area.max_y(),
            ),
            Some(ClipOp::Intersect),
            false,
        );
        dirty_canvas.clear(Color::WHITE);
    }

    let mut painted = Vec::new();

    // Render the dirty nodes
    for (_, nodes) in sorted(dirty_layers.iter()) {
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
                render_fn(fdom, node_id, layout_node, &layout, dirty_canvas);
                painted.push(node_id);
            }
        }
    }

    dirty_canvas.restore();
    dirty_surface.draw(canvas, (0, 0), SamplingOptions::default(), None);
}
