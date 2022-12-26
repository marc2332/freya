use dioxus_core::ElementId;
use dioxus_native_core::{
    node::{Node, NodeType},
    NodeId,
};
use freya_common::NodeArea;
use freya_node_state::NodeState;
use rustc_hash::FxHashMap;

#[derive(Clone)]
pub struct NodeData {
    pub node: Node<NodeState>,
    pub height: u16,
    pub parent_id: Option<NodeId>,
    pub children: Option<Vec<NodeId>>,
}

#[derive(Default, Clone)]
pub struct Layers {
    pub layers: FxHashMap<i16, FxHashMap<NodeId, RenderData>>,
}

#[derive(Clone, Debug)]
pub struct RenderData {
    pub node_state: NodeState,
    pub node_area: NodeArea,
    pub node_id: NodeId,
    pub element_id: Option<ElementId>,
    pub node_type: NodeType,
    pub children: Option<Vec<NodeId>>,
}

impl Layers {
    pub fn calculate_layer(
        &mut self,
        node_data: &NodeData,
        inherited_relative_layer: i16,
    ) -> (i16, i16) {
        // Relative layer (optionally define by the user) + height of the element in the VDOM - inherited relative_layer by parent
        let element_layer = (-node_data.node.state.style.relative_layer + (node_data.height as i16)
            - inherited_relative_layer) as i16;

        (
            element_layer,
            node_data.node.state.style.relative_layer + inherited_relative_layer,
        )
    }

    pub fn add_element(&mut self, node_data: &NodeData, node_area: &NodeArea, node_layer: i16) {
        let layer = self
            .layers
            .entry(node_layer)
            .or_insert_with(FxHashMap::default);

        layer.insert(
            node_data.node.node_data.node_id,
            RenderData {
                element_id: node_data.node.node_data.element_id,
                node_id: node_data.node.node_data.node_id,
                node_type: node_data.node.node_data.node_type.clone(),
                node_state: node_data.node.state.clone(),
                node_area: *node_area,
                children: node_data.children.clone(),
            },
        );
    }
}
