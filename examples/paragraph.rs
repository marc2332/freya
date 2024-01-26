#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    let loremipsum = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

    rsx!(
        rect {
            color: "black",
            width: "100%",
            height: "100%",
            cross_align: "center",
            rect {
                width: "50%",
                height: "100%",
                ScrollView {
                    show_scrollbar: true,
                    theme: theme_with!(ScrollViewTheme {
                        height: "75%".into(),
                    }),
                    paragraph {
                        width: "100%",
                        text_align: "right",
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
                    theme: theme_with!(ScrollViewTheme {
                        height: "25%".into(),
                    }),
                    label {
                        font_size: "100",
                        font_family: "Inter",
                        "Hello World"
                    }
                }
            }
        }
    )
}
