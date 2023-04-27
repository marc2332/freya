use dioxus_native_core::real_dom::NodeImmutable;
use dioxus_native_core::NodeId;
use freya_common::Area;
use freya_dom::{DioxusNode, FreyaDOM};
use freya_node_state::References;
use rustc_hash::FxHashMap;
use skia_safe::textlayout::FontCollection;
use uuid::Uuid;

use crate::process_paragraph;

#[derive(Default, Clone)]
pub struct Layers {
    pub layers: FxHashMap<i16, FxHashMap<NodeId, RenderData>>,
    pub paragraph_elements: FxHashMap<Uuid, FxHashMap<NodeId, Area>>,
}

/// Collection of info about a specific Node to render
#[derive(Clone, Debug)]
pub struct RenderData {
    pub node_area: Area,
    pub node_id: NodeId,
    pub children: Vec<NodeId>,
}

impl RenderData {
    #[inline(always)]
    pub fn get_area(&self) -> &Area {
        &self.node_area
    }

    #[inline(always)]
    pub fn get_id(&self) -> &NodeId {
        &self.node_id
    }

    #[inline(always)]
    pub fn get_children(&self) -> &Vec<NodeId> {
        &self.children
    }

    #[inline(always)]
    pub fn get_node<'a>(&'a self, rdom: &'a FreyaDOM) -> Option<DioxusNode> {
        rdom.dom().get(self.node_id)
    }
}

impl Layers {
    /// Measure all the paragraphs registered under the given TextId
    pub fn measure_paragraph_elements(
        &self,
        text_id: &Uuid,
        dom: &FreyaDOM,
        font_collection: &FontCollection,
    ) {
        let group = self.paragraph_elements.get(text_id);

        if let Some(group) = group {
            for (id, area) in group {
                let node = dom.dom().get(*id);
                if let Some(node) = node {
                    process_paragraph(&node, area, font_collection, true);
                }
            }
        }
    }

    /// Register a paragraph element under it's configured TextId
    pub fn insert_paragraph_element(&mut self, node: &DioxusNode, area: &Area) {
        let references = node.get::<References>().unwrap();
        if let Some(cursor_ref) = &references.cursor_ref {
            let text_group = self
                .paragraph_elements
                .entry(cursor_ref.text_id)
                .or_insert_with(FxHashMap::default);

            text_group.insert(node.id(), *area);
        }
    }

    /// Given the height in the DOM of the Node, it's inherited layer from it's parent
    /// and the defined layer via the `layer` attribute,
    /// calculate it's corresponding layer and it's relative layer for it's children to inherit
    pub fn calculate_layer(
        &mut self,
        relative_layer: i16,
        height: i16,
        inherited_relative_layer: i16,
    ) -> (i16, i16) {
        let element_layer = -relative_layer + height - inherited_relative_layer;
        (element_layer, relative_layer + inherited_relative_layer)
    }

    /// Insert a Node into a layer
    pub fn add_element(&mut self, node: &DioxusNode, node_area: &Area, node_layer: i16) {
        let layer = self
            .layers
            .entry(node_layer)
            .or_insert_with(FxHashMap::default);

        layer.insert(
            node.id(),
            RenderData {
                node_id: node.id(),
                node_area: *node_area,
                children: node.child_ids(),
            },
        );
    }
}
