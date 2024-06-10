use dioxus::prelude::*;
use freya_components::ButtonStatus;
use freya_elements::elements as dioxus_elements;
use freya_native_core::prelude::NodeId;

use crate::hooks::use_node_info;

#[allow(non_snake_case)]
#[component]
pub fn NodeElement(
    node_id: NodeId,
    is_selected: bool,
    onselected: EventHandler<NodeId>,
) -> Element {
    let mut status = use_signal(ButtonStatus::default);
    let node = use_node_info(node_id)?;

    let onmousedown = move |_| onselected.call(node_id);

    let onmouseenter = move |_| {
        status.set(ButtonStatus::Hovering);
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
    let id = node_id.index();

    rsx!(
        rect {
            corner_radius: "7",
            padding: "5",
            background,
            width: "100%",
            height: "27",
            offset_x: "{margin_left}",
            onmousedown,
            onmouseenter,
            onmouseleave,
            label {
                font_size: "14",
                color,
                "{node.tag} ({id})"
            }
        }
    )
}
