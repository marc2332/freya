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
fn Child(cx: Scope) -> Element {
    let (focused, focus) = use_focus(&cx);
    render!(
        Button {
            on_click: move |_| {
                focus();
            }
            label {
                "Am I focused? {focused}"
            }
        }
    )
}

fn app(cx: Scope) -> Element {
    use_init_focus(&cx);
    render!(
        container {
            width: "100%",
            height: "100%",
            Child {},
            Child {},
            Child {},
            Child {},
            Child {},
        }
    )
}
