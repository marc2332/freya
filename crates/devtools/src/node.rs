use dioxus::prelude::*;
use freya_components::{
    ArrowIcon,
    ButtonStatus,
};
use freya_elements::{
    elements as dioxus_elements,
    events::MouseEvent,
};
use freya_native_core::prelude::NodeId;

use crate::hooks::use_node_info;

#[allow(non_snake_case)]
#[component]
pub fn NodeElement(
    node_id: NodeId,
    is_selected: bool,
    is_open: Option<bool>,
    onselected: EventHandler<()>,
    onarrow: EventHandler<()>,
) -> Element {
    let mut status = use_signal(ButtonStatus::default);
    let node = use_node_info(node_id)?;

    let onmousedown = move |_| onselected.call(());

    let onarrowmousedown = move |e: MouseEvent| {
        if is_open.is_some() {
            onarrow.call(());
            e.stop_propagation();
        }
    };

    let onmouseenter = move |_| {
        status.set(ButtonStatus::Hovering);
    };

    let onmouseleave = move |_| {
        status.set(ButtonStatus::default());
    };

    let background = match *status.read() {
        _ if is_selected => "rgb(100, 100, 100)",
        ButtonStatus::Idle => "transparent",
        ButtonStatus::Hovering => "rgb(80, 80, 80)",
    };

    let margin_left = (node.height * 10) as f32 - 20.;
    let id = node_id.index();

    rsx!(
        rect {
            corner_radius: "7",
            padding: "5 5 5 0",
            background,
            width: "100%",
            height: "27",
            offset_x: "{margin_left}",
            onmousedown,
            onmouseenter,
            onmouseleave,
            direction: "horizontal",
            cross_align: "center",
            rect {
                onmousedown: onarrowmousedown,
                margin: "0 8 0 0",
                if let Some(is_open) = is_open {
                    {
                        let arrow_degree = if is_open {
                            0
                        } else {
                            270
                        };
                        rsx!(
                            ArrowIcon {
                                fill: "white",
                                rotate: "{arrow_degree}"
                            }
                        )
                    }
                }
            }
            label {
                font_size: "14",
                color: "white",
                "{node.tag} ({id})"
            }
        }
    )
}
