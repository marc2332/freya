use dioxus_native_core::{real_dom::NodeImmutable, NodeId};
use freya_common::{Area, NodeReferenceLayout};
use freya_dom::{DioxusNode, FreyaDOM};
use freya_node_state::{SizeState, Style};
use rustc_hash::{FxHashMap, FxHashSet};
use skia_safe::textlayout::FontCollection;
use torin::Torin;
use uuid::Uuid;

use crate::DioxusDOM;

#[derive(Default, Clone)]
pub struct Layers {
    pub layers: FxHashMap<i16, FxHashSet<NodeId>>,
    pub paragraph_elements: FxHashMap<Uuid, FxHashSet<NodeId>>,
}

impl Layers {
    pub fn new(rdom: &DioxusDOM, layout: &Torin<NodeId>) -> Self {
        let mut layers = Layers::default();
        let mut inherit_layers = FxHashMap::default();

        rdom.traverse_depth_first(|node| {
            // Add the Node to a Layer
            let node_style = node.get::<Style>().unwrap();

            let inherited_relative_layer = node
                .parent_id()
                .map(|p| *inherit_layers.get(&p).unwrap())
                .unwrap_or(0);

            let (node_layer, node_relative_layer) = Layers::calculate_layer(
                node_style.relative_layer,
                node.height() as i16,
                inherited_relative_layer,
            );

            inherit_layers.insert(node.id(), node_relative_layer);
            layers.add_element(node.id(), node_layer);

            // Notify layout references

            let size_state = &*node.get::<SizeState>().unwrap();

            if let Some(reference) = &size_state.node_ref {
                let areas = layout.get_size(node.id()).unwrap();
                let node_layout = NodeReferenceLayout {
                    area: areas.area,
                    inner: areas.inner_sizes,
                };
                //node_layout.div(1.0);
                reference.send(node_layout).ok();
            }
        });

        layers
    }

    /// Measure all the paragraphs registered under the given TextId
    pub fn measure_paragraph_elements(
        &self,
        text_id: &Uuid,
        _dom: &FreyaDOM,
        _font_collection: &FontCollection,
    ) {
        let _group = self.paragraph_elements.get(text_id);

        /*if let Some(group) = group {
            for (id, area) in group {
                let node = dom.dom().get(*id);
                if let Some(node) = node {
                    process_paragraph(&node, area, font_collection, true);
                }
            }
        } */
    }

    /// Register a paragraph element under it's configured TextId
    pub fn insert_paragraph_element(&mut self, _node: &DioxusNode, _area: &Area) {
        /*let references = node.get::<References>().unwrap();
        if let Some(cursor_ref) = &references.cursor_ref {
            let text_group = self
                .paragraph_elements
                .entry(cursor_ref.text_id)
                .or_insert_with(FxHashMap::default);

            text_group.insert(node.id(), *area);
        } */
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
