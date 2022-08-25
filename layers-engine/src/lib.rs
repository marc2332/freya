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
    pub layers: HashMap<i16, Vec<RenderData>>,
}

#[derive(Default, Clone, Debug)]
pub struct RenderData {
    pub node: NodeData,
    pub area: NodeArea,
    pub parent_area: NodeArea,
}

impl Layers {
    pub fn add_element(
        &mut self,
        node: &NodeData,
        area: &NodeArea,
        parent_area: &NodeArea,
        mut layer_num: i16,
    ) -> i16 {
        let node_data = node.node.as_ref().unwrap();
        let n_layer =
            (-node_data.state.style.z_index + (node_data.height as i16) - layer_num) as i16;

        if !self.layers.contains_key(&n_layer) {
            self.layers.insert(n_layer, vec![]);
        }

        let layer = self.layers.get_mut(&n_layer).unwrap();

        layer.push(RenderData {
            node: node.clone(),
            area: area.clone(),
            parent_area: parent_area.clone(),
        });

        // Elements inside container are moved down so they are under their parent when it's scrolled
        if let NodeType::Element { tag, .. } = &node_data.node_type {
            if tag == "container" {
                layer_num += node_data.height as i16 + 2;
            }
        }

        node_data.state.style.z_index + layer_num
    }
}
