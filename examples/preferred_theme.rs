#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn get_theme(preferred_theme: PreferredTheme) -> Theme {
    match preferred_theme {
        PreferredTheme::Dark => DARK_THEME,
        PreferredTheme::Light => LIGHT_THEME,
    }
}

fn app() -> Element {
    let preferred_theme = use_preferred_theme();
    let mut current_theme = use_init_theme(|| get_theme(*preferred_theme.peek()));

    let is_dark = current_theme.read().name == "dark";

    use_memo(move || {
        let theme = get_theme(preferred_theme());
        if theme != current_theme() {
            current_theme.set(theme);
        }
    });

    rsx!(
        Body {
            Switch {
                enabled: is_dark,
                ontoggled: move |_| {
                    if is_dark {
                        *current_theme.write() = LIGHT_THEME
                    } else {
                        *current_theme.write() = DARK_THEME
                    }
                }
            }
            label {
                "Current Theme: {current_theme.read().name}"
            }
        }
    )
}
