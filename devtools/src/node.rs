use dioxus::prelude::*;
use freya_components::ButtonStatus;
use freya_elements::elements as dioxus_elements;

use crate::TreeNode;

#[allow(non_snake_case)]
#[inline_props]
pub fn NodeElement<'a>(
    cx: Scope<'a>,
    node: &'a TreeNode,
    is_selected: bool,
    onselected: EventHandler<'a, &'a TreeNode>,
) -> Element<'a> {
    let status = use_state(cx, ButtonStatus::default);

    let onmousedown = move |_| onselected.call(node);

    let onmouseover = move |_| {
        if *status.get() != ButtonStatus::Hovering {
            status.set(ButtonStatus::Hovering);
        }
    };

    let onmouseleave = move |_| {
        status.set(ButtonStatus::default());
    };

    let background = if *is_selected {
        "rgb(100, 100, 100)"
    } else {
        "transparent"
    };
    let color = if *is_selected {
        "white"
    } else {
        match *status.get() {
            ButtonStatus::Idle => "white",
            ButtonStatus::Hovering => "rgb(150, 150, 150)",
        }
    };
    let margin_left = (node.height * 10) as f32 + 16.5;

    render!(
        rect {
            corner_radius: "7",
            padding: "5",
            background: background,
            width: "100%",
            height: "27",
            offset_x: "{margin_left}",
            onmousedown: onmousedown,
            onmouseover: onmouseover,
            onmouseleave: onmouseleave,
            label {
                font_size: "14",
                color: "{color}",
                "{node.tag} #{node.id:?}"
            }
        }
    )
}
