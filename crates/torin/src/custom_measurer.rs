use freya_native_core::prelude::SendAnyMap;
use std::sync::Arc;

use crate::{dom_adapter::NodeKey, geometry::Size2D, node::Node, prelude::LayoutNode};

pub trait LayoutMeasurer<Key: NodeKey> {
    fn measure(
        &mut self,
        node_id: Key,
        node: &Node,
        size: &Size2D,
    ) -> Option<(Size2D, Arc<SendAnyMap>)>;

    fn should_measure_inner_children(&mut self, node_id: Key) -> bool;

    fn notify_layout_references(&self, _node_id: Key, _layout_node: &LayoutNode) {}
}
