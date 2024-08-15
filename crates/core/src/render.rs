use freya_common::{
    Compositor,
    Layers,
};
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
    Area,
    LayoutNode,
    Torin,
};

use crate::dom::FreyaDOM;

pub fn process_render(
    fdom: &FreyaDOM,
    surface: &mut Surface,
    dirty_surface: &mut Surface,
    compositor: &mut Compositor,
    mut render_fn: impl FnMut(&FreyaDOM, &NodeId, &LayoutNode, &Torin<NodeId>, &Canvas),
) {
    let canvas = surface.canvas();
    let dirty_canvas = dirty_surface.canvas();

    let layout = fdom.layout();
    let rdom = fdom.rdom();
    let layers = fdom.layers();
    let mut compositor_dirty_area = fdom.compositor_dirty_area();
    let mut compositor_dirty_nodes = fdom.compositor_dirty_nodes();

    let mut dirty_layers = Layers::default();

    let rendering_layers = compositor.run(
        &mut compositor_dirty_nodes,
        &mut compositor_dirty_area,
        &layers,
        &mut dirty_layers,
        |node| layout.get(node).map(|node| node.area),
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
    );

    dirty_canvas.save();
    let compositor_dirty_area: &Option<Area> = &compositor_dirty_area;
    if let Some(dirty_area) = compositor_dirty_area {
        // Clear using the configured window background only the dirty
        // area in which it will render the intersected nodes again
        dirty_canvas.clip_rect(
            Rect::new(
                dirty_area.min_x(),
                dirty_area.min_y(),
                dirty_area.max_x(),
                dirty_area.max_y(),
            ),
            ClipOp::Intersect,
            false,
        );
        dirty_canvas.clear(Color::WHITE);
    }

    let mut painted = Vec::new();

    // Render the layers
    for nodes in sorted(rendering_layers.values()) {
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
    canvas.clear(Color::WHITE);
    dirty_surface.draw(canvas, (0, 0), SamplingOptions::default(), None);
}
