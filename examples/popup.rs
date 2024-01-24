#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Position", (500.0, 450.0));
}

fn app() -> Element {
    let mut show_popup = use_signal(|| false);

    rsx!(
        rect {
            if *show_popup.read() {
                Popup {
                    on_close_request: move |_| {
                        show_popup.set(false)
                    },
                    PopupTitle {
                        label {
                            "Awesome Popup"
                        }
                    }
                    PopupContent {
                        label {
                            "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
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
