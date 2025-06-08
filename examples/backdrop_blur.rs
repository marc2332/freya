#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "iOS-Styled Notifications", (350., 700.));
}

static WALLPAPER: &[u8] = include_bytes!("./wallpaper.png");

#[component]
fn Notification(title: String, description: String, time: String) -> Element {
    rsx!(
        rect {
            main_align: "center",
            width: "fill",
            padding: "14",
            corner_radius: "24",
            background: "rgb(255, 255, 255, 0.5)",
            font_size: "15",
            color: "#000000",
            backdrop_blur: "6",

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
                    background: "rgb(170, 170, 170, 0.4)",
                }

                rect {
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
                                color: "#000000",

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
    )
}

fn app() -> Element {
    let image_data = static_bytes(WALLPAPER);

    rsx!(
        image {
            position: "absolute",
            position_top: "0",
            position_left: "0",
            image_data,
            aspect_ratio: "max",
        }
        rect {
            padding: "6",
            spacing: "6",
            for i in 0..4 {
                Notification {
                    key: "{i}",
                    title: "Notification Title",
                    description: "Hello world!",
                    time: "9:42 AM",
                }
            }
        }
    )
}
