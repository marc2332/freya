use dioxus::core::ElementId;
use dioxus_native_core::real_dom::{Node, NodeType};
use freya_node_state::node::NodeState;
use std::collections::HashMap;

#[derive(Clone)]
pub struct NodeData {
    pub node: Node<NodeState>,
}

#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub struct NodeArea {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Default, Clone)]
pub struct Layers {
    pub layers: HashMap<i16, HashMap<ElementId, RenderData>>,
}

#[derive(Clone)]
pub struct RenderData {
    pub node_data: NodeData,
    pub node_area: NodeArea,
    pub node_children: Vec<ElementId>,
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

    pub fn add_element(&mut self, node_data: &NodeData, area: &NodeArea, node_layer: i16) {
        if !self.layers.contains_key(&node_layer) {
            self.layers.insert(node_layer, HashMap::default());
        }

        let layer = self.layers.get_mut(&node_layer).unwrap();

        let mut node_children = Vec::new();

        if let NodeType::Element { children, .. } = &node_data.node.node_type {
            node_children = children.clone();
        }

        layer.insert(
            node_data.node.id,
            RenderData {
                node_data: node_data.clone(),
                node_area: area.clone(),
                node_children,
            },
        );
    }
}
