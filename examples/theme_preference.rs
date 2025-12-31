#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app).with_size(500., 450.)))
}

fn app() -> impl IntoElement {
    let mut theme = use_init_root_theme(|| Platform::get().preferred_theme.read().to_theme());

    use_side_effect(move || theme.set(Platform::get().preferred_theme.read().to_theme()));

    rect()
        .center()
        .expanded()
        .theme_background()
        .child(Button::new().child("Change the theme in your OS"))
}
