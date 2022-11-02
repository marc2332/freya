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
    render!(
        container {
            background: "rgb(15, 15, 15)",
            padding: "50",
            direction: "both",
            width: "auto",
            height: "auto",
            VirtualScrollView {
                length: 6000,
                item_size: 20.0,
                show_scrollbar: true,
                builder: Box::new(|(k, i)| {
                    rsx!{
                        label {
                            key: "{k}",
                            "Number {i}"
                        }
                    }
                })
            }
        }
    )
}
