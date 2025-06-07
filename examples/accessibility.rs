#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::str::FromStr;

use freya::prelude::*;
use reqwest::Url;

fn main() {
    launch(app);
}

fn app() -> Element {
    let mut focus_a = use_focus();
    let mut focus_b = use_focus();
    let mut focus_c = use_focus();
    let mut focus_d = use_focus();

    let url =
        Url::from_str("https://www.rustacean.net/assets/rustacean-orig-noshadow.png").unwrap();

    rsx!(
        rect {
            a11y_id: focus_a.attribute(),
            background: "rgb(233, 196, 106)",
            padding: "25",
            width: "50%",
            height: "50%",
            onclick: move |_| {
                focus_a.request_focus();
            },
            label {
                a11y_id: focus_c.attribute(),
                onclick: move |e| {
                    e.stop_propagation();
                    focus_c.request_focus();
                },
                "What is this?"
            }
            Button {
                onpress: |e: PressEvent | {
                    e.stop_propagation();
                },
                label {
                    "Button"
                }
            }
        }
        rect {
            a11y_id: focus_b.attribute(),
            background: "rgb(150, 100, 231)",
            padding: "25",
            width: "100%",
            height: "50%",
            a11y_role:"label",
            a11y_name: "This is a rectangle",
            onclick: move |_| {
                focus_b.request_focus();
            },
            label {
                a11y_role:"label",
                a11y_id: focus_d.attribute(),
                onclick: move |e| {
                    e.stop_propagation();
                    focus_d.request_focus();
                },
                color: "white",
                "Hello, World! This is an example."
            }
            NetworkImage {
                url,
                width: "150",
                height: "100",
                alt: "This is an image"
            }
        }
    )
}
