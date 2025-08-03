use freya::prelude::*;
use freya_devtools::NodeStateAttributes;
use freya_native_core::NodeId;

use crate::{
    components::attribute::attribute_element,
    hooks::use_node_info,
};

#[component]
pub fn NodeInspectorLayout(node_id: NodeId, window_id: u64) -> Element {
    let Some(node) = use_node_info(node_id, window_id) else {
        return Ok(VNode::placeholder());
    };

    rsx!(
        ScrollView {
            show_scrollbar: true,
            height : "fill",
            width: "fill",
            {node.state.layout_attributes().into_iter().enumerate().filter_map(|(i, (name, attribute))| {
                let background = if i % 2 == 0 {
                    "rgb(255, 255, 255, 0.1)"
                } else {
                    "transparent"
                };

                let element = attribute_element(i, name, attribute)?;

                Some(rsx!(
                    rect {
                        background,
                        padding: "5 16",
                        {element}
                    }
                ))
            })}
        }
    )
}
