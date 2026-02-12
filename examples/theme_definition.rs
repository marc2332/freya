#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

pub const CUSTOM_THEME: Theme = Theme {
    name: "custom",
    colors: ColorsSheet {
        primary: Color::from_rgb(37, 52, 63),
        secondary: Color::from_rgb(255, 155, 81),
        tertiary: Color::from_rgb(81, 155, 255),
        ..DARK_THEME.colors
    },
    ..DARK_THEME
};

fn app() -> impl IntoElement {
    use_init_theme(|| CUSTOM_THEME);
    let mut toggled = use_state(|| false);

    rect()
        .theme_background()
        .theme_color()
        .center()
        .expanded()
        .child(
            Switch::new()
                .toggled(toggled)
                .on_toggle(move |_| toggled.with_mut(|mut v| *v = !*v)),
        )
}
