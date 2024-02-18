#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "touch", (600.0, 500.0));
}

fn app() -> Element {
    let mut state = use_signal(|| "waiting...");

    let ontouchcancel = move |_: TouchEvent| {
        state.set("canceled touch");
    };

    let ontouchend = move |_: TouchEvent| {
        state.set("ended touch");
    };

    let ontouchmove = move |_: TouchEvent| {
        state.set("moved touch");
    };

    let ontouchstart = move |_: TouchEvent| {
        state.set("started touch");
    };

    rsx!(
        rect {
            width: "100%",
            height: "100%",
            main_align: "center",
            cross_align: "center",
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
