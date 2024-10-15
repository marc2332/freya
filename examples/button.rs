#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Button", (400.0, 350.0));
}

fn app() -> Element {
    rsx!(
        Body {
            rect {
                width: "fill",
                height: "fill",
                spacing: "10",
                main_align: "center",
                cross_align: "center",
                Button {
                    onclick: move |_| println!("Button Clicked!"),
                    label { "Button A" }
                }
                FilledButton {
                    onpress: move |_| println!("Button Pressed!"),
                    label { "Button B" }
                }
                OutlineButton {
                    label { "Button C" }
                }
            }
        }
    )
}
