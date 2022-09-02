use dioxus::core::ElementId;
use dioxus_native_core::real_dom::{Node, NodeType};
use state::node::{NodeState, SizeMode};
use std::collections::HashMap;

#[derive(Default, Clone, Debug)]
pub struct NodeData {
    pub width: SizeMode,
    pub height: SizeMode,
    pub padding: (i32, i32, i32, i32),
    pub node: Option<Node<NodeState>>,
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub struct NodeArea {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Default, Clone, Debug)]
pub struct Layers {
    pub layers: HashMap<i16, HashMap<ElementId, RenderData>>,
}

#[derive(Default, Clone, Debug)]
pub struct RenderData {
    pub node: NodeData,
    pub area: NodeArea,
    pub children: Vec<ElementId>,
}

impl Layers {
    pub fn calculate_layer(
        &mut self,
        node: &NodeData,
        inherited_relative_layer: i16,
    ) -> (i16, i16) {
        let node_data = node.node.as_ref().unwrap();

        // Relative layer (optionally define by the user) + height of the element in the VDOM - inherited relative_layer by parent
        let element_layer = (-node_data.state.style.relative_layer + (node_data.height as i16)
            - inherited_relative_layer) as i16;

        (
            element_layer,
            node_data.state.style.relative_layer + inherited_relative_layer,
        )
    }

    pub fn add_element(&mut self, node: &NodeData, area: &NodeArea, node_layer: i16) {
        let node_data = node.node.as_ref().unwrap();

        if !self.layers.contains_key(&node_layer) {
            self.layers.insert(node_layer, HashMap::default());
        }

        let layer = self.layers.get_mut(&node_layer).unwrap();

        let mut children_s = Vec::new();

        if let NodeType::Element { children, .. } = &node_data.node_type {
            children_s = children.clone();
        }

        layer.insert(
            node_data.id,
            RenderData {
                node: node.clone(),
                area: area.clone(),
                children: children_s,
            },
        );
    }
}
