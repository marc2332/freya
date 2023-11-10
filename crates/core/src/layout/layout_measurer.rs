use crate::layout::*;
use freya_dom::prelude::{DioxusDOMAdapter, FreyaDOM};
use freya_engine::prelude::*;
use torin::geometry::Area;

/// Process the layout of the DOM
pub fn process_layout(
    fdom: &FreyaDOM,
    area: Area,
    font_collection: &mut FontCollection,
    scale_factor: f32,
) -> (Layers, Viewports) {
    let rdom = fdom.rdom();
    let mut dom_adapter = DioxusDOMAdapter::new_with_cache(rdom);
    let skia_measurer = SkiaMeasurer::new(rdom, font_collection);

    // Finds the best Node from where to start measuring
    fdom.layout().find_best_root(&mut dom_adapter);

    let root_id = fdom.rdom().root_id();

    // Measure the layout
    fdom.layout()
        .measure(root_id, area, &mut Some(skia_measurer), &mut dom_adapter);

    // Create the layers
    let layers = Layers::new(rdom, &fdom.layout(), font_collection, scale_factor);

    let mut layers_nums = layers.layers_indices();

    // Order the layers from top to bottom
    layers_nums.sort();

    // Calculate the viewports
    let viewports = Viewports::new(&layers_nums, &layers, fdom);

    (layers, viewports)
}
