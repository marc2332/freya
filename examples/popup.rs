#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Popup", (500.0, 450.0));
}

fn app() -> Element {
    use_init_theme(DARK_THEME);
    let mut show_popup = use_signal(|| false);

    rsx!(
        Body {
            if *show_popup.read() {
                Popup {
                    oncloserequest: move |_| {
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
