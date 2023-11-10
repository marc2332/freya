use crate::layout::*;
use dioxus_native_core::NodeId;
use freya_dom::prelude::FreyaDOM;
use freya_engine::prelude::*;
use torin::prelude::Area;

/// Render the layout
pub fn process_render<HookOptions>(
    viewports: &Viewports,
    dom: &FreyaDOM,
    font_collection: &mut FontCollection,
    layers: &Layers,
    hook_options: &mut HookOptions,
    render_hook: impl Fn(&FreyaDOM, &NodeId, &Area, &mut FontCollection, &Viewports, &mut HookOptions),
) {
    // Render all the layers from the bottom to the top
    for (_, layer) in layers.layers() {
        'elements: for node_id in layer {
            let node_viewports = viewports.get(node_id);
            let layout = dom.layout();
            let areas = layout.get(*node_id);

            if let Some(areas) = areas {
                // Skip elements that are completely out of any their parent's viewport
                if let Some((_, node_viewports)) = node_viewports {
                    for viewport_id in node_viewports {
                        let viewport = viewports.get(viewport_id).unwrap().0;
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
                    &areas.visible_area(),
                    font_collection,
                    viewports,
                    hook_options,
                )
            }
        }
    }
}
