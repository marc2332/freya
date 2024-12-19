#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

enum GradientExample {
    Linear,
    Radial,
    Conic,
}

fn app() -> Element {
    let mut gradient = use_signal(|| GradientExample::Linear);

    let background = match *gradient.read() {
        GradientExample::Linear => {
            "linear-gradient(250deg, orange 15%, rgb(255, 0, 0) 50%, rgb(255, 192, 203) 80%)"
        }
        GradientExample::Radial => {
            "radial-gradient(orange 15%, rgb(255, 0, 0) 50%, rgb(255, 192, 203) 80%)"
        }
        GradientExample::Conic => {
            "conic-gradient(250deg, orange 15%, rgb(255, 0, 0) 50%, rgb(255, 192, 203) 80%)"
        }
    };

    rsx!(rect {
        height: "100%",
        width: "100%",
        background,
        Button {
            onpress: move |_| gradient.set(GradientExample::Linear),
            label { "Linear Gradient" }
        }
        Button {
            onpress: move |_| gradient.set(GradientExample::Radial),
            label { "Radial Gradient" }
        }
        Button {
            onpress: move |_| gradient.set(GradientExample::Conic),
            label { "Conic Gradient" }
        }
    })
}
