#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app).with_size(420., 360.)))
}

fn custom_theme(base_size: f32) -> Theme {
    let mut theme = light_theme();
    TypographyScale {
        base_size,
        title: TypographySize::Fixed(36.),
        subtitle: TypographySize::Relative(1.25),
        ..TypographyScale::default()
    }
    .apply_to(&mut theme.variables);
    theme
}

fn app() -> impl IntoElement {
    let mut theme = use_init_theme(|| custom_theme(18.));

    rect()
        .theme_background()
        .theme_color()
        .theme_font_size(TextRole::Content)
        .center()
        .expanded()
        .spacing(8.)
        .child(label().title().text("Fixed title"))
        .child(label().subtitle().text("Subtitle"))
        .child(label().body().text("Body"))
        .child(label().caption().text("Caption"))
        .child(label().overline().text("Overline"))
        .child("Inherited content size")
        .child(
            rect()
                .horizontal()
                .spacing(8.)
                .child(
                    Button::new()
                        .on_press(move |_| theme.set(custom_theme(18.)))
                        .child("18 px base"),
                )
                .child(
                    Button::new()
                        .on_press(move |_| theme.set(custom_theme(24.)))
                        .child("24 px base"),
                ),
        )
}
