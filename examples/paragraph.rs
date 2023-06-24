#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let loremipsum = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

    render!(
        rect {
            color: "white",
            width: "75%",
            height: "100%",
            background: "black",
            ScrollView {
                show_scrollbar: true,
                width: "100%",
                height: "75%",
                paragraph {
                    width: "100%",
                    align: "right",
                    font_family: "Kablammo Zoink",
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
            ScrollView {
                show_scrollbar: true,
                height: "25%",
                label {
                    color: "white",
                    font_size: "100",
                    font_family: "Inter",
                    "Hello World"
                }
            }
        }
    )
}
