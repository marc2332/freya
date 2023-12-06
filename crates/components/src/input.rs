use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::keyboard::Key;
use freya_elements::events::{KeyboardData, MouseEvent};
use freya_hooks::FontTheme;
use freya_hooks::{
    use_editable, use_focus, use_get_theme, EditableConfig, EditableEvent, EditableMode, TextEditor,
};
use freya_hooks::{use_platform, ButtonTheme};

use winit::window::CursorIcon;
/// Enum to declare is [`Input`] hidden.
#[derive(Default)]
pub enum InputMode {
    /// The input text is shown
    #[default]
    Shown,
    /// The input text is obfuscated with a character
    Hidden(char),
}

impl InputMode {
    pub fn new_password() -> Self {
        Self::Hidden('*')
    }
}

/// Indicates the current status of the Input.
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum InputStatus {
    /// Default state.
    #[default]
    Idle,
    /// Mouse is hovering the input.
    Hovering,
}

/// [`Input`] component properties.
#[derive(Props)]
pub struct InputProps<'a> {
    /// Current value of the Input
    pub value: String,
    /// Handler for the `onchange` event.
    pub onchange: EventHandler<'a, String>,
    /// Is input hidden with a character. By default input text is shown.
    #[props(default = InputMode::Shown, into)]
    hidden: InputMode,
    /// Width of the Input. Default 150.
    #[props(default = "150".to_string(), into)]
    width: String,
    /// Margin of the Input. Default 4.
    #[props(default = "4".to_string(), into)]
    margin: String,
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
    let platform = use_platform(cx);
    let status = use_ref(cx, InputStatus::default);
    let editable = use_editable(
        cx,
        || EditableConfig::new(cx.props.value.to_string()),
        EditableMode::MultipleLinesSingleEditor,
    );
    let theme = use_get_theme(cx);
    let focus_manager = use_focus(cx);

    if &cx.props.value != editable.editor().current().rope() {
        editable.editor().with_mut(|editor| {
            editor.set(&cx.props.value);
        });
    }

    let text = match cx.props.hidden {
        InputMode::Hidden(ch) => ch.to_string().repeat(cx.props.value.len()),
        InputMode::Shown => cx.props.value.clone(),
    };

    use_on_unmount(cx, {
        to_owned![status, platform];
        move || {
            if *status.read() == InputStatus::Hovering {
                platform.set_cursor(CursorIcon::default());
            }
        }
    });

    let onkeydown = {
        to_owned![editable, focus_manager];
        move |e: Event<KeyboardData>| {
            if focus_manager.is_focused() && e.data.key != Key::Enter {
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
            focus_manager.focus();
        }
    };

    let onmouseover = {
        to_owned![editable];
        move |e: MouseEvent| {
            editable.process_event(&EditableEvent::MouseOver(e.data, 0));
        }
    };

    let onmouseenter = {
        to_owned![platform];
        move |_| {
            platform.set_cursor(CursorIcon::Text);
            *status.write_silent() = InputStatus::Hovering;
        }
    };

    let onmouseleave = move |_| {
        platform.set_cursor(CursorIcon::default());
        *status.write_silent() = InputStatus::default();
    };

    let onglobalclick = {
        to_owned![editable];
        move |_| match *status.read() {
            InputStatus::Idle if focus_manager.is_focused() => {
                focus_manager.unfocus();
            }
            InputStatus::Hovering => {
                editable.process_event(&EditableEvent::Click);
            }
            _ => {}
        }
    };

    let cursor_attr = editable.cursor_attr(cx);
    let highlights_attr = editable.highlights_attr(cx, 0);
    let width = &cx.props.width;
    let margin = &cx.props.margin;
    let (background, cursor_char) = if focus_manager.is_focused() {
        (
            theme.button.hover_background,
            editable.editor().cursor_pos().to_string(),
        )
    } else {
        (theme.button.background, "none".to_string())
    };
    let ButtonTheme {
        border_fill,
        font_theme: FontTheme { color, .. },
        ..
    } = theme.button;

    render!(
        rect {
            width: "{width}",
            direction: "vertical",
            color: "{color}",
            background: "{background}",
            border: "1 solid {border_fill}",
            shadow: "0 3 15 0 rgb(0, 0, 0, 0.3)",
            corner_radius: "10",
            margin: "{margin}",
            cursor_reference: cursor_attr,
            main_align: "center",
            paragraph {
                margin: "8 12",
                onkeydown: onkeydown,
                onglobalclick: onglobalclick,
                onmouseenter: onmouseenter,
                onmouseleave: onmouseleave,
                onmousedown: onmousedown,
                onmouseover: onmouseover,
                width: "100%",
                cursor_id: "0",
                cursor_index: "{cursor_char}",
                cursor_mode: "editable",
                cursor_color: "{color}",
                max_lines: "1",
                highlights: highlights_attr,
                text {
                    "{text}"
                }
            }
        }
    )
}
