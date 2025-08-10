use std::{
    borrow::Cow,
    cell::{
        Ref,
        RefCell,
    },
    rc::Rc,
};

use ::warnings::Warning;
use dioxus::prelude::*;
use freya_core::platform::CursorIcon;
use freya_elements::{
    self as dioxus_elements,
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

use crate::ScrollView;

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

#[derive(Clone)]
pub struct InputValidator {
    valid: Rc<RefCell<bool>>,
    text: Rc<RefCell<String>>,
}

impl InputValidator {
    pub fn new(text: String) -> Self {
        Self {
            valid: Rc::new(RefCell::new(true)),
            text: Rc::new(RefCell::new(text)),
        }
    }

    /// Read the text to validate.
    pub fn text(&self) -> Ref<String> {
        self.text.borrow()
    }

    /// Mark the text as valid.
    pub fn set_valid(&self, is_valid: bool) {
        *self.valid.borrow_mut() = is_valid;
    }

    /// Check if the text was marked as valid.
    pub fn is_valid(&self) -> bool {
        *self.valid.borrow()
    }
}

/// Properties for the [`Input`] component.
#[derive(Props, Clone, PartialEq)]
pub struct InputProps {
    /// Theme override.
    pub theme: Option<InputThemeWith>,
    /// Text to show for when there is no value
    pub placeholder: ReadOnlySignal<Option<String>>,
    /// Current value of the Input.
    pub value: ReadOnlySignal<String>,
    /// Handler for the `onchange` event.
    pub onchange: EventHandler<String>,
    /// Display mode for Input. By default, input text is shown as it is provided.
    #[props(default = InputMode::Shown, into)]
    pub mode: InputMode,
    /// Automatically focus this Input upon creation. Default `false`.
    #[props(default = false)]
    pub auto_focus: bool,
    /// Handler for the `onvalidate` function.
    pub onvalidate: Option<EventHandler<InputValidator>>,
    #[props(default = "150".to_string())]
    pub width: String,
    /// Handler for the `onfocuschange` function.
    pub onfocuschange: Option<EventHandler<bool>>,
}

/// Small box to edit text.
///
/// # Styling
/// Inherits the [`InputTheme`](freya_hooks::InputTheme) theme.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> Element {
///     let mut value = use_signal(String::new);
///
///     rsx!(
///         label {
///             "Value: {value}"
///         }
///         Input {
///             value,
///             onchange: move |e| {
///                  value.set(e)
///             }
///         }
///     )
/// }
/// # use freya_testing::prelude::*;
/// # launch_doc(|| {
/// #   rsx!(
/// #       Preview {
/// #           Input {
/// #               value: "Some text...",
/// #               onchange: move |_| { }
/// #           }
/// #       }
/// #   )
/// # }, (250., 250.).into(), "./images/gallery_input.png");
/// ```
/// # Preview
/// ![Input Preview][input]
#[cfg_attr(feature = "docs",
    doc = embed_doc_image::embed_image!("input", "images/gallery_input.png")
)]
#[allow(non_snake_case)]
pub fn Input(
    InputProps {
        theme,
        value,
        onchange,
        mode,
        placeholder,
        auto_focus,
        onvalidate,
        width,
        onfocuschange,
    }: InputProps,
) -> Element {
    let platform = use_platform();
    let mut status = use_signal(InputStatus::default);
    let mut editable = use_editable(
        || EditableConfig::new(value.to_string()),
        EditableMode::MultipleLinesSingleEditor,
    );
    let InputTheme {
        border_fill,
        focus_border_fill,
        margin,
        corner_radius,
        font_theme,
        placeholder_font_theme,
        shadow,
        background,
        hover_background,
        inner_horizontal_margin,
        inner_vertical_margin,
    } = use_applied_theme!(&theme, input);
    let mut focus = use_focus();
    let mut drag_origin = use_signal(|| None);

    let value = value.read();
    let placeholder = placeholder.read();
    let display_placeholder = value.is_empty() && placeholder.is_some();

    let _allow_write_in_component_body =
        ::warnings::Allow::new(warnings::signal_write_in_component_body::ID);
    let _allow_read_and_write_in_reactive_scope =
        ::warnings::Allow::new(warnings::signal_read_and_write_in_reactive_scope::ID);

    if &*value != editable.editor().read().rope() {
        editable.editor_mut().write().set(&value);
        editable.editor_mut().write().editor_history().clear();
    }

    use_drop(move || {
        if *status.peek() == InputStatus::Hovering {
            platform.set_cursor(CursorIcon::default());
        }
    });

    use_effect(move || {
        if !focus.is_focused() {
            editable.editor_mut().write().clear_selection();
        }

        if let Some(onfocuschange) = onfocuschange {
            onfocuschange.call(focus.is_focused())
        }
    });

    let onkeydown = move |e: Event<KeyboardData>| {
        if e.data.key != Key::Enter && e.data.key != Key::Tab {
            e.stop_propagation();
            editable.process_event(&EditableEvent::KeyDown(e.data));
            let text = editable.editor().peek().to_string();

            let apply_change = if let Some(onvalidate) = onvalidate {
                let editor = editable.editor_mut();
                let mut editor = editor.write();
                let validator = InputValidator::new(text.clone());
                onvalidate(validator.clone());
                let is_valid = validator.is_valid();

                if !is_valid {
                    // If it is not valid then undo the latest change and discard all the redos
                    let undo_result = editor.undo();
                    if let Some(idx) = undo_result {
                        editor.set_cursor_pos(idx);
                    }
                    editor.editor_history().clear_redos();
                }

                is_valid
            } else {
                true
            };

            if apply_change {
                onchange.call(text);
            }
        }
    };

    let onkeyup = move |e: Event<KeyboardData>| {
        e.stop_propagation();
        editable.process_event(&EditableEvent::KeyUp(e.data));
    };

    let oninputmousedown = move |e: MouseEvent| {
        if !display_placeholder {
            editable.process_event(&EditableEvent::MouseDown(e.data, 0));
        }
        focus.request_focus();
    };

    let onmousedown = move |e: MouseEvent| {
        e.stop_propagation();
        drag_origin.set(Some(e.get_screen_coordinates() - e.element_coordinates));
        if !display_placeholder {
            editable.process_event(&EditableEvent::MouseDown(e.data, 0));
        }
        focus.request_focus();
    };

    let onglobalmousemove = move |mut e: MouseEvent| {
        if focus.is_focused() {
            if let Some(drag_origin) = drag_origin() {
                let data = Rc::get_mut(&mut e.data).unwrap();
                data.element_coordinates.x -= drag_origin.x;
                data.element_coordinates.y -= drag_origin.y;
                editable.process_event(&EditableEvent::MouseMove(e.data, 0));
            }
        }
    };

    let onmouseenter = move |_| {
        platform.set_cursor(CursorIcon::Text);
        *status.write() = InputStatus::Hovering;
    };

    let onmouseleave = move |_| {
        platform.set_cursor(CursorIcon::default());
        *status.write() = InputStatus::default();
    };

    let onglobalclick = move |_| {
        match *status.read() {
            InputStatus::Idle if focus.is_focused() => {
                editable.process_event(&EditableEvent::Click);
            }
            InputStatus::Hovering => {
                editable.process_event(&EditableEvent::Click);
            }
            _ => {}
        };

        // Unfocus input when this:
        // + is focused
        // + it has not just being dragged
        // + a global click happened
        if focus.is_focused() {
            if drag_origin.read().is_some() {
                drag_origin.set(None);
            } else {
                focus.request_unfocus();
            }
        }
    };

    let a11y_id = focus.attribute();
    let cursor_reference = editable.cursor_attr();
    let highlights = editable.highlights_attr(0);

    let (background, cursor_char) = if focus.is_focused() {
        (
            hover_background,
            editable.editor().read().cursor_pos().to_string(),
        )
    } else {
        (background, "none".to_string())
    };
    let border = if focus.is_focused_with_keyboard() {
        format!("2 inner {focus_border_fill}")
    } else {
        format!("1 inner {border_fill}")
    };

    let color = if display_placeholder {
        placeholder_font_theme.color
    } else {
        font_theme.color
    };

    let text = match (mode, &*placeholder) {
        (_, Some(placeholder)) if display_placeholder => Cow::Borrowed(placeholder.as_str()),
        (InputMode::Hidden(ch), _) => Cow::Owned(ch.to_string().repeat(value.len())),
        (InputMode::Shown, _) => Cow::Borrowed(value.as_str()),
    };

    rsx!(
        rect {
            width,
            direction: "vertical",
            color: "{color}",
            background: "{background}",
            border,
            shadow: "{shadow}",
            corner_radius: "{corner_radius}",
            margin: "{margin}",
            main_align: "center",
            a11y_id,
            a11y_role: "text-input",
            a11y_auto_focus: "{auto_focus}",
            a11y_value: "{text}",
            onkeydown,
            onkeyup,
            overflow: "clip",
            onmousedown: oninputmousedown,
            onmouseenter,
            onmouseleave,
            ScrollView {
                height: "auto",
                direction: "horizontal",
                show_scrollbar: false,
                paragraph {
                    min_width: "calc(100% - {inner_horizontal_margin} - {inner_horizontal_margin})",
                    margin: "{inner_vertical_margin} {inner_horizontal_margin}",
                    onglobalclick,
                    onmousedown,
                    onglobalmousemove,
                    cursor_reference,
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
                value,
                onchange: move |new_value| {
                    value.set(new_value);
                }
            })
        }

        let mut utils = launch_test(input_app);
        let root = utils.root();
        let text = root.get(0).get(0).get(0).get(0).get(0).get(0);
        utils.wait_for_update().await;

        // Default value
        assert_eq!(text.get(0).text(), Some("Hello, Worl"));

        assert_eq!(utils.focus_id(), ACCESSIBILITY_ROOT_ID);

        // Focus the input in the end of the text
        utils.push_event(TestEvent::Mouse {
            name: MouseEventName::MouseDown,
            cursor: (115., 25.).into(),
            button: Some(MouseButton::Left),
        });
        utils.wait_for_update().await;
        utils.wait_for_update().await;
        utils.wait_for_update().await;

        assert_ne!(utils.focus_id(), ACCESSIBILITY_ROOT_ID);

        // Write "d"
        utils.push_event(TestEvent::Keyboard {
            name: KeyboardEventName::KeyDown,
            key: Key::Character("d".to_string()),
            code: Code::KeyD,
            modifiers: Modifiers::default(),
        });
        utils.wait_for_update().await;

        // Check that "d" has been written into the input.
        assert_eq!(text.get(0).text(), Some("Hello, World"));
    }

    #[tokio::test]
    pub async fn validate() {
        fn input_app() -> Element {
            let mut value = use_signal(|| "A".to_string());

            rsx!(Input {
                value: value.read().clone(),
                onvalidate: |validator: InputValidator| {
                    if validator.text().len() > 3 {
                        validator.set_valid(false)
                    }
                },
                onchange: move |new_value| {
                    value.set(new_value);
                }
            },)
        }

        let mut utils = launch_test(input_app);
        let root = utils.root();
        let text = root.get(0).get(0).get(0).get(0).get(0).get(0);
        utils.wait_for_update().await;

        // Default value
        assert_eq!(text.get(0).text(), Some("A"));

        // Focus the input in the end of the text
        utils.push_event(TestEvent::Mouse {
            name: MouseEventName::MouseDown,
            cursor: (115., 25.).into(),
            button: Some(MouseButton::Left),
        });
        utils.wait_for_update().await;
        utils.wait_for_update().await;
        utils.wait_for_update().await;

        assert_ne!(utils.focus_id(), ACCESSIBILITY_ROOT_ID);

        // Try to write "BCDEFG"
        for c in ['B', 'C', 'D', 'E', 'F', 'G'] {
            utils.push_event(TestEvent::Keyboard {
                name: KeyboardEventName::KeyDown,
                key: Key::Character(c.to_string()),
                code: Code::Unidentified,
                modifiers: Modifiers::default(),
            });
            utils.wait_for_update().await;
        }

        // Check that only "BC" was been written to the input.
        assert_eq!(text.get(0).text(), Some("ABC"));
    }
}
