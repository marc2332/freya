use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_engine::prelude::*;
use freya_node_state::{Border, Fill, Shadow};

#[allow(non_snake_case)]
#[inline_props]
pub fn Property<'a>(cx: Scope<'a>, name: &'a str, value: String) -> Element<'a> {
    render!(
        rect {
            overflow: "clip",
            height: "30",
            width: "100%",
            direction: "horizontal",
            padding: "10",
            paragraph {
                width: "100%",
                text {
                    font_size: "15",
                    color: "rgb(71, 180, 240)",
                    "{name}"
                }
                text {
                    font_size: "15",
                    color: "rgb(215, 215, 215)",
                    ": "
                }
                text {
                    font_size: "15",
                    color: "rgb(252,181,172)",
                    "{value}"
                }
            }
        }
    )
}

#[allow(non_snake_case)]
#[inline_props]
pub fn LinearGradientProperty<'a>(cx: Scope<'a>, name: &'a str, fill: Fill) -> Element<'a> {
    render!(
        rect {
            padding: "5 10",
            paragraph {
                line_height: "1.9",
                text {
                    font_size: "15",
                    color: "rgb(71, 180, 240)",
                    "{name}"
                }
                text {
                    font_size: "15",
                    color: "rgb(215, 215, 215)",
                    ": "
                }
                text {
                    font_size: "15",
                    color: "rgb(252,181,172)",
                    "{fill}",
                }
            }
        }
    )
}

#[allow(non_snake_case)]
#[inline_props]
pub fn ColorProperty<'a>(cx: Scope<'a>, name: &'a str, fill: Fill) -> Element<'a> {
    render!(
        rect {
            overflow: "clip",
            height: "30",
            width: "100%",
            direction: "horizontal",
            padding: "10",
            label {
                font_size: "15",
                color: "rgb(71, 180, 240)",
                "{name}"
            }
            label {
                font_size: "15",
                color: "rgb(215, 215, 215)",
                ": "
            }
            rect {
                width: "5"
            }
            rect {
                width: "17",
                height: "17",
                corner_radius: "5",
                background: "white",
                padding: "2.5",
                rect {
                    corner_radius: "3",
                    width: "100%",
                    height: "100%",
                    background: "{fill}",
                }
            }
            rect {
                width: "5"
            }
            label {
                font_size: "15",
                color: "rgb(252,181,172)",
                "{fill}",
            }
        }
    )
}

#[allow(non_snake_case)]
#[inline_props]
pub fn ShadowProperty<'a>(cx: Scope<'a>, name: &'a str, shadow: Shadow) -> Element<'a> {
    render!(
        rect {
            overflow: "clip",
            height: "30",
            width: "100%",
            direction: "horizontal",
            padding: "10",
            paragraph {
                text {
                    font_size: "15",
                    color: "rgb(71, 180, 240)",
                    "{name}"
                }
                text {
                    font_size: "15",
                    color: "rgb(215, 215, 215)",
                    ": "
                }
                text {
                    font_size: "15",
                    color: "rgb(252,181,172)",
                    "{shadow.position:?} {shadow.x} {shadow.y} {shadow.blur} {shadow.spread}"
                }
            }
            rect {
                width: "5"
            }
            rect {
                width: "17",
                height: "17",
                corner_radius: "5",
                background: "white",
                padding: "2.5",
                rect {
                    corner_radius: "3",
                    width: "100%",
                    height: "100%",
                    background: "{shadow.fill}",
                }
            }
            rect {
                width: "5"
            }
            label {
                font_size: "15",
                color: "rgb(252,181,172)",

                "{shadow.fill}"
            }
        }
    )
}

#[allow(non_snake_case)]
#[inline_props]
pub fn BorderProperty<'a>(cx: Scope<'a>, name: &'a str, border: Border) -> Element<'a> {
    render!(
        rect {
            overflow: "clip",
            height: "30",
            width: "100%",
            direction: "horizontal",
            padding: "10",
            paragraph {
                text {
                    font_size: "15",
                    color: "rgb(71, 180, 240)",
                    "{name}"
                }
                text {
                    font_size: "15",
                    color: "rgb(215, 215, 215)",
                    ": "
                }
                text {
                    font_size: "15",
                    color: "rgb(252,181,172)",
                    "{border.width} {border.style:?} {border.alignment:?}"
                }
            }
            rect {
                width: "5"
            }
            rect {
                width: "17",
                height: "17",
                corner_radius: "5",
                background: "white",
                padding: "2.5",
                rect {
                    corner_radius: "3",
                    width: "100%",
                    height: "100%",
                    background: "{border.fill}",
                }
            }
            rect {
                width: "5"
            }
            label {
                font_size: "15",
                color: "rgb(252,181,172)",
                "{border.fill}"
            }
        }
    )
}

#[allow(non_snake_case)]
#[inline_props]
pub fn TextShadowProperty<'a>(
    cx: Scope<'a>,
    name: &'a str,
    text_shadow: TextShadow,
) -> Element<'a> {
    let color = text_shadow.color.to_rgb();
    render!(
        rect {
            overflow: "clip",
            height: "30",
            width: "100%",
            direction: "horizontal",
            padding: "10",
            paragraph {
                text {
                    font_size: "15",
                    color: "rgb(71, 180, 240)",
                    "{name}"
                }
                text {
                    font_size: "15",
                    color: "rgb(215, 215, 215)",
                    ": "
                }
                text {
                    font_size: "15",
                    color: "rgb(252,181,172)",
                    "{text_shadow.offset.x} {text_shadow.offset.y} {text_shadow.blur_sigma}"
                }
            }
            rect {
                width: "5"
            }
            rect {
                width: "17",
                height: "17",
                corner_radius: "5",
                background: "white",
                padding: "2.5",
                rect {
                    corner_radius: "3",
                    width: "100%",
                    height: "100%",
                    background: "rgb({color.r}, {color.g}, {color.b})",
                }
            }
            rect {
                width: "5"
            }
            label {
                font_size: "15",
                color: "rgb(252,181,172)",

                "rgb({color.r}, {color.g}, {color.b})"
            }
        }
    )
}
