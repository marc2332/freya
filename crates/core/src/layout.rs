use freya_engine::prelude::*;
use freya_native_core::prelude::{
    NodeId,
    NodeImmutable,
};
use itertools::Itertools;
use torin::{
    geometry::Area,
    torin::Torin,
};

use crate::{
    accessibility::{
        AccessibilityDirtyNodes,
        NodeAccessibility,
    },
    dom::*,
    render::{
        Compositor,
        CompositorDirtyArea,
        SkiaMeasurer,
    },
};

/// Process the layout of the DOM
#[allow(clippy::too_many_arguments)]
pub fn process_layout(
    rdom: &DioxusDOM,
    layout: &mut Torin<NodeId>,
    images_cache: &mut ImagesCache,
    dirty_accessibility_tree: &mut AccessibilityDirtyNodes,
    compositor_dirty_nodes: &mut CompositorDirtyNodes,
    compositor_dirty_area: &mut CompositorDirtyArea,
    area: Area,
    font_collection: &mut FontCollection,
    scale_factor: f32,
    fallback_fonts: &[String],
) {
    let mut dom_adapter = DioxusDOMAdapter::new(rdom, scale_factor);
    let skia_measurer = SkiaMeasurer::new(
        rdom,
        font_collection,
        fallback_fonts,
        scale_factor,
        images_cache,
    );

    // Finds the best Node from where to start measuring
    layout.find_best_root(&mut dom_adapter);

    let mut buffer = layout.dirty.keys().copied().collect_vec();
    while let Some(node_id) = buffer.pop() {
        if let Some(node) = rdom.get(node_id) {
            if let Some(area) = Compositor::get_drawing_area(node_id, layout, rdom, scale_factor) {
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
    let root_id = rdom.root_id();

    // Measure the layout
    layout.measure(root_id, area, &mut Some(skia_measurer), &mut dom_adapter);
}
