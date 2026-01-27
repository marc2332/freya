use std::{
    any::Any,
    rc::Rc,
};

use crate::{
    geometry::Size2D,
    measure::Phase,
    node::Node,
    prelude::Area,
    tree_adapter::NodeKey,
};

pub trait LayoutMeasurer<Key: NodeKey> {
    fn measure(
        &mut self,
        node_id: Key,
        node: &Node,
        size: &Size2D,
        phase: Phase,
        parent_phase: Phase,
    ) -> Option<(Size2D, Rc<dyn Any>)>;

    fn should_hook_measurement(&mut self, node_id: Key) -> bool;

    fn should_measure_inner_children(&mut self, node_id: Key) -> bool;

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
        _phase: Phase,
        _parent_phase: Phase,
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
