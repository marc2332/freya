use freya_engine::prelude::*;
use freya_native_core::prelude::NodeImmutable;
use itertools::Itertools;
use torin::geometry::Area;

use crate::{
    dom::*,
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

        // Finds the best Node from where to start measuring
        fdom.layout().find_best_root(&mut dom_adapter);

        // Generate the dirty rect from the invalidated nodes
        // TODO: Maybe move this to torin?
        {
            let layout = fdom.layout();
            let mut buffer = layout.dirty.iter().copied().collect_vec();
            while let Some(node_id) = buffer.pop() {
                if let Some(layout_node) = layout.get(node_id) {
                    let mut dirty_rect = fdom.dirty_rect();
                    let area = layout_node.visible_area();

                    if let Some(dirty_rect) = &mut *dirty_rect {
                        *dirty_rect = dirty_rect.union(&area);
                    } else {
                        *dirty_rect = Some(area)
                    }

                    if let Some(node) = rdom.get(node_id) {
                        buffer.extend(node.child_ids());
                    }
                }
            }
        }

        let root_id = fdom.rdom().root_id();

        // Measure the layout
        fdom.layout()
            .measure(root_id, area, &mut Some(skia_measurer), &mut dom_adapter);
    }
}
