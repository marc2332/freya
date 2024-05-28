#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
use freya_core::types::PreferredTheme;

fn main() {
    launch(app);
}

#[allow(non_snake_case)]
fn TheOtherSwitch() -> Element {
    let mut theme = use_theme();

    let is_enabled = theme.read().name == "dark";

    rsx!(Switch {
        enabled: is_enabled,
        ontoggled: move |_| {
            if is_enabled {
                *theme.write() = LIGHT_THEME
            } else {
                *theme.write() = DARK_THEME
            }
        }
    })
}

fn app() -> Element {
    let mut current_theme = use_init_default_theme();
    let preferred_theme = use_preferred_theme();
    let mut enabled = use_signal(|| true);

    let is_enabled = if *enabled.read() { "Yes" } else { "No" };

    use_memo(move || {
        let theme = match preferred_theme() {
            PreferredTheme::Dark => DARK_THEME,
            PreferredTheme::Light => LIGHT_THEME,
        };
        current_theme.set(theme);
    });

    rsx!(
        Body {
            theme: theme_with!(BodyTheme {
                padding: "20".into(),
            }),
            Switch {
                enabled: *enabled.read(),
                ontoggled: move |_| {
                    enabled.toggle();
                }
            }
            label {
                "Is enabled? {is_enabled}"
            }
            TheOtherSwitch { }
        }
    )
}
