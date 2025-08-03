use freya::prelude::*;
use freya_native_core::NodeId;

use crate::hooks::use_node_info;

#[component]
pub fn NodeInspectorComputedLayout(node_id: NodeId, window_id: u64) -> Element {
    let Some(node) = use_node_info(node_id, window_id) else {
        return Ok(VNode::placeholder());
    };

    let inner_area = format!(
        "{}x{}",
        node.layout_node.inner_area.width().round(),
        node.layout_node.inner_area.height().round()
    );
    let area = format!(
        "{}x{}",
        node.layout_node.area.width().round(),
        node.layout_node.area.height().round()
    );
    let paddings = node.state.layout.padding;
    let margins = node.state.layout.margin;

    rsx!(
        ScrollView {
            show_scrollbar: true,
            rect {
                padding: "20",
                cross_align: "center",
                width: "100%",
                rect {
                    width: "100%",
                    max_width: "300",
                    label {
                        height: "25",
                        "Area: {area}"
                    }
                    rect {
                        width: "100%",
                        height: "250",
                        main_align: "center",
                        cross_align: "center",
                        background: "rgb(197, 46, 139)",
                        corner_radius: "5",
                        content: "flex",
                        TooltipContainer {
                            tooltip: rsx!(
                                Tooltip {
                                    text: "Top margin"
                                }
                            ),
                            label {
                                main_align: "center",
                                text_align: "center",
                                width: "100%",
                                height: "25",
                                "{margins.top()}"
                            }
                        }
                        rect {
                            direction: "horizontal",
                            content: "flex",
                            height: "flex(1)",
                            width: "100%",
                            cross_align: "center",
                            TooltipContainer {
                                tooltip: rsx!(
                                    Tooltip {
                                        text: "Left margin"
                                    }
                                ),
                                label {
                                    main_align: "center",
                                    text_align: "center",
                                    width: "25",
                                    height: "25",
                                    "{margins.left()}"
                                }
                            }

                            rect {
                                width: "flex(1)",
                                height: "200",
                                main_align: "center",
                                cross_align: "center",
                                background: "rgb(71, 180, 240)",
                                corner_radius: "5",
                                content: "flex",
                                TooltipContainer {
                                    tooltip: rsx!(
                                        Tooltip {
                                            text: "Top padding"
                                        }
                                    ),
                                    label {
                                        main_align: "center",
                                        text_align: "center",
                                        width: "100%",
                                        height: "25",
                                        "{paddings.top()}"
                                    }
                                }
                                rect {
                                    direction: "horizontal",
                                    content: "flex",
                                    height: "flex(1)",
                                    cross_align: "center",
                                    TooltipContainer {
                                        tooltip: rsx!(
                                            Tooltip {
                                                text: "Left padding"
                                            }
                                        ),
                                        label {
                                            main_align: "center",
                                            text_align: "center",
                                            width: "25",
                                            height: "25",
                                            "{paddings.left()}"
                                        }
                                    }

                                    rect {
                                        width: "flex(1)",
                                        height: "fill",
                                        main_align: "center",
                                        cross_align: "center",
                                        background: "rgb(40, 40, 40)",
                                        corner_radius: "5",
                                        TooltipContainer {
                                            tooltip: rsx!(
                                                Tooltip {
                                                    text: "Inner area"
                                                }
                                            ),
                                            label {
                                                "{inner_area}"
                                            }
                                        }
                                    }
                                    TooltipContainer {
                                        tooltip: rsx!(
                                            Tooltip {
                                                text: "Right padding"
                                            }
                                        ),
                                        label {
                                            main_align: "center",
                                            text_align: "center",
                                            width: "25",
                                            height: "25",
                                            "{paddings.right()}"
                                        }
                                    }

                                }
                                TooltipContainer {
                                    tooltip: rsx!(
                                        Tooltip {
                                            text: "Bottom padding"
                                        }
                                    ),
                                    label {
                                        main_align: "center",
                                        text_align: "center",
                                        width: "100%",
                                        height: "25",
                                        "{paddings.bottom()}"
                                    }
                                }

                            }
                            TooltipContainer {
                                tooltip: rsx!(
                                    Tooltip {
                                        text: "Right margin"
                                    }
                                ),
                                label {
                                    main_align: "center",
                                    text_align: "center",
                                    width: "25",
                                    height: "25",
                                    "{margins.right()}"
                                }
                            }

                        }
                        TooltipContainer {
                            tooltip: rsx!(
                                Tooltip {
                                    text: "Bottom margin"
                                }
                            ),
                            label {
                                main_align: "center",
                                text_align: "center",
                                width: "100%",
                                height: "25",
                                "{margins.bottom()}"
                            }
                        }

                    }
                }
            }
        }
    )
}
