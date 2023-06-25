use crate::CursorArea;
use crate::ScrollView;
use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::{KeyboardData, MouseEvent};
use freya_hooks::{
    use_editable, use_focus, use_get_theme, EditableConfig, EditableEvent, EditableMode, TextEditor,
};
use winit::window::CursorIcon;

/// [`Input`] component properties.
#[derive(Props)]
pub struct InputProps<'a> {
    /// Current value of the Input
    pub value: String,
    /// Handler for the `onchange` event.
    pub onchange: EventHandler<'a, String>,
    /// Width of the Input. Default 100.
    #[props(default = "150".to_string(), into)]
    width: String,
    /// Height of the Input. Default 100.
    #[props(default = "35".to_string(), into)]
    height: String,
    /// Max lines for the Input. Default 1.
    #[props(default = "1".to_string(), into)]
    max_lines: String,
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
    let width = &cx.props.width;
    let height = &cx.props.height;
    let max_lines = &cx.props.max_lines;

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
        CursorArea {
            icon: CursorIcon::Text,
            rect {
                onkeydown: onkeydown,
                onclick: move |_| {
                    focus();
                },
                width: "auto",
                height: "auto",
                direction: "both",
                padding: "1.5",
                rect {
                    width: "{width}",
                    height: "{height}",
                    direction: "vertical",
                    display: "center",
                    color: "{button_theme.font_theme.color}",
                    shadow: "0 5 20 0 rgb(0, 0, 0, 100)",
                    corner_radius: "5",
                    padding: "8",
                    background: "{button_theme.background}",
                    cursor_reference: cursor_attr,
                    color: "white",
                    ScrollView {
                        show_scrollbar: true,
                        paragraph {
                            width: "100%",
                            cursor_id: "0",
                            cursor_index: "{cursor_char}",
                            cursor_mode: "editable",
                            cursor_color: "white",
                            max_lines: "{max_lines}",
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
            }
        }
    )
}
