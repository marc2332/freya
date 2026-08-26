use rustc_hash::FxHashMap;
use torin::{
    node::Node,
    prelude::{
        Direction,
        TreeAdapter,
    },
    size::Size,
};

use crate::node_id::NodeId;

pub struct TreeAdapterFreya<'a> {
    pub layout_nodes: &'a FxHashMap<NodeId, Node>,
    pub parents: &'a FxHashMap<NodeId, NodeId>,
    pub children: &'a FxHashMap<NodeId, Vec<NodeId>>,
    pub heights: &'a FxHashMap<NodeId, u16>,
}

impl TreeAdapter<NodeId> for TreeAdapterFreya<'_> {
    fn root_id(&self) -> NodeId {
        NodeId::ROOT
    }

    fn read_node<R>(
        &self,
        node_id: &NodeId,
        reader: impl FnOnce(&Node, &[NodeId]) -> R,
    ) -> Option<R> {
        let children = self.children.get(node_id).map_or(&[][..], Vec::as_slice);

        if *node_id == NodeId::ROOT {
            let root = Node::from_size_and_direction(Size::Fill, Size::Fill, Direction::Vertical);
            return Some(reader(&root, children));
        }

        self.layout_nodes
            .get(node_id)
            .map(|layout_node| reader(layout_node, children))
    }

    fn height(&self, node_id: &NodeId) -> Option<u16> {
        self.heights.get(node_id).copied()
    }

    fn parent_of(&self, node_id: &NodeId) -> Option<NodeId> {
        self.parents.get(node_id).copied()
    }
}
