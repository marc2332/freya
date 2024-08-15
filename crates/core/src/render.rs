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
    LayoutNode,
    Torin,
};

use crate::dom::FreyaDOM;

pub fn process_render(
    fdom: &FreyaDOM,
    background: Color,
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

    // Process what nodes need to be rendered
    let rendering_layers = compositor.run(
        &mut compositor_dirty_nodes,
        &mut compositor_dirty_area,
        &layers,
        &mut dirty_layers,
        &layout,
    );

    dirty_canvas.save();

    compositor_dirty_area.round_out();

    if let Some(dirty_area) = compositor_dirty_area.take() {
        // Clear using the the background only, but only the dirty
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
        dirty_canvas.clear(background);
    }

    // Render the layers
    for (_, nodes) in sorted(rendering_layers.iter()) {
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
            }
        }
    }

    dirty_canvas.restore();
    canvas.clear(background);
    dirty_surface.draw(canvas, (0, 0), SamplingOptions::default(), None);
}
