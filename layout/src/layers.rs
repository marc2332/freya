use dioxus_native_core::{
    prelude::{ElementNode, NodeType},
    real_dom::NodeImmutable,
    NodeId,
};
use freya_common::NodeReferenceLayout;
use freya_dom::FreyaDOM;
use freya_node_state::{CursorMode, CursorSettings, References, SizeState, Style};
use rustc_hash::{FxHashMap, FxHashSet};
use skia_safe::textlayout::FontCollection;
use torin::Torin;
use uuid::Uuid;

use crate::{process_paragraph, DioxusDOM};

#[derive(Default, Clone)]
pub struct Layers {
    pub layers: FxHashMap<i16, FxHashSet<NodeId>>,
    pub paragraph_elements: FxHashMap<Uuid, FxHashSet<NodeId>>,
}

impl Layers {
    pub fn new(
        rdom: &DioxusDOM,
        layout: &Torin<NodeId>,
        font_collection: &FontCollection,
        scale_factor: f32,
    ) -> Self {
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

            // Register paragraph elements

            if let NodeType::Element(ElementNode { tag, .. }) = &*node.node_type() {
                if tag == "paragraph" {
                    let cursor_settings = node.get::<CursorSettings>().unwrap();
                    let is_editable = CursorMode::Editable == cursor_settings.mode;

                    let references = node.get::<References>().unwrap();
                    if is_editable {
                        if let Some(cursor_ref) = &references.cursor_ref {
                            let text_group = layers
                                .paragraph_elements
                                .entry(cursor_ref.text_id)
                                .or_insert_with(FxHashSet::default);

                            text_group.insert(node.id());
                        }
                    }
                }
            }

            // Notify layout references

            let size_state = &*node.get::<SizeState>().unwrap();

            if let Some(reference) = &size_state.node_ref {
                let areas = layout.get_size(node.id()).unwrap();
                let mut node_layout = NodeReferenceLayout {
                    area: areas.area,
                    inner: areas.inner_sizes,
                };
                node_layout.div(scale_factor);
                reference.send(node_layout).ok();
            }
        });

        layers.measure_all_paragraph_elements(rdom, layout, font_collection);

        layers
    }

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
                let areas = layout.get_size(*node_id);
                if let Some((node, areas)) = node.zip(areas) {
                    process_paragraph(&node, &areas.area, font_collection, true);
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
                let areas = layout.get_size(*node_id);

                if let Some((node, areas)) = node.zip(areas) {
                    process_paragraph(&node, &areas.area, font_collection, true);
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
