use std::rc::Rc;

use rustc_hash::FxHashMap;
use torin::{
    prelude::{
        Direction,
        TreeAdapter,
    },
    scaled::Scaled,
    size::Size,
};

use crate::{
    element::ElementExt,
    node_id::NodeId,
};

pub struct TreeAdapterFreya<'a> {
    pub elements: &'a FxHashMap<NodeId, Rc<dyn ElementExt>>,
    pub parents: &'a FxHashMap<NodeId, NodeId>,
    pub children: &'a FxHashMap<NodeId, Vec<NodeId>>,
    pub heights: &'a FxHashMap<NodeId, u16>,
    pub scale_factor: f64,
}

impl TreeAdapter<NodeId> for TreeAdapterFreya<'_> {
    fn root_id(&self) -> NodeId {
        NodeId::ROOT
    }

    fn get_node(&self, node_id: &NodeId) -> Option<torin::prelude::Node> {
        if *node_id == NodeId::ROOT {
            return Some(torin::node::Node::from_size_and_direction(
                Size::Fill,
                Size::Fill,
                Direction::Vertical,
            ));
        }
        self.elements.get(node_id).map(|node| {
            let mut layout_node = node.layout().layout.clone();
            layout_node.scale(self.scale_factor as f32);
            layout_node
        })
    }

    fn height(&self, node_id: &NodeId) -> Option<u16> {
        self.heights.get(node_id).cloned()
    }

    fn parent_of(&self, node_id: &NodeId) -> Option<NodeId> {
        self.parents.get(node_id).cloned()
    }

    fn children_of(&mut self, node_id: &NodeId) -> Vec<NodeId> {
        self.children.get(node_id).cloned().unwrap_or_default()
    }
}
