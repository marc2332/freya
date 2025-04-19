#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Popup", (500.0, 450.0));
}

fn app() -> Element {
    let mut my_popup = use_popup::<String>();

    let onpress = move |_| async move {
        let name = my_popup.open().await;
        if let Some(name) = &*name {
            println!("Name is: {name}!");
        }
    };

    rsx!(
        Button {
            onpress,
            label {
                "Ask name"
            }
        }
        if my_popup.is_open() {
            AskNamePopup {}
        }
    )
}

#[component]
fn AskNamePopup() -> Element {
    let mut popup_answer = use_popup_answer::<String>();

    rsx!(
        Popup {
            oncloserequest: move |_| {
                popup_answer.answer(None);
            },
            PopupTitle {
                label {
                    "Ask Name"
                }
            }
            PopupContent {
                Button {
                    onpress: move |_| {
                        popup_answer.answer("Marc".to_string())
                    },
                    label {
                        "Answer 'Marc'"
                    }
                }
            }
        }
    )
}
