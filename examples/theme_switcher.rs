#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let mut theme = use_init_theme(light_theme);
    let is_light = theme.read().name == "light";

    rect()
        .theme_background()
        .theme_color()
        .center()
        .expanded()
        .spacing(6.)
        .child("Switch theme")
        .child(Switch::new().toggled(is_light).on_toggle(move |_| {
            if is_light {
                theme.set(dark_theme());
            } else {
                theme.set(light_theme());
            }
        }))
}
