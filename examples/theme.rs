#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus::prelude::*;
use fermi::*;
use freya::{dioxus_elements, *};

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let theme = use_atom_ref(&cx, THEME);
    let enabled = use_state(&cx, || false);

    render!(
        rect {
            height: "100%",
            width: "100%",
            Button {
                on_click: |_| {
                    *theme.write() = LIGHT_THEME.clone();
                },
                label {
                    width: "100",
                    "Light"
                }
            }
            Button {
                on_click: |_| {
                    *theme.write() = DARK_THEME.clone();
                },
                label {
                    width: "100",
                    "Dark"
                }
            }
            Switch {
                enabled: enabled,
                ontoggled: |_| {
                    enabled.set(!enabled.get());
                }
            }
        }
    )
}
