use dioxus::{core::UiEvent, events::MouseData, prelude::*};
use trev::launch;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            height: "100%",
            width: "100%",
            background: "red",
            Area {

            }
            Area {

            }
        }
    })
}

#[allow(non_snake_case)]
fn Area<'a>(cx: Scope<'a>) -> Element {
    let cursor_pos_over = use_state(&cx, || (0f64, 0f64));
    let cursor_pos_click = use_state(&cx, || (0f64, 0f64));

    let cursor_moved = |ev: UiEvent<MouseData>| {
        cursor_pos_over.with_mut(|cursor_pos| {
            let pos = ev.data.client_coordinates();
            cursor_pos.0 = pos.x;
            cursor_pos.1 = pos.y;
        })
    };

    let cursor_clicked = |ev: UiEvent<MouseData>| {
        cursor_pos_click.with_mut(|cursor_pos| {
            let pos = ev.data.client_coordinates();
            cursor_pos.0 = pos.x;
            cursor_pos.1 = pos.y;
        })
    };

    cx.render(rsx! {
        div {
            height: "50%",
            width: "100%",
            background: "blue",
            padding: "10",
            onmouseover: cursor_moved,
            onclick: cursor_clicked,
            p {
                "Mouse is at [x: {cursor_pos_over.0}, y: {cursor_pos_over.1}] ",
            },
            p {
                "Mouse clicked at [x: {cursor_pos_click.0}, y: {cursor_pos_click.1}]"
            }
        }
    })
}
