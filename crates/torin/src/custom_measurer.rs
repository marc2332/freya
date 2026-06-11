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

/// Result of a [LayoutMeasurer::post_measure] step.
pub struct PostMeasure<Key: NodeKey> {
    /// Corrected content size, re-applied through the Node's min/max sizing.
    pub content_size: Option<Size2D>,
    /// `(x, y)` offsets to move each visible child's subtree by.
    pub offsets: Vec<(Key, Length, Length)>,
    /// Children whose subtree must be hidden from painting and events.
    pub hidden_children: Vec<Key>,
}

impl<Key: NodeKey> Default for PostMeasure<Key> {
    fn default() -> Self {
        Self {
            content_size: None,
            offsets: Vec::new(),
            hidden_children: Vec::new(),
        }
    }
}

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

    /// Runs after a node and its children are measured.
    fn post_measure(
        &mut self,
        _node_id: Key,
        _node_layout: &LayoutNode,
        _children: &[Key],
        _layout: &Torin<Key>,
    ) -> PostMeasure<Key> {
        PostMeasure::default()
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
