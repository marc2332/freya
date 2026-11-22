#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app).with_size(420., 260.)))
}

fn custom_theme(base_radius: f32) -> Theme {
    let mut theme = light_theme();
    theme.set(
        "corner_radius",
        CornerRadiusThemePreference {
            base_radius,
            md: CornerRadiusSize::Relative(1.25),
            ..CornerRadiusThemePreference::default()
        },
    );
    theme
}

fn app() -> impl IntoElement {
    let mut theme = use_init_theme(|| custom_theme(8.));

    rect()
        .theme_background()
        .theme_color()
        .expanded()
        .center()
        .spacing(12.)
        .child(
            rect()
                .horizontal()
                .spacing(8.)
                .child(Button::new().child("Themed medium"))
                .child(Button::new().corner_radius(6.).child("Fixed")),
        )
        .child(
            rect()
                .horizontal()
                .spacing(8.)
                .child(
                    Button::new()
                        .on_press(move |_| theme.set(custom_theme(8.)))
                        .child("8 px base"),
                )
                .child(
                    Button::new()
                        .on_press(move |_| theme.set(custom_theme(16.)))
                        .child("16 px base"),
                ),
        )
}
