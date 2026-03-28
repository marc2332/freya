#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let mut show = use_state(|| false);

    rect().center().expanded().child(
        Attached::new(
            Button::new()
                .child("Toggle")
                .on_press(move |_| show.toggle()),
        )
        .maybe_child(show().then(|| rect().child(Button::new().child("Attached")))),
    )
}
