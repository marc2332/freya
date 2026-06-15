#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app).with_size(420., 360.)))
}

fn custom_theme() -> Theme {
    let mut theme = light_theme();
    theme.set(
        "typography",
        TypographyThemePreference {
            title: Preference::Specific(36.0),
            subtitle: Preference::Specific(22.0),
            body: Preference::Specific(16.0),
            caption: Preference::Specific(13.0),
            overline: Preference::Specific(11.0),
        },
    );
    theme
}

fn app() -> impl IntoElement {
    use_init_theme(custom_theme);

    rect()
        .theme_background()
        .theme_color()
        .center()
        .expanded()
        .spacing(8.)
        .child(label().title().text("Title"))
        .child(label().subtitle().text("Subtitle"))
        .child(label().body().text("Body"))
        .child(label().caption().text("Caption"))
        .child(label().overline().text("Overline"))
}
