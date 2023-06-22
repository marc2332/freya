use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_node_state::{Border, Shadow};
use skia_safe::{textlayout::TextShadow, Color};

#[allow(non_snake_case)]
#[inline_props]
pub fn Property<'a>(cx: Scope<'a>, name: &'a str, value: String) -> Element<'a> {
    render!(
        container {
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
pub fn ColorfulProperty<'a>(cx: Scope<'a>, name: &'a str, color: &'a Color) -> Element<'a> {
    let color = color.to_rgb();
    render!(
        container {
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
                radius: "5",
                background: "white",
                padding: "2.5",
                rect {
                    radius: "3",
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

#[allow(non_snake_case)]
#[inline_props]
pub fn ShadowProperty<'a>(cx: Scope<'a>, name: &'a str, shadow: &'a Shadow) -> Element<'a> {
    let color = shadow.color.to_rgb();
    render!(
        container {
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
                radius: "5",
                background: "white",
                padding: "2.5",
                rect {
                    radius: "3",
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

#[allow(non_snake_case)]
#[inline_props]
pub fn BorderProperty<'a>(cx: Scope<'a>, name: &'a str, border: &'a Border) -> Element<'a> {
    let color = border.color.to_rgb();
    render!(
        container {
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
                radius: "5",
                background: "white",
                padding: "2.5",
                rect {
                    radius: "3",
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

#[allow(non_snake_case)]
#[inline_props]
pub fn TextShadowProperty<'a>(
    cx: Scope<'a>,
    name: &'a str,
    text_shadow: &'a TextShadow,
) -> Element<'a> {
    let color = text_shadow.color.to_rgb();
    render!(
        container {
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
                radius: "5",
                background: "white",
                padding: "2.5",
                rect {
                    radius: "3",
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
