#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus::prelude::*;
use elements_namespace as dioxus_elements;

use freya::launch;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx!(
        rect {
            height: "100%",
            width: "100%",
            direction: "horizontal",
            rect {
                height: "100%",
                width: "10%",
                background: "rgb(66, 75, 84)",
            }
            rect {
                height: "100%",
                width: "10%",
                background: "rgb(179, 141, 151)",
            }
            rect {
                height: "100%",
                width: "10%",
                background: "rgb(213, 172, 169)",
            }
            rect {
                height: "100%",
                width: "10%",
                background: "rgb(235, 207, 178)",
            }
            rect {
                height: "100%",
                width: "10%",
                background: "rgb(197, 186, 175)",
            }
            rect {
                height: "100%",
                width: "10%",
                background: "rgb(237, 238, 201)",
            }
            rect {
                height: "100%",
                width: "10%",
                background: "rgb(221, 231, 199)",
            }
            rect {
                height: "100%",
                width: "10%",
                background: "rgb(191, 216, 189)",
            }
            rect {
                height: "100%",
                width: "10%",
                background: "rgb(152, 201, 163)",
            }
            rect {
                height: "100%",
                width: "10%",
                background: "rgb(119, 191, 163)",
            }
        }
    ))
}
