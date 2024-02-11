#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    let mut positions = use_signal(|| (0.0f64, 0.0f64));
    let mut clicking = use_signal::<Option<CursorPoint>>(|| None);

    let onglobalmouseover = move |e: MouseEvent| {
        if let Some(clicked) = *clicking.read() {
            let coordinates = e.get_screen_coordinates();
            positions.set((coordinates.x - clicked.x, coordinates.y - clicked.y));
        }
    };

    let onmousedown = move |e: MouseEvent| {
        clicking.set(Some(e.get_element_coordinates()));
    };

    let onglobalclick = move |_: MouseEvent| {
        clicking.set(None);
    };

    rsx!(
        rect {
            background: "rgb(35, 35, 35)",
            width: "100%",
            height: "100%",
            offset_x: "{positions.read().0}",
            offset_y: "{positions.read().1}",
            rect {
                background: "rgb(255, 166, 0)",
                width: "120",
                height: "120",
                corner_radius: "16",
                shadow: "0 0 35 10 rgb(255, 255, 255, 0.4)",
                onglobalclick,
                onglobalmouseover,
                onmousedown,
                main_align: "center",
                cross_align: "center",
                label {
                    color: "white",
                    "Drag me"
                }
            }
        }
    )
}
