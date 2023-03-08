#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fmt::Display;

use freya::prelude::*;

fn main() {
    launch(app);
}

#[derive(PartialEq, Clone)]
enum Values {
    A,
    B,
    C,
}

impl Display for Values {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Values::A => f.write_str("Value A"),
            Values::C => f.write_str("Value C"),
            Values::B => f.write_str("Value B"),
        }
    }
}

fn app(cx: Scope) -> Element {
    render!(
        ThemeProvider {
            Body {

            }
        }
    )
}

#[allow(non_snake_case)]
fn Body(cx: Scope) -> Element {
    let selected_dropdown = use_state(cx, || Values::A);
    let theme = use_theme(cx);
    let body_theme = &theme.read().body;

    render!(
        rect {
            background: "{body_theme.background}",
            padding: "7.5",
            width: "100%",
            height: "100%",
            Dropdown {
                value: selected_dropdown.get().clone(),
                DropdownItem { onclick: move |_| selected_dropdown.set(Values::A), value: Values::A, label { "A" } }
                DropdownItem { onclick: move |_| selected_dropdown.set(Values::B), value: Values::B, label { "B" } }
                DropdownItem { onclick: move |_| selected_dropdown.set(Values::C), value: Values::C, label { "C" } }
            }
        }
    )
}
