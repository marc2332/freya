use dioxus_native_core::NodeId;
use freya_dom::prelude::FreyaDOM;
use freya_layout::Layers;
use skia_safe::textlayout::FontCollection;
use torin::prelude::Area;

use crate::ViewportsCollection;

/// Render the layout
pub fn process_render<HookOptions>(
    viewports_collection: &ViewportsCollection,
    dom: &FreyaDOM,
    font_collection: &mut FontCollection,
    layers: &Layers,
    hook_options: &mut HookOptions,
    render_hook: impl Fn(
        &FreyaDOM,
        &NodeId,
        &Area,
        &mut FontCollection,
        &ViewportsCollection,
        &mut HookOptions,
    ),
) {
    let mut layers_nums: Vec<&i16> = layers.layers.keys().collect();

    // Order the layers from top to bottom
    layers_nums.sort();

    // Render all the layers from the bottom to the top
    for layer_num in &layers_nums {
        let layer = layers.layers.get(layer_num).unwrap();
        'elements: for node_id in layer {
            let viewports = viewports_collection.get(node_id);
            let layout = dom.layout();
            let areas = layout.get(*node_id);

            if let Some(areas) = areas {
                // Skip elements that are completely out of any their parent's viewport
                if let Some((_, viewports)) = viewports {
                    for viewport_id in viewports {
                        let viewport = viewports_collection.get(viewport_id).unwrap().0;
                        if let Some(viewport) = viewport {
                            if !viewport.intersects(&areas.area) {
                                continue 'elements;
                            }
                        }
                    }
                }

                // Render the element
                render_hook(
                    dom,
                    node_id,
                    &areas.box_area(),
                    font_collection,
                    viewports_collection,
                    hook_options,
                )
            }
        }
    }
}
