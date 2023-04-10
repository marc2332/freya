use dioxus_native_core::real_dom::NodeImmutable;
use dioxus_native_core::NodeId;
use freya_common::Area;
use freya_dom::{DioxusNode, FreyaDOM};
use rustc_hash::FxHashMap;

#[derive(Default, Clone)]
pub struct Layers {
    pub layers: FxHashMap<i16, FxHashMap<NodeId, RenderData>>,
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
