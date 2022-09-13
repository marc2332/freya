#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus::prelude::*;
use freya::{dioxus_elements, *};

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx!(
        rect {
            height: "100%",
            width: "100%",
            padding: "100",
            background: "black",
            ScrollView {
                rect {
                    height: "200",
                    width: "100%",
                    background: "red",
                    padding: "20",
                    rect {
                        height: "100%",
                        width: "100%",
                        background: "blue",
                        label { "hi" }
                    }
                }
                rect {
                    height: "200",
                    width: "100%",
                    background: "red",
                    label { "hi" }
                }
                rect {
                    height: "200",
                    width: "100%",
                    background: "red",
                    label { "hi" }
                }
            }
        }
    ))
}
