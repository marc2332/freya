#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Controlled Example", (600.0, 600.0));
}

fn app() -> Element {
    let mut scroll_controller = use_scroll_controller(|| ScrollConfig {
        default_vertical_position: ScrollPosition::End,
        ..Default::default()
    });

    let scroll_to_top = move |_| {
        scroll_controller.scroll_to(ScrollPosition::Start, ScrollDirection::Vertical);
    };

    let scroll_to_bottom = move |_| {
        scroll_controller.scroll_to(ScrollPosition::End, ScrollDirection::Vertical);
    };

    rsx!(
        rect {
            height: "fill",
            width: "fill",
            direction: "horizontal",
            ScrollView {
                scroll_controller,
                width: "50%",
                Button {
                    onclick: scroll_to_bottom,
                    label {
                        "Scroll to Bottom"
                    }
                }
                Card {}
                Card {}
                Card {}
            }
            ScrollView {
                scroll_controller,
                width: "50%",
                Card {}
                Card {}
                Card {}
                Button {
                    onclick: scroll_to_top,
                    label {
                        "Scroll to Top"
                    }
                }
            }
        }
    )
}

#[component]
fn Card() -> Element {
    rsx!(
        rect {
            border: "15 solid rgb(43,106,208)",
            height: "220",
            width: "420",
            background: "white",
            padding: "25",
            label {  "Scroll..." }
        }
    )
}
