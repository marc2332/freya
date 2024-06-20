use freya_engine::prelude::*;
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

        let root_id = fdom.rdom().root_id();

        // Measure the layout
        fdom.layout()
            .measure(root_id, area, &mut Some(skia_measurer), &mut dom_adapter);
    }
}
