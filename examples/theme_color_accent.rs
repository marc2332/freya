#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn accent_theme(accent: Color) -> Theme {
    let r = accent.r();
    let g = accent.g();
    let b = accent.b();
    let secondary = Color::from_rgb(
        r.saturating_add(50),
        g.saturating_add(50),
        b.saturating_add(50),
    );
    let tertiary = Color::from_rgb(
        r.saturating_sub(30),
        g.saturating_sub(30),
        b.saturating_sub(30),
    );

    let mut theme = light_theme();
    theme.name = "accent";
    theme.colors = ColorsSheet {
        primary: accent,
        secondary,
        tertiary,
        ..LIGHT_COLORS
    };
    theme
}

fn app() -> impl IntoElement {
    let mut theme = use_init_theme(light_theme);
    let mut accent = use_state(|| Color::from_rgb(103, 80, 164));
    let mut toggled = use_state(|| false);

    rect()
        .theme_background()
        .theme_color()
        .center()
        .expanded()
        .spacing(16.)
        .child("Pick an accent color")
        .child(
            ColorPicker::new(move |color| {
                accent.set(color);
                theme.set(accent_theme(color));
            })
            .value(accent()),
        )
        .child(
            rect()
                .cross_align(Alignment::Center)
                .spacing(8.)
                .child(Button::new().filled().child("Filled button"))
                .child(Button::new().outline().child("Outline button"))
                .child(
                    Switch::new()
                        .toggled(toggled)
                        .on_toggle(move |_| toggled.toggle()),
                )
                .child(Slider::new(|_| {}).value(50.).size(Size::px(150.))),
        )
}
