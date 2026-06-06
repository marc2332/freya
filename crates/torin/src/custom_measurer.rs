use std::{
    any::Any,
    rc::Rc,
};

use crate::{
    geometry::Size2D,
    node::Node,
    prelude::{
        Area,
        Length,
    },
    torin::Torin,
    tree_adapter::{
        LayoutNode,
        NodeKey,
    },
};

pub trait LayoutMeasurer<Key: NodeKey> {
    fn measure(
        &mut self,
        node_id: Key,
        node: &Node,
        size: &Size2D,
    ) -> Option<(Size2D, Rc<dyn Any>)>;

    fn should_hook_measurement(&mut self, node_id: Key) -> bool;

    fn should_measure_inner_children(&mut self, node_id: Key) -> bool;

    /// Whether [LayoutMeasurer::post_measure] should run for this node.
    fn should_post_measure(&mut self, _node_id: Key) -> bool {
        false
    }

    /// Runs after a node and its children are measured. Returns an optional corrected content
    /// size (re-applied through min/max) and `(x, y)` offsets to move each child's subtree.
    fn post_measure(
        &mut self,
        _node_id: Key,
        _node_layout: &LayoutNode,
        _children: &[Key],
        _layout: &Torin<Key>,
    ) -> (Option<Size2D>, Vec<(Key, Length, Length)>) {
        (None, Vec::new())
    }

    fn notify_layout_references(
        &mut self,
        _node_id: Key,
        _area: Area,
        _visible_area: Area,
        _inner_sizes: Size2D,
    ) {
    }
}

// No-op measurer, use it when you don't need one.
pub struct NoopMeasurer;

impl LayoutMeasurer<usize> for NoopMeasurer {
    fn measure(
        &mut self,
        _node_id: usize,
        _node: &Node,
        _size: &Size2D,
    ) -> Option<(Size2D, Rc<dyn Any>)> {
        None
    }

    fn should_hook_measurement(&mut self, _node_id: usize) -> bool {
        false
    }

    fn should_measure_inner_children(&mut self, _node_id: usize) -> bool {
        false
    }
}
