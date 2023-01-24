use dioxus::{core::Event, prelude::*};
use dioxus_elements::events_data::{Key, KeyboardData};
use freya_elements as dioxus_elements;
use freya_hooks::{use_focus, use_get_theme};

/// Properties for the Input component.
#[derive(Props)]
pub struct InputProps<'a> {
    pub value: &'a str,
    pub onchange: EventHandler<'a, String>,
}

/// A controlled Input component.
#[allow(non_snake_case)]
pub fn Input<'a>(cx: Scope<'a, InputProps<'a>>) -> Element {
    let theme = use_get_theme(cx);
    let button_theme = &theme.button;
    let (focused, focus) = use_focus(cx);
    let text = cx.props.value;
    let onkeydown = move |e: Event<KeyboardData>| {
        if focused {
            if let Key::Character(text_char) = &e.data.key {
                // Add a new char
                cx.props.onchange.call(format!("{}{}", text, text_char));
            } else if let Key::Backspace = e.data.key {
                // Remove the last character
                let mut content = text.to_string();
                content.pop();
                cx.props.onchange.call(content);
            }
        }
    };

    render!(
        container {
            onkeydown: onkeydown,
            onclick: move |_| {
                focus();
            },
            width: "auto",
            height: "auto",
            direction: "both",
            padding: "3",
            container {
                width: "100",
                height: "35",
                direction: "both",
                color: "{button_theme.font_theme.color}",
                shadow: "0 5 15 10 black",
                radius: "5",
                padding: "17",
                background: "{button_theme.background}",
                label {
                    "{text}"
                }
            }
        }
    )
}
