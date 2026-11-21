#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let toggled = use_state(|| false);
    rect()
        .center()
        .expanded()
        .spacing(6.)
        .child(Switch::new(toggled))
        .child(Switch::new(toggled).enabled(false))
        .child(Switch::new(toggled).expanded())
        .child(Switch::new(toggled).expanded().enabled(false))
}
