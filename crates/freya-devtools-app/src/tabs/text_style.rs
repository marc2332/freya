use freya::prelude::*;
use freya_core::integration::NodeId;
use freya_devtools::NodeStateAttributes;

use crate::{
    components::attribute::attributes_list,
    hooks::use_node_info,
};

#[derive(PartialEq)]
pub struct NodeInspectorTextStyle {
    pub node_id: NodeId,
    pub window_id: u64,
}

impl Component for NodeInspectorTextStyle {
    fn render(&self) -> impl IntoElement {
        let Some(node) = use_node_info(self.node_id, self.window_id) else {
            return rect().into_element();
        };
        attributes_list(node.state.text_style_attributes())
    }
}
