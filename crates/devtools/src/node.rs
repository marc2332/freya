use dioxus::prelude::*;
use freya_components::ButtonStatus;
use freya_elements::elements as dioxus_elements;

use crate::TreeNode;

#[allow(non_snake_case)]
#[component]
pub fn NodeElement(
    node: TreeNode,
    is_selected: bool,
    onselected: EventHandler<TreeNode>,
) -> Element {
    let mut status = use_signal(ButtonStatus::default);

    let onmousedown = {
        to_owned![node];
        move |_| onselected.call(node.clone())
    };

    let onmouseover = move |_| {
        if *status.read() != ButtonStatus::Hovering {
            status.set(ButtonStatus::Hovering);
        }
    };

    let onmouseleave = move |_| {
        status.set(ButtonStatus::default());
    };

    let background = if is_selected {
        "rgb(100, 100, 100)"
    } else {
        "transparent"
    };
    let color = if is_selected {
        "white"
    } else {
        match *status.read() {
            ButtonStatus::Idle => "white",
            ButtonStatus::Hovering => "rgb(150, 150, 150)",
        }
    };
    let margin_left = (node.height * 10) as f32 + 16.5;

    rsx!(
        rect {
            corner_radius: "7",
            padding: "5",
            background: background,
            width: "100%",
            height: "27",
            offset_x: "{margin_left}",
            onmousedown,
            onmouseover,
            onmouseleave,
            label {
                font_size: "14",
                color: "{color}",
                "{node.tag} #{node.id:?}"
            }
        }
    )
}
