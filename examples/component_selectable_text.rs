#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    rect()
        .padding(25.)
        .spacing(10.)
        .font_size(25.)
        .child(label().font_size(35.).text("Select the text from below"))
        .child(SelectableText::new(
            "You can select this looooooooooong text",
        ))
        .child(SelectableText::new("Or this short text :)"))
}
