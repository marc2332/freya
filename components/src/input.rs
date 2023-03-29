use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::{keyboard::Key, KeyboardData};
use freya_hooks::{use_focus, use_get_theme};

/// [`Input`] component properties.
#[derive(Props)]
pub struct InputProps<'a> {
    /// Current value of the Input
    pub value: &'a str,
    /// Handler for the `onchange` event.
    pub onchange: EventHandler<'a, String>,
}

/// Controlled `Input` component.
///
/// # Props
/// See [`InputProps`].
///
/// # Styling
/// Inherits the [`ButtonTheme`](freya_hooks::ButtonTheme) theme.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app(cx: Scope) -> Element {
///     use_init_focus(cx);
///     let value = use_state(cx, String::new);
///
///     render!(
///         label {
///             "Value: {value}"
///         }
///         Input {
///             value: &value,
///             onchange: |e| {
///                  value.set(e)
///             }
///         }
///     )
/// }
/// ```
#[allow(non_snake_case)]
pub fn Input<'a>(cx: Scope<'a, InputProps<'a>>) -> Element {
    let theme = use_get_theme(cx);
    let button_theme = &theme.button;
    let focus_manager = use_focus(cx);
    let text = cx.props.value;
    let onkeydown = move |e: Event<KeyboardData>| {
        if focus_manager.is_focused() {
            if let Key::Character(text_char) = &e.data.key {
                // Add a new char
                cx.props.onchange.call(format!("{text}{text_char}"));
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
                focus_manager.focus();
            },
            width: "auto",
            height: "auto",
            direction: "both",
            padding: "1.5",
            container {
                width: "100",
                height: "35",
                direction: "both",
                color: "{button_theme.font_theme.color}",
                shadow: "0 5 15 10 black",
                radius: "5",
                padding: "8",
                background: "{button_theme.background}",
                label {
                    "{text}"
                }
            }
        }
    )
}
