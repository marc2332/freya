#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::rc::Rc;

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Works", (400.0, 350.0));
}

fn app() -> Element {
    rsx!(
        rect {
            height: "100%",
            width: "100%",
            main_align: "center",
            cross_align: "center",
            background: "rgb(0, 119, 182)",
            color: "white",
            onclick: |e| println!("{e:?}"),
            label {
                font_size: "50",
                font_weight: "bold",
                "Hello World!"
            }
        }
    )
}
