#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let focus_a = use_focus_accessibility(cx);
    let focus_b = use_focus_accessibility(cx);
    let focus_c = use_focus_accessibility(cx);
    let (id_a, attr_a) = use_accessibility(cx);
    let (id_b, attr_b) = use_accessibility(cx);
    let (id_c, attr_c) = use_accessibility(cx);
    render!(
        rect {
            accessibility_id: attr_a,
            background: "rgb(233, 196, 106)",
            padding: "25",
            width: "50%",
            height: "50%",
            onclick: move |_| {
                focus_a(id_a);
            },
            label {
                accessibility_id: attr_c,
                onclick: move |_| {
                    focus_c(id_c);
                },
                "test"
            }
        }
        rect {
            accessibility_id: attr_b,
            background: "rgb(150, 100, 231)",
            padding: "25",
            width: "50%",
            height: "50%",
            onclick: move |_| {
                focus_b(id_b);
            }
        }
    )
}
