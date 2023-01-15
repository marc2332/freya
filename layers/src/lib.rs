use dioxus_core::ElementId;
use dioxus_native_core::{
    node::{Node, NodeType},
    NodeId,
};
use freya_common::NodeArea;
use freya_node_state::{CustomAttributeValues, NodeState};
use rustc_hash::FxHashMap;

pub type DioxusNode = Node<NodeState, CustomAttributeValues>;

#[derive(Default, Clone)]
pub struct Layers {
    pub layers: FxHashMap<i16, FxHashMap<NodeId, RenderData>>,
}

/// Collection of info about a specific Node to render
#[derive(Clone, Debug)]
pub struct RenderData {
    pub node_area: NodeArea,
    pub element_id: Option<ElementId>,
    pub node: DioxusNode,
    pub children: Option<Vec<NodeId>>,
}

impl RenderData {
    #[inline(always)]
    pub fn get_area(&self) -> &NodeArea {
        &self.node_area
    }

    #[inline(always)]
    pub fn get_element_id(&self) -> &Option<ElementId> {
        &self.element_id
    }

    #[inline(always)]
    pub fn get_type(&self) -> &NodeType<CustomAttributeValues> {
        &self.node.node_data.node_type
    }

    #[inline(always)]
    pub fn get_id(&self) -> &NodeId {
        &self.node.node_data.node_id
    }

    #[inline(always)]
    pub fn get_children(&self) -> &Option<Vec<NodeId>> {
        &self.children
    }

    #[inline(always)]
    pub fn get_state(&self) -> &NodeState {
        &self.node.state
    }
}

impl Layers {
    pub fn calculate_layer(
        &mut self,
        relative_layer: i16,
        height: i16,
        inherited_relative_layer: i16,
    ) -> (i16, i16) {
        // Relative layer (optionally define by the user) + height of the element in the VDOM - inherited relative_layer by parent
        let element_layer = -relative_layer + height - inherited_relative_layer;

        (element_layer, relative_layer + inherited_relative_layer)
    }

    pub fn add_element(
        &mut self,
        node: &DioxusNode,
        node_children: Option<Vec<NodeId>>,
        node_area: &NodeArea,
        node_layer: i16,
    ) {
        let layer = self
            .layers
            .entry(node_layer)
            .or_insert_with(FxHashMap::default);

        layer.insert(
            node.node_data.node_id,
            RenderData {
                element_id: node.node_data.element_id,
                node: node.clone(),
                node_area: *node_area,
                children: node_children,
            },
        );
    }
}
