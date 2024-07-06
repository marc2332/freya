#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Scroll example", (400.0, 400.0));
}

fn app() -> Element {
    rsx!(
        rect {
            height: "fill",
            width: "fill",
            ScrollView {
                theme: theme_with!(ScrollViewTheme {
                    height: "50%".into(),
                }),
                Card {}
                Card {}
                Card {}
            }
            ScrollView {
                direction: "horizontal",
                theme: theme_with!(ScrollViewTheme {
                    height: "50%".into(),
                }),
                Card {},
                Card {},
                Card {}
            }
        }
    )
}

#[component]
fn Card() -> Element {
    rsx!(
        rect {
            border: "15 solid rgb(43,106,208)",
            height: "220",
            width: "420",
            background: "white",
            padding: "25",
            label {  "Scroll..." }
        }
    )
}
