use freya::prelude::*;
use freya_core::values::{
    Border,
    Fill,
    Shadow,
    TextShadow,
};

#[component]
pub fn Property(name: String, value: String) -> Element {
    rsx!(
        rect {
            overflow: "clip",
            width: "100%",
            direction: "horizontal",
            cross_align: "center",
            paragraph {
                width: "100%",
                text {
                    font_size: "15",
                    color: "rgb(102, 163, 217)",
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

#[component]
pub fn GradientProperty(name: String, fill: Fill) -> Element {
    rsx!(
        paragraph {
            line_height: "1.9",
            text {
                font_size: "15",
                color: "rgb(102, 163, 217)",
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
    )
}

#[component]
pub fn ColorProperty(name: String, fill: Fill) -> Element {
    rsx!(
        rect {
            overflow: "clip",
            width: "100%",
            direction: "horizontal",
            cross_align: "center",
            label {
                font_size: "15",
                color: "rgb(102, 163, 217)",
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
                "{fill}"
            }
        }
    )
}

#[component]
pub fn ShadowProperty(name: String, shadow: Shadow) -> Element {
    rsx!(
        rect {
            overflow: "clip",
            width: "100%",
            direction: "horizontal",
            cross_align: "center",
            paragraph {
                text {
                    font_size: "15",
                    color: "rgb(102, 163, 217)",
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

#[component]
pub fn BorderProperty(name: String, border: Border) -> Element {
    rsx!(
        rect {
            overflow: "clip",
            width: "100%",
            direction: "horizontal",
            cross_align: "center",
            paragraph {
                text {
                    font_size: "15",
                    color: "rgb(102, 163, 217)",
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
                    "{border.width} {border.alignment:?}"
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

#[component]
pub fn TextShadowProperty(name: String, text_shadow: TextShadow) -> Element {
    let color = text_shadow.color.to_rgb();
    rsx!(
        rect {
            overflow: "clip",
            width: "100%",
            direction: "horizontal",
            cross_align: "center",
            paragraph {
                text {
                    font_size: "15",
                    color: "rgb(102, 163, 217)",
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
                    "{text_shadow.offset.0} {text_shadow.offset.1} {text_shadow.blur_sigma}"
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
