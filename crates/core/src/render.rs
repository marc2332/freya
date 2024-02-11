use crate::layout::*;
use dioxus_native_core::NodeId;
use freya_dom::prelude::FreyaDOM;
use freya_engine::prelude::*;
use torin::prelude::Area;

/// Call the render function for the nodes that should be rendered.
pub fn process_render<RenderOptions>(
    viewports: &Viewports,
    fdom: &FreyaDOM,
    font_collection: &mut FontCollection,
    layers: &Layers,
    render_options: &mut RenderOptions,
    render_fn: impl Fn(&FreyaDOM, &NodeId, &Area, &mut FontCollection, &Viewports, &mut RenderOptions),
) {
    // Render all the layers from the bottom to the top
    for (_, layer) in layers.layers() {
        'elements: for node_id in layer {
            let node_viewports = viewports.get(node_id);
            let layout = fdom.layout();
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
                render_fn(
                    fdom,
                    node_id,
                    &areas.visible_area(),
                    font_collection,
                    viewports,
                    render_options,
                )
            }
        }
    }
}
