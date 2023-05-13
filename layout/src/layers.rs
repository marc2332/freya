use dioxus_native_core::NodeId;
use freya_dom::FreyaDOM;
use rustc_hash::{FxHashMap, FxHashSet};
use skia_safe::textlayout::FontCollection;
use torin::torin::Torin;
use uuid::Uuid;

use crate::{measure_paragraph, DioxusDOM};

#[derive(Default, Clone)]
pub struct Layers {
    pub layers: FxHashMap<i16, FxHashSet<NodeId>>,
    pub paragraph_elements: FxHashMap<Uuid, FxHashSet<NodeId>>,
}

impl Layers {
    /// Measure all the paragraphs
    pub fn measure_all_paragraph_elements(
        &self,
        rdom: &DioxusDOM,
        layout: &Torin<NodeId>,
        font_collection: &FontCollection,
    ) {
        for group in self.paragraph_elements.values() {
            for node_id in group {
                let node = rdom.get(*node_id);
                let areas = layout.get(*node_id);
                if let Some((node, areas)) = node.zip(areas) {
                    measure_paragraph(&node, &areas.area, font_collection, true);
                }
            }
        }
    }

    /// Measure all the paragraphs registered under the given TextId
    pub fn measure_paragraph_elements(
        &self,
        text_id: &Uuid,
        dom: &FreyaDOM,
        font_collection: &FontCollection,
    ) {
        let group = self.paragraph_elements.get(text_id);
        let layout = dom.layout();
        if let Some(group) = group {
            for node_id in group {
                let node = dom.rdom().get(*node_id);
                let areas = layout.get(*node_id);

                if let Some((node, areas)) = node.zip(areas) {
                    measure_paragraph(&node, &areas.area, font_collection, true);
                }
            }
        }
    }

    /// Given the height in the DOM of the Node, it's inherited layer from it's parent
    /// and the defined layer via the `layer` attribute,
    /// calculate it's corresponding layer and it's relative layer for it's children to inherit
    pub fn calculate_layer(
        relative_layer: i16,
        height: i16,
        inherited_relative_layer: i16,
    ) -> (i16, i16) {
        let element_layer = -relative_layer + height - inherited_relative_layer;
        (element_layer, relative_layer + inherited_relative_layer)
    }

    /// Insert a Node into a layer
    pub fn add_element(&mut self, node_id: NodeId, node_layer: i16) {
        let layer = self
            .layers
            .entry(node_layer)
            .or_insert_with(FxHashSet::default);

        layer.insert(node_id);
    }
}
