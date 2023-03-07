use dioxus::prelude::*;
use freya_elements as dioxus_elements;

use crate::TreeNode;

#[allow(non_snake_case)]
#[inline_props]
pub fn NodeElement<'a>(
    cx: Scope<'a>,
    node: &'a TreeNode,
    is_selected: bool,
    onselected: EventHandler<'a, &'a TreeNode>,
) -> Element<'a> {
    let text_color = use_state(cx, || "white");

    let mut color = *text_color.get();
    let margin_left = (node.height * 10) as f32 + 16.5;
    let mut background = "transparent";

    if *is_selected {
        color = "white";
        background = "rgb(100, 100, 100)";
    };

    render!(
        rect {
            radius: "7",
            padding: "5",
            background: background,
            width: "100%",
            height: "27",
            scroll_x: "{margin_left}",
            onmousedown: |_| onselected.call(node),
            onmouseover: move |_| {
                text_color.set("rgb(150, 150, 150)");
            },
            onmouseleave: move |_| {
                text_color.set("white");
            },
            label {
                font_size: "14",
                color: "{color}",
                "{node.tag} #{node.id.0}"
            }
        }
    )
}
