#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    let mut background = use_signal(|| "linear-gradient(250deg, orange 15%, rgb(255, 0, 0) 50%, rgb(255, 192, 203) 80%)");
    rsx!(rect {
        height: "100%",
        width: "100%",
        background: *background.read(),
        Button {
            onclick: move |_| background.set("linear-gradient(250deg, orange 15%, rgb(255, 0, 0) 50%, rgb(255, 192, 203) 80%)"),
            label { "Linear Gradient" }
        }
        Button {
            onclick: move |_| background.set("radial-gradient(orange 15%, rgb(255, 0, 0) 50%, rgb(255, 192, 203) 80%)"),
            label { "Radial Gradient" }
        }
        Button {
            onclick: move |_| background.set("conic-gradient(250deg, orange 15%, rgb(255, 0, 0) 50%, rgb(255, 192, 203) 80%)"),
            label { "Radial Gradient" }
        }
    })
}
