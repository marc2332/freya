#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Controlled Example", (600.0, 600.0));
}

fn app() -> Element {
    let scroll_controller = use_scroll_controller(|| ScrollConfig {
        direction: "vertical".to_string(),
        initial: ScrollPosition::Bottom,
    });
    rsx!(
        rect {
            height: "fill",
            width: "fill",
            direction: "horizontal",
            ScrollView {
                scroll_controller,
                theme: theme_with!(ScrollViewTheme {
                    width: "50%".into(),
                }),
                Card {}
                Card {}
                Card {}
            }
            ScrollView {
                scroll_controller,
                theme: theme_with!(ScrollViewTheme {
                    width: "50%".into(),
                }),
                Card {}
                Card {}
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
