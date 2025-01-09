#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Popup", (500.0, 450.0));
}

fn app() -> Element {
    use_init_theme(|| DARK_THEME);
    let mut value = use_signal(|| "Default text".to_string());
    let mut show_popup = use_signal(|| false);

    rsx!(
        Body {
            label {
                "Value is: {value}"
            }
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
                        rect {
                            spacing: "10",
                            label {
                                "Change the input value:"
                            }
                            Input {
                                value,
                                onchange: move |text| {
                                    value.set(text);
                                }
                            }
                            Button {
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
