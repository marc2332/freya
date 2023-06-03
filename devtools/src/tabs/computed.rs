use dioxus::prelude::*;
use freya_components::*;
use freya_elements::elements as dioxus_elements;

use crate::{NodeInspectorBar, TreeNode};

#[allow(non_snake_case)]
#[inline_props]
pub fn NodeInspectorComputed<'a>(cx: Scope<'a>, node: &'a TreeNode) -> Element<'a> {
    let inner_area = format!(
        "{}x{}",
        node.areas.inner_area.width(),
        node.areas.inner_area.height()
    );
    let area = format!("{}x{}", node.areas.area.width(), node.areas.area.height());
    let paddings = node.state.size.padding;

    render!(
        container {
            width: "100%",
            height: "50%",
            NodeInspectorBar { }
            ScrollView {
                show_scrollbar: true,
                height: "calc(100% - 35)",
                width: "100%",
                rect {
                    width: "100%",
                    height: "200",
                    padding: "5",
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
}
