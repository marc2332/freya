#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    render!(
        rect {
            height: "100%",
            width: "100%",
            Area {

            }
            Area {

            }
        }
    )
}

#[allow(non_snake_case)]
fn Area<'a>(cx: Scope<'a>) -> Element {
    let cursor_pos_click = use_state(cx, || (0f64, 0f64));

    let cursor_clicked = |e: MouseEvent| {
        cursor_pos_click.with_mut(|cursor_pos| {
            let pos = e.get_element_coordinates();
            cursor_pos.0 = pos.x;
            cursor_pos.1 = pos.y;
        })
    };

    render!(
        rect {
            color: "white",
            height: "50%",
            width: "100%",
            background: "blue",
            padding: "10",
            onclick: cursor_clicked,
            label {
                "Mouse clicked at [x: {cursor_pos_click.0}, y: {cursor_pos_click.1}]"
            }
        }
    )
}
