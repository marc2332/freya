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
    pub parent_area: NodeArea,
    pub children: Vec<ElementId>,
}

impl Layers {
    pub fn add_element(
        &mut self,
        node: &NodeData,
        area: &NodeArea,
        parent_area: &NodeArea,
        layer_num: i16,
    ) -> i16 {
        let node_data = node.node.as_ref().unwrap();
        let n_layer =
            (-node_data.state.style.z_index + (node_data.height as i16) - layer_num) as i16;

        if !self.layers.contains_key(&n_layer) {
            self.layers.insert(n_layer, HashMap::default());
        }

        let layer = self.layers.get_mut(&n_layer).unwrap();

        let mut children_s = Vec::new();

        if let NodeType::Element { children, .. } = &node_data.node_type {
            children_s = children.clone();
        }

        layer.insert(
            node_data.id,
            RenderData {
                node: node.clone(),
                area: area.clone(),
                parent_area: parent_area.clone(),
                children: children_s,
            },
        );

        node_data.state.style.z_index + layer_num
    }
}
