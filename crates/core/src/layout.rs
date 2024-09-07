use freya_engine::prelude::*;
use freya_native_core::prelude::NodeImmutable;
use itertools::Itertools;
use torin::geometry::Area;

use crate::{
    dom::*,
    prelude::NodeAccessibility,
    skia::SkiaMeasurer,
};

/// Process the layout of the DOM
pub fn process_layout(
    fdom: &FreyaDOM,
    area: Area,
    font_collection: &mut FontCollection,
    scale_factor: f64,
    default_fonts: &[String],
) {
    {
        let rdom = fdom.rdom();
        let mut dom_adapter = DioxusDOMAdapter::new(rdom, scale_factor as f32);
        let skia_measurer =
            SkiaMeasurer::new(rdom, font_collection, default_fonts, scale_factor as f32);

        let mut layout = fdom.layout();

        // Finds the best Node from where to start measuring
        layout.find_best_root(&mut dom_adapter);

        // Invalidate those accessible elements whose layout has been affected
        let mut dirty_accessibility_tree = fdom.dirty_accessibility_tree();
        let mut buffer = layout.dirty.iter().copied().collect_vec();
        while let Some(node_id) = buffer.pop() {
            if let Some(node) = rdom.get(node_id) {
                if node.get_accessibility_id().is_some() {
                    dirty_accessibility_tree.add_or_update(node_id);
                }
                buffer.extend(node.child_ids());
            }
        }

        let root_id = fdom.rdom().root_id();

        // Measure the layout
        layout.measure(root_id, area, &mut Some(skia_measurer), &mut dom_adapter);
    }
}
