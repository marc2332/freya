#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let value = use_state(String::new);
    rect().center().expanded().child(
        Input::new(value)
            .placeholder("Write your message")
            .multiline(true)
            .width(Size::px(300.))
            .height(Size::px(150.)),
    )
}
