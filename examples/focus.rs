#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

#[allow(non_snake_case)]
fn Child(cx: Scope) -> Element {
    let (focused, focus) = use_focus(cx);
    render!(
        Button {
            onclick: move |_| {
                focus();
            }
            label {
                "Am I focused? {focused}"
            }
        }
    )
}

fn app(cx: Scope) -> Element {
    use_init_focus(cx);
    render!(
       rect {
           overflow: "clip",
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
