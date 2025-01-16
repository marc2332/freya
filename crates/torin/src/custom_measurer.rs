use crate::{
    dom_adapter::NodeKey,
    geometry::Size2D,
    node::Node,
    prelude::{
        Area,
        NodeData,
    },
};

pub trait LayoutMeasurer<Key: NodeKey, Data: NodeData> {
    fn measure(&mut self, node_id: Key, node: &Node, size: &Size2D) -> Option<(Size2D, Data)>;

    fn should_measure_inner_children(&mut self, node_id: Key) -> bool;

    fn notify_layout_references(&self, _node_id: Key, _area: Area, _inner_sizes: Size2D) {}
}

// No-op measurer, use it when you don't need one.
pub struct NoopMeasurer;

impl LayoutMeasurer<usize, ()> for NoopMeasurer {
    fn measure(&mut self, _node_id: usize, _node: &Node, _size: &Size2D) -> Option<(Size2D, ())> {
        None
    }

    fn should_measure_inner_children(&mut self, _node_id: usize) -> bool {
        false
    }
}
