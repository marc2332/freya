#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "iOS-Styled Notifications", (322.5, 699.0));
}

static WALLPAPER: &[u8] = include_bytes!("./wallpaper.png");

#[component]
fn Notification(title: String, description: String, time: String) -> Element {
    rsx!(
        rect {
            layer: "-9999",
            width: "fill",
            margin: "8",
            corner_radius: "24",
            background: "hsl(0deg, 0%, 65%, 70%)",
            backdrop_blur: "150",

            rect {
                direction: "vertical",
                main_align: "center",
                layer: "-9999",
                width: "100%",
                height: "auto",
                padding: "14",
                corner_radius: "24",
                blend_mode: "color-dodge",
                background: "#333333",
                font_size: "15",
                line_height: "1.3333",
                color: "#000000",

                rect {
                    direction: "horizontal",
                    corner_radius: "9",
                    spacing: "10",
                    padding: "0 8 0 0",

                    rect {
                        width: "38",
                        height: "38",
                        corner_radius: "8",
                        corner_smoothing: "60%",
                        background: "#3D3D3D",
                        blend_mode: "overlay",
                    }

                    rect {
                        direction: "vertical",

                        rect {
                            width: "fill",
                            direction: "horizontal",
                            main_align: "space-between",

                            label {
                                font_weight: "semi-bold",

                                "{title}"
                            }

                            rect {
                                rect {
                                    position: "absolute",
                                    position_top: "0",
                                    position_left: "0",

                                    label {
                                        font_weight: "normal",
                                        color: "hsl(0deg, 0%, 50%, 50%)",

                                        "{time}"
                                    }
                                }

                                label {
                                    font_weight: "normal",
                                    color: "#3D3D3D",
                                    blend_mode: "overlay",
                                    layer: "-9999",

                                    "{time}"
                                }
                            }
                        }

                        label {
                            font_weight: "normal",

                            "{description}"
                        }
                    }
                }
            }
        }
    )
}

fn app() -> Element {
    let image_data = static_bytes(WALLPAPER);

    rsx!(
        rect {
            width: "100%",
            height: "100%",
            background: "#f5f5f5",

            rect {
                position: "absolute",
                position_top: "0",
                position_left: "0",
                cross_align: "center",
                width: "fill",
                height: "fill",

                image {
                    image_data: image_data,
                    width: "fill",
                    height: "fill",
                }
            }

            Notification {
                title: "Notification Title",
                description: "Hello world!",
                time: "9:42 AM",
            }
        }
    )
}
