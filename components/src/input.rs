use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::{KeyboardData, MouseEvent};
use freya_hooks::{
    use_editable, use_focus, use_get_theme, EditableConfig, EditableEvent, EditableMode, TextEditor,
};

/// [`Input`] component properties.
#[derive(Props)]
pub struct InputProps<'a> {
    /// Current value of the Input
    pub value: String,
    /// Handler for the `onchange` event.
    pub onchange: EventHandler<'a, String>,
}

/// `Input` component.
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
///             value: value.get().clone(),
///             onchange: |e| {
///                  value.set(e)
///             }
///         }
///     )
/// }
/// ```
#[allow(non_snake_case)]
pub fn Input<'a>(cx: Scope<'a, InputProps<'a>>) -> Element {
    let editable = use_editable(
        cx,
        || EditableConfig::new(cx.props.value.to_string()),
        EditableMode::MultipleLinesSingleEditor,
    );
    let theme = use_get_theme(cx);
    let (focused, focus) = use_focus(cx);

    let text = &cx.props.value;
    let button_theme = &theme.button;
    let cursor_attr = editable.cursor_attr(cx);
    let highlights_attr = editable.highlights_attr(cx, 0);

    use_effect(cx, &(cx.props.value.to_string(),), {
        to_owned![editable];
        move |(text,)| {
            editable.editor().with_mut(|editor| {
                editor.set(&text);
            });
            async move {}
        }
    });

    let onkeydown = {
        to_owned![editable];
        move |e: Event<KeyboardData>| {
            if focused {
                editable.process_event(&EditableEvent::KeyDown(e.data));
                cx.props
                    .onchange
                    .call(editable.editor().current().to_string());
            }
        }
    };

    let onmousedown = {
        to_owned![editable];
        move |e: MouseEvent| {
            editable.process_event(&EditableEvent::MouseDown(e.data, 0));
        }
    };

    let onmouseover = {
        to_owned![editable];
        move |e: MouseEvent| {
            editable.process_event(&EditableEvent::MouseOver(e.data, 0));
        }
    };

    let onclick = {
        to_owned![editable];
        move |_: MouseEvent| {
            editable.process_event(&EditableEvent::Click);
        }
    };

    let cursor_char = if focused {
        editable.editor().cursor_pos().to_string()
    } else {
        "none".to_string()
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
                cursor_reference: cursor_attr,
                color: "white",
                paragraph {
                    width: "100%",
                    cursor_id: "0",
                    cursor_index: "{cursor_char}",
                    cursor_mode: "editable",
                    cursor_color: "white",
                    max_lines: "1",
                    onclick: onclick,
                    onmouseover: onmouseover,
                    onmousedown: onmousedown,
                    highlights: highlights_attr,
                    text {
                        "{text}"
                    }
                }
            }
        }
    )
}
