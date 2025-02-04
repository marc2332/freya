use freya_engine::prelude::*;
use freya_native_core::prelude::NodeImmutable;
use itertools::Itertools;
use torin::geometry::Area;

use crate::{
    accessibility::NodeAccessibility,
    dom::*,
    render::{
        Compositor,
        SkiaMeasurer,
    },
};

/// Process the layout of the DOM
pub fn process_layout(
    fdom: &FreyaDOM,
    area: Area,
    font_collection: &mut FontCollection,
    scale_factor: f32,
    default_fonts: &[String],
) {
    {
        let rdom = fdom.rdom();
        let mut images_cache = fdom.images_cache();
        let mut dom_adapter = DioxusDOMAdapter::new(rdom, scale_factor);
        let skia_measurer = SkiaMeasurer::new(
            rdom,
            font_collection,
            default_fonts,
            scale_factor,
            &mut images_cache,
        );

        let mut layout = fdom.layout();

        // Finds the best Node from where to start measuring
        layout.find_best_root(&mut dom_adapter);

        let mut dirty_accessibility_tree = fdom.accessibility_dirty_nodes();
        let mut compositor_dirty_nodes = fdom.compositor_dirty_nodes();
        let mut compositor_dirty_area = fdom.compositor_dirty_area();
        let mut buffer = layout.dirty.keys().copied().collect_vec();
        while let Some(node_id) = buffer.pop() {
            if let Some(node) = rdom.get(node_id) {
                if let Some(area) =
                    Compositor::get_drawing_area(node_id, &layout, rdom, scale_factor)
                {
                    // Unite the invalidated area with the dirty area
                    compositor_dirty_area.unite_or_insert(&area);

                    // Mark these elements as dirty for the compositor
                    compositor_dirty_nodes.insert(node_id);

                    // Mark as invalidated this node as its layout has changed
                    if node.get_accessibility_id().is_some() {
                        dirty_accessibility_tree.add_or_update(node_id);
                    }
                }
                // Continue iterating in the children of this node
                buffer.extend(node.child_ids());
            }
        }
        let root_id = fdom.rdom().root_id();

        // Measure the layout
        layout.measure(root_id, area, &mut Some(skia_measurer), &mut dom_adapter);
    }
}
