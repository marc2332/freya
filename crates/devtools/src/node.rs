use std::borrow::Cow;

use accesskit::Role;
use dioxus::prelude::*;
use freya_components::{
    ArrowIcon,
    OutlineButton,
    PressEvent,
};
use freya_elements::elements as dioxus_elements;
use freya_hooks::{
    theme_with,
    ButtonThemeWith,
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
    let node = use_node_info(node_id)?;

    let onselect = move |_| onselected.call(());

    let onopen = move |e: PressEvent| {
        if is_open.is_some() {
            onarrow.call(());
            e.stop_propagation();
        }
    };

    let margin_left = (node.height * 10) as f32 - 20.;
    let id = node_id.index();

    let role = node
        .state
        .accessibility
        .builder
        .clone()
        .and_then(|builder| {
            let built_node = builder.build();
            let role = built_node.role();
            if role != Role::GenericContainer {
                serde_json::to_value(role)
                    .ok()
                    .and_then(|v| v.as_str().map(String::from))
            } else {
                None
            }
        });
    let name = role
        .map(|role| format!("{}, tag: {}", role, node.tag))
        .unwrap_or_else(|| node.tag.to_string());

    let mut theme = theme_with!(ButtonTheme {
        width: "100%".into(),
        height: "27".into(),
        border_fill: "none".into()
    });

    if is_selected {
        theme.background = Some(Cow::Borrowed("rgb(25, 25, 25)"));
        theme.hover_background = Some(Cow::Borrowed("rgb(25, 25, 25)"));
    } else {
        theme.background = Some(Cow::Borrowed("none"));
        theme.hover_background = Some(Cow::Borrowed("rgb(30, 30, 30)"));
    }

    rsx!(
        OutlineButton {
            theme,
            onpress: onselect,
            rect {
                offset_x: "{margin_left}",
                direction: "horizontal",
                width: "fill",
                cross_align: "center",
                rect {
                    width: "20",
                    if let Some(is_open) = is_open {
                        {
                            let arrow_degree = if is_open {
                                0
                            } else {
                                270
                            };
                            rsx!(
                                OutlineButton {
                                    theme: theme_with!(ButtonTheme {
                                        corner_radius: "99".into(),
                                        border_fill: "none".into(),
                                        padding: "2".into(),
                                        background: "none".into(),
                                        hover_background: "none".into(),
                                    }),
                                    onpress: onopen,
                                    ArrowIcon {
                                        fill: "white",
                                        rotate: "{arrow_degree}"
                                    }
                                }
                            )
                        }
                    }
                }
                label {
                    font_size: "14",
                    color: "white",
                    "{name}, id: {id}"
                }
            }
        }
    )
}
