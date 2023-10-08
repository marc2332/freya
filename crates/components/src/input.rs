use crate::CursorArea;
use crate::ScrollView;
use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::{KeyboardData, MouseEvent};
use freya_hooks::ButtonTheme;
use freya_hooks::FontTheme;
use freya_hooks::{
    use_editable, use_focus, use_get_theme, EditableConfig, EditableEvent, EditableMode, TextEditor,
};
use winit::window::CursorIcon;
/// Enum to declare is [`Input`] hidden.
pub enum InputIsHidden {
    /// The input shown
    Shown,
    /// The input is obfuscated with a character
    Hidden(char),
}
/// [`Input`] component properties.
#[derive(Props)]
pub struct InputProps<'a> {
    /// Current value of the Input
    pub value: String,
    /// Handler for the `onchange` event.
    pub onchange: EventHandler<'a, String>,
    /// Is input hidden with a character. By defaultv input is shown
    #[props(default = InputIsHidden::Shown, into)]
    hidden: InputIsHidden,
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
    let focus_manager = use_focus(cx);

    let text = match cx.props.hidden {
        InputIsHidden::Hidden(ch) => ch.to_string().repeat(cx.props.value.len()),
        InputIsHidden::Shown => cx.props.value.clone(),
    };
    let cursor_attr = editable.cursor_attr(cx);
    let highlights_attr = editable.highlights_attr(cx, 0);
    let width = &cx.props.width;
    let height = &cx.props.height;
    let max_lines = &cx.props.max_lines;

    use_memo(cx, &(cx.props.value.to_string(),), {
        to_owned![editable];
        move |(text,)| {
            editable.editor().with_mut(|editor| {
                editor.set(&text);
            });
        }
    });

    let onkeydown = {
        to_owned![editable, focus_manager];
        move |e: Event<KeyboardData>| {
            if focus_manager.is_focused() {
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
            focus_manager.focus();
        }
    };

    let cursor_char = if focus_manager.is_focused() {
        editable.editor().cursor_pos().to_string()
    } else {
        "none".to_string()
    };
    let ButtonTheme {
        background,
        font_theme: FontTheme { color, .. },
        ..
    } = theme.button;

    render!(
        CursorArea {
            icon: CursorIcon::Text,
            rect {
                onkeydown: onkeydown,
                onclick: onclick,
                width: "auto",
                height: "auto",
                direction: "both",
                padding: "1.5",
                rect {
                    width: "{width}",
                    height: "{height}",
                    direction: "vertical",
                    display: "center",
                    color: "{color}",
                    background: "{background}",
                    shadow: "0 3 15 0 rgb(0, 0, 0, 70)",
                    corner_radius: "5",
                    padding: "8",
                    cursor_reference: cursor_attr,
                    ScrollView {
                        scroll_with_arrows: false,
                        paragraph {
                            width: "100%",
                            cursor_id: "0",
                            cursor_index: "{cursor_char}",
                            cursor_mode: "editable",
                            cursor_color: "{color}",
                            max_lines: "{max_lines}",
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
