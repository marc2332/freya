use dioxus::prelude::*;
use freya_components::*;
use freya_elements::elements as dioxus_elements;

use crate::{NodeInspectorBar, TreeNode};

#[allow(non_snake_case)]
#[inline_props]
pub fn NodeInspectorStyle<'a>(cx: Scope<'a>, _node: &'a TreeNode) -> Element<'a> {
    render!(
        container {
            width: "100%",
            height: "50%",
            NodeInspectorBar { }
            ScrollView {
                show_scrollbar: true,
                height: "calc(100% - 35)",
                width: "100%",

            }
        }
    )
}
