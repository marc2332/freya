use crate::{geometry::Area, node::Node, node_resolver::NodeKey};

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
