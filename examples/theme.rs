#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus::prelude::*;
use freya::{dioxus_elements, *};

fn main() {
    launch(app);
}

pub fn use_init_theme(cx: &ScopeState, theme: Theme) -> Theme {
    use_context_provider(cx, || theme.clone());
    theme
}

pub fn use_init_default_theme(cx: &ScopeState) -> Theme {
    use_context_provider(cx, || DARK_THEME);
    DARK_THEME
}

pub fn use_theme(cx: &ScopeState) -> UseSharedState<Theme> {
    use_context::<Theme>(cx).unwrap()
}

#[allow(non_snake_case)]
fn Comp(cx: Scope) -> Element {
    let theme = use_theme(&cx);

    let is_enabled = *theme.read() == DARK_THEME;

    render!(Switch {
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

fn app(cx: Scope) -> Element {
    use_init_theme(&cx, DARK_THEME);
    let enabled = use_state(&cx, || false);

    render!(
        Switch {
            enabled: *enabled.get(),
            ontoggled: |_| {
                enabled.set(!enabled.get());
            }
        }
        Comp {

        }
    )
}
