#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    rsx!(
        rect {
            width: "fill",
            height: "fill",
            spacing: "20",
            main_align: "center",
            cross_align: "center",
            Button {
                onpress: |_| println!("Banana"),
                label {
                    "1 Banana"
                }
            },
            Button {
                onpress: |_| println!("Apples"),
                label {
                    "2 Apples"
                }
            },
            Button {
                onpress: |_| println!("Oranges"),
                label {
                    "3 Oranges"
                }
            },
            Button {
                onpress: |_| println!("Strawberries"),
                label {
                    "4 Strawberries"
                }
            },
        }
    )
}
