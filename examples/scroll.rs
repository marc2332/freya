#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Scroll example", (400.0, 400.0));
}

#[allow(non_snake_case)]
fn Card(cx: Scope) -> Element {
    let scroll = use_scroll(cx);

    let position = scroll.read().scroll_y;

    render!(
        rect {
            height: "200",
            width: "400",
            background: "rgb(214, 40, 40)",
            padding: "15",
            rect {
                height: "100%",
                width: "100%",
                background: "rgb(27, 38, 59)",
                padding: "15",
                label {  "Scroll support :)" }
                label {
                    "Position: {-position}"
                }
            }
        }
    )
}

fn app(cx: Scope) -> Element {
    use_init_scroll(cx);
    let scroll = use_scroll(cx);

    let onclick = |_| scroll.write().go_top();

    render!(
        rect {
            height: "100%",
            width: "100%",
            padding: "50",
            background: "white",
            color: "white",
            ScrollView {
                show_scrollbar: true,
                Card { }
                Card { }
                Card { }
                Button {
                    onclick: onclick,
                    label {
                        "Go to top"
                    }
                }
            }
        }
    )
}
