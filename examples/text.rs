#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let percentage = use_state(cx, || 20.0);

    render!(
        rect {
            width: "100%",
            height: "100%",
            background: "black",
            padding: "100",
            Slider {
                width: 100.0,
                value: *percentage.get(),
                onmoved: |p| {
                    percentage.set(p);
                }
            }
        }
    )
}
