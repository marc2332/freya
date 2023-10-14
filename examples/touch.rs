#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "touch", (600.0, 500.0));
}

fn app(cx: Scope) -> Element {
    let state = use_state(cx, || "waiting...");

    let ontouchcancel = |_: TouchEvent| {
        state.set("canceled touch");
    };

    let ontouchend = |_: TouchEvent| {
        state.set("ended touch");
    };

    let ontouchmove = |_: TouchEvent| {
        state.set("moved touch");
    };

    let ontouchstart = |_: TouchEvent| {
        state.set("started touch");
    };

    render!(
        rect {
            width: "100%",
            height: "100%",
            main_alignment: "center",
            cross_alignment: "center",
            background: "rgb(35, 35, 35)",
            ontouchcancel: ontouchcancel,
            ontouchend: ontouchend,
            ontouchmove: ontouchmove,
            ontouchstart: ontouchstart,
            label {
                font_size: "75",
                color: "white",
                "{state}"
            }
        }
    )
}
