#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Dynamic theme", (400.0, 350.0));
}

fn app() -> Element {
    let mut brightness = use_signal(|| 50);
    let text_brightness = 100 - *brightness.read();

    rsx!(
        rect {
            height: "50%",
            width: "100%",
            main_align: "center",
            cross_align: "center",
            background: "rgb(0, 119, 182)",
            color: "white",
            shadow: "0 4 20 5 rgb(0, 0, 0, 80)",
            label {
                font_size: "25",
                font_weight: "bold",
                "Background brightness: {brightness}"
            }
            label {
                font_size: "25",
                font_weight: "bold",
                "Text brightness: {text_brightness}"
            }
        }
        rect {
            height: "50%",
            width: "100%",
            main_align: "center",
            cross_align: "center",
            direction: "horizontal",
            Button {
                theme: theme_with!(ButtonTheme {
                    hover_background: format!("hsl(100deg, 100%, {}%)", brightness - 10).into(),
                    background: format!("hsl(100deg, 100%, {}%)", brightness).into(),
                    font_theme: theme_with!(FontTheme {
                        color: format!("hsl(0deg, 0%, {}%)", text_brightness).into(),
                    }),
                }),
                onclick: move |_| {
                    if *brightness.read() < 100 {
                        brightness += 10;
                    }
                },
                label {
                    "Increase background brightness"
                }
            }
            Button {
                theme: theme_with!(ButtonTheme {
                    hover_background: format!("hsl(60deg, 100%, {}%)", brightness - 10).into(),
                    background: format!("hsl(60deg, 100%, {}%)", brightness).into(),
                    font_theme: theme_with!(FontTheme {
                        color: format!("hsl(0deg, 0%, {}%)", text_brightness).into(),
                    }),
                }),
                onclick: move |_| {
                    if *brightness.read() > 0 {
                        brightness -= 10;
                    }
                },
                label {
                    "Decrease background brightness"
                }
            }
        }
    )
}
