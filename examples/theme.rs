#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use components::*;
use dioxus::prelude::*;
use elements_namespace as dioxus_elements;
use fermi::*;

use freya::launch;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let theme = use_atom_ref(&cx, THEME);

    cx.render(rsx!(rect {
        height: "100%",
        width: "100%",
        Button {
            on_click: |_| {
                *theme.write() = LIGHT_THEME;
            },
            label {
                width: "100",
                "Light"
            }
        }
        Button {
            on_click: |_| {
                *theme.write() = DARK_THEME;
            },
            label {
                width: "100",
                "Dark"
            }
        }
    }))
}
