#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let onclick = |e: MouseEvent| {
        e.stop_propagation();
        println!("clicked C");
    };

    render!(
        rect {
            height: "100%",
            width: "100%",
            padding: "100",
            onclick: |_| println!("clicked A"),
            rect {
                height: "100%",
                width: "100%",
                padding: "100",
                onclick: |_| println!("clicked B"),
                rect {
                    height: "100%",
                    width: "100%",
                    background: "red",
                    padding: "100",
                    onclick: onclick
                }
            }
        }
    )
}
