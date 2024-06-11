use dioxus::prelude::*;
use freya_components::*;
use freya_elements::elements as dioxus_elements;
use freya_hooks::{
    theme_with,
    ScrollViewThemeWith,
};
use freya_native_core::NodeId;

use crate::{
    hooks::use_node_info,
    NodeIdSerializer,
};

#[allow(non_snake_case)]
#[component]
pub fn NodeInspectorLayout(node_id: String) -> Element {
    let node_id = NodeId::deserialize(&node_id);
    let node = use_node_info(node_id)?;

    let inner_area = format!(
        "{}x{}",
        node.layout_node.inner_area.width(),
        node.layout_node.inner_area.height()
    );
    let area = format!(
        "{}x{}",
        node.layout_node.area.width(),
        node.layout_node.area.height()
    );
    let paddings = node.state.size.padding;

    rsx!(
        ScrollView {
            show_scrollbar: true,
            theme: theme_with!(ScrollViewTheme {
                height : "calc(100% - 35)".into(),
            }),
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
                    main_align: "center",
                    cross_align: "center",
                    background: "rgb(40, 40, 40)",
                    rect {
                        width: "100%",
                        height: "100%",
                        background: "rgb(71, 180, 240)",
                        corner_radius: "5",
                        rect {
                            main_align: "center",
                            cross_align: "center",
                            width: "100%",
                            height: "25",
                            label {
                                width: "100%",
                                text_align: "center",
                                "{paddings.top()}"
                            }
                        }
                        rect {
                            width: "100%",
                            height: "calc(100% - 50)",
                            direction: "horizontal",
                            rect {
                                main_align: "center",
                                cross_align: "center",
                                width: "25",
                                height: "100%",
                                label {
                                    width: "100%",
                                    text_align: "center",
                                    "{paddings.left()}"
                                }
                            }
                            rect {
                                width: "calc(100% - 50)",
                                height: "100%",
                                main_align: "center",
                                cross_align: "center",
                                background: "rgb(40, 40, 40)",
                                corner_radius: "5",
                                label {
                                    "{inner_area}"
                                }
                            }
                            rect {
                                main_align: "center",
                                cross_align: "center",
                                width: "25",
                                height: "100%",
                                label {
                                    width: "100%",
                                    text_align: "center",
                                    "{paddings.right()}"
                                }
                            }
                        }
                        rect {
                            main_align: "center",
                            cross_align: "center",
                            width: "100%",
                            height: "25",
                            label {
                                width: "100%",
                                text_align: "center",
                                "{paddings.bottom()}"
                            }
                        }
                    }
                }
            }
        }
    )
}
