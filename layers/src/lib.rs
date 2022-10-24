use std::collections::BTreeMap;

use dioxus::core::ElementId;
use dioxus_native_core::real_dom::{Node, NodeType};
use freya_layout_common::NodeArea;
use freya_node_state::node::NodeState;

#[derive(Clone)]
pub struct NodeData {
    pub node: Node<NodeState>,
}

#[derive(Default, Clone)]
pub struct Layers {
    pub layers: BTreeMap<i16, BTreeMap<usize, RenderData>>,
}

#[derive(Clone)]
pub struct RenderData {
    pub node_state: NodeState,
    pub node_area: NodeArea,
    pub node_id: ElementId,
    pub node_type: NodeType,
}

impl Layers {
    pub fn calculate_layer(
        &mut self,
        node_data: &NodeData,
        inherited_relative_layer: i16,
    ) -> (i16, i16) {
        // Relative layer (optionally define by the user) + height of the element in the VDOM - inherited relative_layer by parent
        let element_layer = (-node_data.node.state.style.relative_layer
            + (node_data.node.height as i16)
            - inherited_relative_layer) as i16;

        (
            element_layer,
            node_data.node.state.style.relative_layer + inherited_relative_layer,
        )
    }

    pub fn add_element(&mut self, node_data: &NodeData, node_area: &NodeArea, node_layer: i16) {
        let layer = self.layers.entry(node_layer).or_insert_with(BTreeMap::new);

        layer.insert(
            node_data.node.id.0,
            RenderData {
                node_id: node_data.node.id,
                node_type: node_data.node.node_type.clone(),
                node_state: node_data.node.state.clone(),
                node_area: *node_area,
            },
        );
    }
}
