use crate::{
    dom_adapter::NodeKey,
    geometry::{Area, Size2D},
    node::Node,
};

pub trait LayoutMeasurer<Key: NodeKey> {
    fn measure(
        &mut self,
        node_id: Key,
        node: &Node,
        parent_area: &Area,
        available_parent_area: &Area,
    ) -> Option<Size2D>;
}
