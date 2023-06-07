use dioxus_native_core::node::NodeType;
use dioxus_native_core::prelude::ElementNode;
use dioxus_native_core::real_dom::NodeImmutable;

use freya_dom::prelude::FreyaDOM;
use freya_layout::Layers;

use rustc_hash::FxHashMap;

use crate::ViewportsCollection;

// Calculate all the applicable viewports for the given nodes
pub fn calculate_viewports(
    layers_nums: &[&i16],
    layers: &Layers,
    fdom: &FreyaDOM,
) -> ViewportsCollection {
    let mut viewports_collection = FxHashMap::default();
    let layout = fdom.layout();
    let rdom = fdom.rdom();

    for layer_num in layers_nums {
        let layer = layers.layers.get(layer_num).unwrap();
        for node_id in layer {
            let node = rdom.get(*node_id);
            let node_areas = layout.get(*node_id);

            if let Some((node, node_areas)) = node.zip(node_areas) {
                let node_type = &*node.node_type();

                if let NodeType::Element(ElementNode { tag, .. }) = node_type {
                    // `container` elements will clip any overflow from it's children
                    if tag == "container" {
                        viewports_collection
                            .entry(*node_id)
                            .or_insert_with(|| (None, Vec::new()))
                            .0 = Some(node_areas.area);
                    }

                    for child in node.children() {
                        if viewports_collection.contains_key(node_id) {
                            let mut inherited_viewports =
                                viewports_collection.get(node_id).unwrap().1.clone();

                            inherited_viewports.push(*node_id);

                            viewports_collection.insert(child.id(), (None, inherited_viewports));
                        }
                    }
                }
            }
        }
    }
    viewports_collection
}
