use dioxus::prelude::*;
use freya_elements::{
    elements as dioxus_elements,
    events::{
        keyboard::Key,
        KeyboardData,
        MouseEvent,
    },
};
use freya_hooks::{
    use_applied_theme,
    use_editable,
    use_focus,
    use_platform,
    EditableConfig,
    EditableEvent,
    EditableMode,
    InputTheme,
    InputThemeWith,
    TextEditor,
};
use winit::window::CursorIcon;

/// Enum to declare is [`Input`] hidden.
#[derive(Default, Clone, PartialEq)]
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

/// Properties for the [`Input`] component.
#[derive(Props, Clone, PartialEq)]
pub struct InputProps {
    /// Theme override.
    pub theme: Option<InputThemeWith>,
    /// Text to show for when there is no value
    pub placeholder: Option<String>,
    /// Current value of the Input
    pub value: String,
    /// Handler for the `onchange` event.
    pub onchange: EventHandler<String>,
    /// Display mode for Input. By default, input text is shown as it is provided.
    #[props(default = InputMode::Shown, into)]
    pub mode: InputMode,
    /// Automatically focus this Input upon creation. Default `false`.
    #[props(default = false)]
    pub auto_focus: bool,
}

/// Small box to edit text.
///
/// # Styling
/// Inherits the [`InputTheme`](freya_hooks::InputTheme) theme.
///
/// # Example
///
/// ```rust,no_run
/// # use freya::prelude::*;
/// fn app() -> Element {
///     let mut value = use_signal(String::new);
///
///     rsx!(
///         label {
///             "Value: {value}"
///         }
///         Input {
///             value: value.read().clone(),
///             onchange: move |e| {
///                  value.set(e)
///             }
///         }
///     )
/// }
/// ```
#[allow(non_snake_case)]
pub fn Input(
    InputProps {
        theme,
        value,
        onchange,
        mode,
        placeholder,
        auto_focus,
    }: InputProps,
) -> Element {
    let platform = use_platform();
    let mut status = use_signal(InputStatus::default);
    let mut editable = use_editable(
        || EditableConfig::new(value.to_string()),
        EditableMode::MultipleLinesSingleEditor,
    );
    let theme = use_applied_theme!(&theme, input);
    let mut focus = use_focus();

    let is_focused = focus.is_focused();
    let display_placeholder = value.is_empty() && placeholder.is_some() && !is_focused;

    if &value != editable.editor().read().rope() {
        editable.editor_mut().write().set(&value);
    }

    use_drop(move || {
        if *status.peek() == InputStatus::Hovering {
            platform.set_cursor(CursorIcon::default());
        }
    });

    let onkeydown = move |e: Event<KeyboardData>| {
        if e.data.key != Key::Enter && e.data.key != Key::Tab {
            e.stop_propagation();
            editable.process_event(&EditableEvent::KeyDown(e.data));
            onchange.call(editable.editor().peek().to_string());
        }
    };

    let onkeyup = move |e: Event<KeyboardData>| {
        e.stop_propagation();
        editable.process_event(&EditableEvent::KeyUp(e.data));
    };

    let onmousedown = move |e: MouseEvent| {
        if !display_placeholder {
            editable.process_event(&EditableEvent::MouseDown(e.data, 0));
        }
        focus.focus();
    };

    let onmousemove = move |e: MouseEvent| {
        editable.process_event(&EditableEvent::MouseMove(e.data, 0));
    };

    let onmouseenter = move |_| {
        platform.set_cursor(CursorIcon::Text);
        *status.write() = InputStatus::Hovering;
    };

    let onmouseleave = move |_| {
        platform.set_cursor(CursorIcon::default());
        *status.write() = InputStatus::default();
    };

    let onglobalclick = move |_| match *status.read() {
        InputStatus::Idle if focus.is_focused() => {
            focus.unfocus();
            editable.process_event(&EditableEvent::Click);
        }
        InputStatus::Hovering => {
            editable.process_event(&EditableEvent::Click);
        }
        _ => {}
    };

    let a11y_id = focus.attribute();
    let cursor_reference = editable.cursor_attr();
    let highlights = editable.highlights_attr(0);

    let (background, cursor_char) = if focus.is_focused() {
        (
            theme.hover_background,
            editable.editor().read().visible_cursor_pos().to_string(),
        )
    } else {
        (theme.background, "none".to_string())
    };
    let InputTheme {
        border_fill,
        width,
        margin,
        corner_radius,
        font_theme,
        placeholder_font_theme,
        shadow,
        ..
    } = theme;

    let color = if display_placeholder {
        placeholder_font_theme.color
    } else {
        font_theme.color
    };

    let text = match (mode, placeholder) {
        (_, Some(placeholder)) if display_placeholder => placeholder,
        (InputMode::Hidden(ch), _) => ch.to_string().repeat(value.len()),
        (InputMode::Shown, _) => value,
    };

    rsx!(
        rect {
            width: "{width}",
            direction: "vertical",
            color: "{color}",
            background: "{background}",
            border: "1 solid {border_fill}",
            shadow: "{shadow}",
            corner_radius: "{corner_radius}",
            margin: "{margin}",
            main_align: "center",
            cursor_reference,
            a11y_id,
            a11y_role: "textInput",
            a11y_auto_focus: "{auto_focus}",
            onkeydown,
            onkeyup,
            paragraph {
                margin: "8 12",
                onglobalclick,
                onmouseenter,
                onmouseleave,
                onmousedown,
                onmousemove,
                width: "100%",
                cursor_id: "0",
                cursor_index: "{cursor_char}",
                cursor_mode: "editable",
                cursor_color: "{color}",
                max_lines: "1",
                highlights,
                text {
                    "{text}"
                }
            }
        }
    )
}

#[cfg(test)]
mod test {
    use freya::prelude::*;
    use freya_testing::prelude::*;

    #[tokio::test]
    pub async fn input() {
        fn input_app() -> Element {
            let mut value = use_signal(|| "Hello, Worl".to_string());

            rsx!(Input {
                value: value.read().clone(),
                onchange: move |new_value| {
                    value.set(new_value);
                }
            },)
        }

        let mut utils = launch_test(input_app);
        let root = utils.root();
        let text = root.get(0).get(0).get(0);
        utils.wait_for_update().await;

        // Default value
        assert_eq!(text.get(0).text(), Some("Hello, Worl"));

        assert_eq!(utils.focus_id(), ACCESSIBILITY_ROOT_ID);

        // Focus the input in the end of the text
        utils.push_event(PlatformEvent::Mouse {
            name: EventName::MouseDown,
            cursor: (115., 25.).into(),
            button: Some(MouseButton::Left),
        });
        utils.wait_for_update().await;
        utils.wait_for_update().await;
        utils.wait_for_update().await;

        assert_ne!(utils.focus_id(), ACCESSIBILITY_ROOT_ID);

        // Write "d"
        utils.push_event(PlatformEvent::Keyboard {
            name: EventName::KeyDown,
            key: Key::Character("d".to_string()),
            code: Code::KeyD,
            modifiers: Modifiers::default(),
        });
        utils.wait_for_update().await;

        // Check that "d" has been written into the input.
        assert_eq!(text.get(0).text(), Some("Hello, World"));
    }
}
