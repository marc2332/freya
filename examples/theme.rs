#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus::prelude::*;
use freya::{dioxus_elements, *};

fn main() {
    launch(app);
}

#[allow(non_snake_case)]
fn TheOtherSwitch(cx: Scope) -> Element {
    let theme = use_theme(&cx);

    let is_enabled = theme.read().name == "dark";

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
    let enabled = use_state(&cx, || true);

    render!(
        ThemeProvider {
            theme: LIGHT_THEME,
            Switch {
                enabled: *enabled.get(),
                ontoggled: |_| {
                    enabled.set(!enabled.get());
                }
            }
            TheOtherSwitch { }
        }
    )
}
