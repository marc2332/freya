#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

#[allow(non_snake_case)]
fn Child() -> Element {
    let mut focus_manager = use_focus();
    let is_focused = focus_manager.is_focused();
    rsx!(
        rect {
            width: "100%",
            height: "60",
            focus_id: focus_manager.attribute(),
            background: "rgb(45, 45, 45)",
            padding: "10",
            color: "white",
            onclick: move |_| {
                focus_manager.focus();
            },
            label {
                "Am I focused? {is_focused}"
            }
        }
    )
}

fn app() -> Element {
    rsx!(
        rect {
            width: "100%",
            height: "100%",
            Child {},
            Child {},
            Child {},
            Child {},
            Child {}
        }
    )
}
