#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    let mut cursor_pos_over = use_signal(|| CursorPoint::default());
    let mut cursor_pos_click = use_signal(|| CursorPoint::default());

    let onmousemove = move |e: MouseEvent| {
        let cursor_pos = e.get_screen_coordinates();
        cursor_pos_over.set(cursor_pos);
    };

    let onclick = move |e: MouseEvent| {
        let cursor_pos = e.get_screen_coordinates();
        cursor_pos_click.set(cursor_pos);
    };

    rsx!(
        rect {
            height: "fill",
            width: "fill",
            background: "rgb(0, 119, 182)",
            color: "white",
            padding: "15",
            onmousemove,
            onclick,
            label {
                "Mouse is at [x: {cursor_pos_over.read().x}, y: {cursor_pos_over.read().y}] ",
            },
            label {
                "Mouse clicked at [x: {cursor_pos_click.read().x}, y: {cursor_pos_click.read().y}]"
            }
        }
    )
}
