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
        Button {
            onclick: move |_| println!("Button Clicked!"),
            label { "Button A" }
        }
        Button {
            onpress: move |_| println!("Button Pressed!"),
            label { "Button B" }
        }
        Button {
            label { "Button C" }
        }
    )
}
