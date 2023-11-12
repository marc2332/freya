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
            height: "50%",
            width: "100%",
            onclick: |_| println!("clicked 1"),
            rect {
                height: "100%",
                width: "100%",
                background: "blue",
                onclick: |_| println!("clicked 2"),
            }
        }
        rect {
            height: "50%",
            width: "100%",
            onclick: |_| println!("clicked 3"),
            Test { }
        }
    )
}

#[allow(non_snake_case)]
fn Test(cx: Scope) -> Element {
    render!(rect {
        height: "100%",
        width: "100%",
        background: "red",
        onclick: |_| println!("clicked 4"),
    })
}
