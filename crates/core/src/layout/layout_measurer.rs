use crate::layout::*;
use freya_dom::prelude::{DioxusDOMAdapter, FreyaDOM};
use freya_engine::prelude::*;
use tokio::time::Instant;
use torin::geometry::Area;

/// Process the layout of the DOM
pub fn process_layout(
    fdom: &FreyaDOM,
    area: Area,
    font_collection: &mut FontCollection,
    scale_factor: f32,
) -> Layers {
    let rdom = fdom.rdom();
    let mut dom_adapter = DioxusDOMAdapter::new_with_cache(rdom);
    let skia_measurer = SkiaMeasurer::new(rdom, font_collection);

    let inst = Instant::now();
    // Finds the best Node from where to start measuring
    fdom.layout().find_best_root(&mut dom_adapter);
    println!("root {}", inst.elapsed().as_millis());

    let root_id = fdom.rdom().root_id();

    let inst = Instant::now();
    // Measure the layout
    fdom.layout()
        .measure(root_id, area, &mut Some(skia_measurer), &mut dom_adapter);
    println!("layout {}", inst.elapsed().as_millis());

    let inst = Instant::now();
    // Create the layers
    let layers = Layers::new(rdom, &fdom.layout(), scale_factor);
    println!("layers {}", inst.elapsed().as_millis());

    layers
}
