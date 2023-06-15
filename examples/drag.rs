#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let hovering = use_state(cx, || false);
    let positions = use_state(cx, || (0.0f64, 0.0f64));
    let clicking = use_state(cx, || false);

    let onmouseleave = |_: MouseEvent| {
        if !(*clicking.get()) {
            hovering.set(false);
        }
    };

    let onmouseover = |e: MouseEvent| {
        hovering.set(true);
        if *clicking.get() {
            let coordinates = e.get_screen_coordinates();
            positions.set((coordinates.x - 50.0, coordinates.y - 50.0));
        }
    };

    let onmousedown = |_: MouseEvent| {
        clicking.set(true);
    };

    let onclick = |_: MouseEvent| {
        clicking.set(false);
    };

    render!(
        container {
            background: "rgb(35, 35, 35)",
            width: "100%",
            height: "100%",
            offset_x: "{positions.0}",
            offset_y: "{positions.1}",
            onmousedown: onmousedown,
            onclick: onclick,
            label {
                width: "100",
                color: "white",
                "Drag me"
            }
            container {
                background: "rgb(255, 166, 0)",
                direction: "both",
                width: "100",
                height: "100",
                radius: "15",
                shadow: "0 0 50 0 rgb(255, 255, 255, 150)",
                onmouseover: onmouseover,
                onmouseleave: onmouseleave
            }
        }
    )
}
