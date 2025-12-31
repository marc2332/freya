#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let mut theme = use_init_theme(|| LIGHT_THEME);
    let is_light = *theme.read() == LIGHT_THEME;

    rect()
        .theme_background()
        .theme_color()
        .center()
        .expanded()
        .spacing(6.)
        .child("Switch theme")
        .child(Switch::new().toggled(is_light).on_toggle(move |_| {
            if is_light {
                theme.set(DARK_THEME);
            } else {
                theme.set(LIGHT_THEME);
            }
        }))
}
