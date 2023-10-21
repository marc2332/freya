use dioxus::prelude::*;
use dioxus_native_core::NodeId;
use freya_components::*;
use freya_elements::elements as dioxus_elements;

use crate::{hooks::use_selected_node, NodeInspectorBar};

#[allow(non_snake_case)]
#[component]
pub fn NodeInspectorLayout(cx: Scope, node_id: NodeId) -> Element {
    let node = use_selected_node(cx, &cx.props.node_id);

    if let Some(node) = node {
        let inner_area = format!(
            "{}x{}",
            node.areas.inner_area.width(),
            node.areas.inner_area.height()
        );
        let area = format!("{}x{}", node.areas.area.width(), node.areas.area.height());
        let paddings = node.state.size.padding;

        render!(
            rect {
                overflow: "clip",
                width: "100%",
                height: "50%",
                NodeInspectorBar {
                    node_id: *node_id
                }
                ScrollView {
                    show_scrollbar: true,
                    height: "calc(100% - 35)",
                    width: "100%",
                    rect {
                        width: "100%",
                        height: "200",
                        padding: "20",
                        label {
                            height: "25",
                            "Area: {area}"
                        }
                        rect {
                            width: "100%",
                            height: "calc(100% - 25)",
                            display: "center",
                            direction: "both",
                            background: "rgb(40, 40, 40)",
                            rect {
                                width: "100%",
                                height: "100%",
                                background: "rgb(71, 180, 240)",
                                corner_radius: "5",
                                rect {
                                    direction: "both",
                                    display: "center",
                                    width: "100%",
                                    height: "25",
                                    label {
                                        width: "100%",
                                        align: "center",
                                        "{paddings.top()}"
                                    }
                                }
                                rect {
                                    width: "100%",
                                    height: "calc(100% - 50)",
                                    direction: "horizontal",
                                    rect {
                                        direction: "vertical",
                                        display: "center",
                                        width: "25",
                                        height: "100%",
                                        label {
                                            width: "100%",
                                            align: "center",
                                            "{paddings.left()}"
                                        }
                                    }
                                    rect {
                                        width: "calc(100% - 50)",
                                        height: "100%",
                                        display: "center",
                                        direction: "both",
                                        background: "rgb(40, 40, 40)",
                                        corner_radius: "5",
                                        label {
                                            "{inner_area}"
                                        }
                                    }
                                    rect {
                                        direction: "vertical",
                                        display: "center",
                                        width: "25",
                                        height: "100%",
                                        label {
                                            width: "100%",
                                            align: "center",
                                            "{paddings.right()}"
                                        }
                                    }
                                }
                                rect {
                                    direction: "both",
                                    display: "center",
                                    width: "100%",
                                    height: "25",
                                    label {
                                        width: "100%",
                                        align: "center",
                                        "{paddings.bottom()}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        )
    } else {
        None
    }
}
