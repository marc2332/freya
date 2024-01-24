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
fn PopupBackground(children: Element) -> Element {
    rsx!(rect {
        height: "100%",
        width: "100%",
        background: "rgb(0, 0, 0, 150)",
        position: "absolute",
        layer: "-1",
        main_align: "center",
        cross_align: "center",
        {children}
    })
}

#[allow(non_snake_case)]
#[component]
fn Popup(children: Element) -> Element {
    rsx!(
        PopupBackground {
            rect {
                padding: "10",
                corner_radius: "8",
                background: "white",
                shadow: "0 4 5 0 rgb(0, 0, 0, 30)",
                width: "300",
                height: "200",
                {children}
            }
        }
    )
}

fn app() -> Element {
    let mut show_popup = use_signal(|| false);

    rsx!(
        rect {
            if *show_popup.read() {
                Popup {
                    label {
                        "whatever"
                    }
                    Button {
                        onclick: move |_| show_popup.set(false),
                        label {
                            "Close"
                        }
                    }
                }
            }
            Button {
                onclick: move |_| show_popup.set(true),
                label {
                    "Open"
                }
            }
        }
    )
}
