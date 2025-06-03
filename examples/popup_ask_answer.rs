#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Popup", (500.0, 450.0));
}

fn app() -> Element {
    let mut my_popup = use_popup::<String, String>();

    let onpress = move |_| async move {
        let name = my_popup.open("What's your name?".to_string()).await;
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
    let mut popup_answer = use_popup_answer::<String, String>();
    let data = popup_answer.data();

    rsx!(
        Popup {
            oncloserequest: move |_| {
                popup_answer.answer(None);
            },
            PopupTitle {
                text: "{data}"
            }
            PopupButtons {
                PopupButton {
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
