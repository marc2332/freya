use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::{KeyboardData, MouseEvent};
use freya_hooks::{
    use_editable, use_focus, use_get_theme, EditableEvent, EditableMode, TextEditor,
};

/// [`Input`] component properties.
#[derive(Props)]
pub struct InputProps<'a> {
    /// Current value of the Input
    pub value: &'a str,
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
    let editable = use_editable(
        cx,
        || cx.props.value.to_string(),
        EditableMode::MultipleLinesSingleEditor,
    );
    let theme = use_get_theme(cx);
    let (focused, focus) = use_focus(cx);

    let click_notifier = editable.click_notifier().clone();
    let text = cx.props.value;
    let button_theme = &theme.button;
    let cursor_attr = editable.cursor_attr(cx);
    let highlights_attr = editable.highlights_attr(cx, 0);

    let onkeydown = {
        to_owned![editable];
        move |e: Event<KeyboardData>| {
            if focused {
                editable.editor.with_mut(|editor| {
                    editor.process_key(&e.data.key, &e.data.code, &e.data.modifiers);
                    cx.props.onchange.call(editor.to_string());
                });
            }
        }
    };

    use_effect(cx, &(cx.props.value.to_string(),), {
        to_owned![editable];
        move |(text,)| {
            editable.editor().with_mut(|e| {
                e.set(&text);
            });
            async move {}
        }
    });

    let onmousedown = {
        to_owned![click_notifier];
        move |e: MouseEvent| {
            click_notifier
                .send(EditableEvent::MouseDown(e.data, 0))
                .ok();
        }
    };

    let onmouseover = {
        to_owned![click_notifier];
        move |e: MouseEvent| {
            click_notifier
                .send(EditableEvent::MouseOver(e.data, 0))
                .ok();
        }
    };

    let onclick = move |_: MouseEvent| {
        click_notifier.send(EditableEvent::Click).ok();
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
