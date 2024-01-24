use std::vec::IntoIter;

use dioxus_native_core::prelude::ElementNode;
use dioxus_native_core::real_dom::NodeImmutable;
use dioxus_native_core::tree::TreeRef;
use dioxus_native_core::{node::NodeType, NodeId};
use freya_common::NodeReferenceLayout;
use freya_dom::dom::DioxusNode;
use freya_dom::prelude::{DioxusDOM, FreyaDOM};
use itertools::sorted;

use freya_engine::prelude::*;
use freya_node_state::{CursorMode, CursorSettings, LayoutState, References, Style};
use rustc_hash::FxHashMap;
use torin::torin::Torin;
use uuid::Uuid;

use crate::layout::*;

fn traverse_dom(rdom: &DioxusDOM, mut f: impl FnMut(DioxusNode) -> bool) {
    let mut stack = vec![rdom.root_id()];
    let tree = rdom.tree_ref();
    while let Some(id) = stack.pop() {
        if let Some(node) = rdom.get(id) {
            let traverse_children = f(node);
            if traverse_children {
                let children = tree.children_ids_advanced(id, true);
                stack.extend(children.iter().copied().rev());
            }
        }
    }
}

#[derive(Default, Clone)]
pub struct Layers {
    pub layers: FxHashMap<i16, Vec<NodeId>>,
    pub paragraph_elements: FxHashMap<Uuid, Vec<NodeId>>,
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

        traverse_dom(rdom, |node| {
            let areas = layout.get(node.id());

            // Some elements like placeholders are not measured
            if let Some(areas) = areas {
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

                let traverse_inner_children =
                    if let NodeType::Element(ElementNode { tag, .. }) = &*node.node_type() {
                        let is_paragraph = tag == "paragraph";
                        if is_paragraph {
                            let cursor_settings = node.get::<CursorSettings>().unwrap();
                            let is_editable = CursorMode::Editable == cursor_settings.mode;

                            let references = node.get::<References>().unwrap();
                            if is_editable {
                                if let Some(cursor_ref) = &references.cursor_ref {
                                    let text_group = layers
                                        .paragraph_elements
                                        .entry(cursor_ref.text_id)
                                        .or_default();

                                    text_group.push(node.id());
                                }
                            }
                        }

                        // Traverse all elements except paragraphs
                        !is_paragraph
                    } else {
                        false
                    };

                // Notify layout references

                let size_state = &*node.get::<LayoutState>().unwrap();

                if let Some(reference) = &size_state.node_ref {
                    let mut node_layout = NodeReferenceLayout {
                        area: areas.area,
                        inner: areas.inner_sizes,
                    };
                    node_layout.div(scale_factor);
                    reference.0.send(node_layout).ok();
                }

                traverse_inner_children
            } else {
                false
            }
        });

        layers.measure_all_paragraph_elements(rdom, layout, font_collection, scale_factor);

        layers
    }

    pub fn layers(&self) -> IntoIter<(&i16, &Vec<NodeId>)> {
        sorted(self.layers.iter())
    }

    pub fn len_paragraph_elements(&self) -> usize {
        self.paragraph_elements.len()
    }

    pub fn len_layers(&self) -> usize {
        self.layers.len()
    }

    /// Measure all the paragraphs
    pub fn measure_all_paragraph_elements(
        &self,
        rdom: &DioxusDOM,
        layout: &Torin<NodeId>,
        font_collection: &FontCollection,
        scale_factor: f32,
    ) {
        for group in self.paragraph_elements.values() {
            for node_id in group {
                let node = rdom.get(*node_id);
                let areas = layout.get(*node_id);
                if let Some((node, areas)) = node.zip(areas) {
                    measure_paragraph(&node, &areas.area, font_collection, true, scale_factor);
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
        scale_factor: f32,
    ) {
        let group = self.paragraph_elements.get(text_id);
        let layout = dom.layout();
        if let Some(group) = group {
            for node_id in group {
                let node = dom.rdom().get(*node_id);
                let areas = layout.get(*node_id);

                if let Some((node, areas)) = node.zip(areas) {
                    measure_paragraph(&node, &areas.area, font_collection, true, scale_factor);
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
        let layer = self.layers.entry(node_layer).or_default();

        layer.push(node_id);
    }
}
