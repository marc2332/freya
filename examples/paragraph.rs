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
    let loremipsum = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

    cx.render(rsx!(
        rect {
            width: "75%",
            height: "100%",
            background: "black",
            container {
                width: "100%",
                height: "75%",
                paragraph {
                    width: "100%",
                    text {
                        color: "rgb(240, 50, 100)",
                        "{loremipsum.repeat(2)}"
                    }
                    text {
                        color: "rgb(25, 160, 200)",
                        "{loremipsum.repeat(1)}"
                    }
                }
            }
            label {
                font_size: "100",
                font_family: "Inter",
                "Hello World"
            }
        }
    ))
}
