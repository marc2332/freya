use std::borrow::Cow;

use accesskit::Role;
use freya::prelude::*;
use freya_native_core::prelude::NodeId;

use crate::hooks::use_node_info;

#[component]
pub fn NodeElement(
    node_id: NodeId,
    window_id: u64,
    is_selected: bool,
    is_open: Option<bool>,
    onselected: EventHandler<()>,
    onarrow: EventHandler<()>,
) -> Element {
    let Some(node) = use_node_info(node_id, window_id) else {
        return Ok(VNode::placeholder());
    };

    let onselect = move |_| onselected.call(());

    let onopen = move |e: PressEvent| {
        if is_open.is_some() {
            onarrow.call(());
            e.stop_propagation();
        }
    };

    let margin_left = (node.height * 10) as f32 - 18.;
    let id = node_id.index();

    let role = node.state.accessibility.builder.clone().and_then(|node| {
        let role = node.role();
        if role != Role::GenericContainer {
            Some(role)
        } else {
            None
        }
    });

    let mut theme = theme_with!(ButtonTheme {
        corner_radius: "99".into(),
        width: "100%".into(),
        height: "27".into(),
        border_fill: "none".into()
    });

    if is_selected {
        theme.background = Some(Cow::Borrowed("rgb(40, 40, 40)"));
        theme.hover_background = Some(Cow::Borrowed("rgb(40, 40, 40)"));
    } else {
        theme.background = Some(Cow::Borrowed("none"));
        theme.hover_background = Some(Cow::Borrowed("rgb(45, 45, 45)"));
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
                    width: "25",
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
                                        padding: "6".into(),
                                        background: "none".into(),
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
                paragraph {
                    max_lines: "1",
                    text_overflow: "ellipsis",
                    text {
                        font_size: "14",
                        color: "white",
                        if node.is_window {
                            "Window"
                        } else if let Some(role) = role {
                            "{role:?}"
                        }  else {
                            "{node.tag}"
                        }
                    }
                    text {
                        font_size: "14",
                        color: "rgb(200, 200, 200)",
                        if node.is_window {
                            ", id: {window_id}"
                        } else {
                            ", id: {id}"
                        }
                    }
                }
            }
        }
    )
}
