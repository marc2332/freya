#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_params(app, "Popup", (500.0, 450.0));
}

fn app() -> Element {
    use_init_theme(|| DARK_THEME);
    let mut show_popup = use_signal(|| false);

    rsx!(
        Body {
            if *show_popup.read() {
                Popup {
                    oncloserequest: move |_| {
                        show_popup.set(false)
                    },
                    PopupTitle {
                        text: "Release New Version"
                    }
                    PopupContent {
                        label {
                            "Do you want to release a new version?"
                        }
                    }
                    PopupButtons {
                        PopupButton {
                            onpress: move |_| {
                                show_popup.set(false)
                            },
                            label {
                                "Cancel"
                            }
                        }
                        PopupButton {
                            onpress: move |_| {
                                show_popup.set(false)
                            },
                            label {
                                "Submit"
                            }
                        }
                    }
                }
            }
            Button {
                onpress: move |_| show_popup.set(true),
                label {
                    "Open"
                }
            }
        }
    )
}
