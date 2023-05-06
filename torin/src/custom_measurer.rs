use crate::{dom_adapter::NodeKey, geometry::Area, node::Node};

pub trait LayoutMeasurer<Key: NodeKey> {
    fn measure(
        &mut self,
        node_id: Key,
        node: &Node,
        area: &Area,
        parent_area: &Area,
        available_parent_area: &Area,
    ) -> Option<Area>;
}
