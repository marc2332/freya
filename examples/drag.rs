#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    let mut hovering = use_signal(|| false);
    let mut positions = use_signal(|| (0.0f64, 0.0f64));
    let mut clicking = use_signal(|| false);

    let onmouseleave = move |_: MouseEvent| {
        if !(*clicking.read()) {
            hovering.set(false);
        }
    };

    let onmouseover = move |e: MouseEvent| {
        hovering.set(true);
        if *clicking.read() {
            let coordinates = e.get_screen_coordinates();
            positions.set((coordinates.x - 50.0, coordinates.y - 50.0));
        }
    };

    let onmousedown = move |_: MouseEvent| {
        clicking.set(true);
    };

    let onclick = move |_: MouseEvent| {
        clicking.set(false);
    };

    rsx!(
        rect {
            overflow: "clip",
            background: "rgb(35, 35, 35)",
            width: "100%",
            height: "100%",
            offset_x: "{positions.read().0}",
            offset_y: "{positions.read().1}",
            onmousedown: onmousedown,
            onclick: onclick,
            label {
                width: "100",
                color: "white",
                "Drag me"
            }
            rect {
                overflow: "clip",
                background: "rgb(255, 166, 0)",
                width: "100",
                height: "100",
                corner_radius: "15",
                shadow: "0 0 50 0 rgb(255, 255, 255, 0.6)",
                onmouseover: onmouseover,
                onmouseleave: onmouseleave
            }
        }
    )
}
