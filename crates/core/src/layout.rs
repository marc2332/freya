use freya_dom::prelude::{DioxusDOMAdapter, FreyaDOM};
use freya_engine::prelude::*;
use freya_layout::{Layers, SkiaMeasurer};
use torin::geometry::Area;

use crate::{layers::process_layers, viewports::calculate_viewports, ViewportsCollection};

/// Process the layout of the DOM
pub fn process_layout(
    fdom: &FreyaDOM,
    area: Area,
    font_collection: &mut FontCollection,
    scale_factor: f32,
) -> (Layers, ViewportsCollection) {
    let rdom = fdom.rdom();
    let dom_adapter = DioxusDOMAdapter::new(rdom);
    let skia_measurer = SkiaMeasurer::new(rdom, font_collection);

    // Finds the best Node from where to start measuring
    fdom.layout().find_best_root(&dom_adapter);

    let root_id = fdom.rdom().root_id();

    // Measure the layout
    fdom.layout()
        .measure(root_id, area, &mut Some(skia_measurer), &dom_adapter);

    // Create the layers
    let mut layers = Layers::default();
    process_layers(
        &mut layers,
        rdom,
        &fdom.layout(),
        font_collection,
        scale_factor,
    );

    let mut layers_nums: Vec<&i16> = layers.layers.keys().collect();

    // Order the layers from top to bottom
    layers_nums.sort();

    // Calculate the viewports
    let viewports_collection = calculate_viewports(&layers_nums, &layers, fdom);

    (layers, viewports_collection)
}
