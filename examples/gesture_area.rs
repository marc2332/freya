#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let gesture = use_state(cx, || "Tap here".to_string());
    render!(
        GestureArea {
            ongesture: move |g| gesture.set(format!("{g:?}")),
            rect {
                width: "100%",
                height: "100%",
                direction: "vertical",
                main_align: "center",
                label {
                    align: "center",
                    width: "100%",
                    font_size: "70",
                    "{gesture}"
                }
            }
        }
    )
}
