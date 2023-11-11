#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Position", (400.0, 350.0));
}

#[allow(non_snake_case)]
#[inline_props]
fn PopupBackground<'a>(cx: Scope<'a>, children: Element<'a>) -> Element {
    render!(rect {
        height: "100%",
        width: "100%",
        background: "rgb(0, 0, 0, 150)",
        position: "absolute",
        layer: "-1",
        main_align: "center",
        cross_align: "center",
        children
    })
}

#[allow(non_snake_case)]
#[inline_props]
fn Popup<'a>(cx: Scope<'a>, children: Element<'a>) -> Element {
    render!(
        PopupBackground {
            rect {
                padding: "10",
                corner_radius: "8",
                background: "white",
                shadow: "0 4 5 0 rgb(0, 0, 0, 30)",
                width: "300",
                height: "200",
                children
            }
        }
    )
}

fn app(cx: Scope) -> Element {
    let show_popup = use_state(cx, || false);

    render!(
        rect {
            if *show_popup.get() {
                render!(
                    Popup {
                        label {
                            "whatever"
                        }
                        Button {
                            onclick: |_| show_popup.set(false),
                            label {
                                "Close"
                            }
                        }
                    }
                )
            }
            Button {
                onclick: |_| show_popup.set(true),
                label {
                    "Open"
                }
            }
        }
    )
}
