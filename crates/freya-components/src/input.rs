use std::{
    borrow::Cow,
    cell::{
        Ref,
        RefCell,
    },
    rc::Rc,
};

use freya_core::prelude::*;
use freya_edit::*;
use torin::{
    prelude::{
        Alignment,
        Area,
        Direction,
    },
    size::Size,
};

use crate::{
    get_theme,
    scrollviews::ScrollView,
    theming::component_themes::InputThemePartial,
};

#[derive(Default, Clone, PartialEq)]
pub enum InputMode {
    #[default]
    Shown,
    Hidden(char),
}

impl InputMode {
    pub fn new_password() -> Self {
        Self::Hidden('*')
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum InputStatus {
    /// Default state.
    #[default]
    Idle,
    /// Pointer is hovering the input.
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
    pub fn text(&'_ self) -> Ref<'_, String> {
        self.text.borrow()
    }
    pub fn set_valid(&self, is_valid: bool) {
        *self.valid.borrow_mut() = is_valid;
    }
    pub fn is_valid(&self) -> bool {
        *self.valid.borrow()
    }
}

/// Small box to write some text.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     let mut value = use_state(String::new);
///
///     rect()
///         .expanded()
///         .center()
///         .spacing(6.)
///         .child(
///             Input::new()
///                 .placeholder("Type your name")
///                 .value(value.read().clone())
///                 .onchange(move |v| value.set(v)),
///         )
///         .child(format!("Your name is {}", value.read()))
/// }
///
/// # use freya_testing::prelude::*;
/// # launch_doc(|| {
/// #   rect().center().expanded().child(Input::new() .value("Ferris"))
/// # }, (250., 250.).into(), "./images/gallery_input.png");
/// ```
/// # Preview
/// ![Input Preview][input]
#[cfg_attr(feature = "docs",
    doc = embed_doc_image::embed_image!("input", "images/gallery_input.png")
)]
#[derive(Clone, PartialEq)]
pub struct Input {
    pub(crate) theme: Option<InputThemePartial>,
    value: Cow<'static, str>,
    placeholder: Option<Cow<'static, str>>,
    on_change: Option<EventHandler<String>>,
    on_validate: Option<EventHandler<InputValidator>>,
    mode: InputMode,
    auto_focus: bool,
    width: Size,
    enabled: bool,
    key: DiffKey,
}

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}

impl Input {
    pub fn new() -> Self {
        Input {
            theme: None,
            value: Cow::default(),
            placeholder: None,
            on_change: None,
            on_validate: None,
            mode: InputMode::default(),
            auto_focus: false,
            width: Size::px(150.),
            enabled: true,
            key: DiffKey::default(),
        }
    }

    pub fn enabled(mut self, enabled: impl Into<bool>) -> Self {
        self.enabled = enabled.into();
        self
    }

    pub fn value(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.value = value.into();
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<Cow<'static, str>>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn onchange(mut self, handler: impl FnMut(String) + 'static) -> Self {
        self.on_change = Some(EventHandler::new(handler));
        self
    }

    pub fn onvalidate(mut self, handler: impl FnMut(InputValidator) + 'static) -> Self {
        self.on_validate = Some(EventHandler::new(handler));
        self
    }

    pub fn mode(mut self, mode: InputMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn auto_focus(mut self, auto_focus: bool) -> Self {
        self.auto_focus = auto_focus;
        self
    }

    pub fn width(mut self, width: impl Into<Size>) -> Self {
        self.width = width.into();
        self
    }

    pub fn theme(mut self, theme: InputThemePartial) -> Self {
        self.theme = Some(theme);
        self
    }

    pub fn key(mut self, key: impl Into<DiffKey>) -> Self {
        self.key = key.into();
        self
    }
}

impl Render for Input {
    fn render(&self) -> impl IntoElement {
        let focus = use_focus();
        let focus_status = use_focus_status(focus);
        let holder = use_state(ParagraphHolder::default);
        let mut area = use_state(Area::default);
        let mut status = use_state(InputStatus::default);
        let mut editable = use_editable(
            || self.value.to_string(),
            EditableConfig::new,
            EditableMode::MultipleLinesSingleEditor,
        );
        let mut is_dragging = use_state(|| false);
        let mut ime_preedit = use_state(|| None);

        let enabled = use_reactive(&self.enabled);
        use_drop(move || {
            if status() == InputStatus::Hovering && enabled() {
                Cursor::set(CursorIcon::default());
            }
        });

        let theme = get_theme!(&self.theme, input);

        let display_placeholder = self.value.is_empty() && self.placeholder.is_some();
        let on_change = self.on_change.clone();
        let on_validate = self.on_validate.clone();

        if &*self.value != editable.editor().read().rope() {
            editable.editor_mut().write().set(&self.value);
            editable.editor_mut().write().editor_history().clear();
        }

        let on_ime_preedit = move |e: Event<ImePreeditEventData>| {
            ime_preedit.set(Some(e.data().text.clone()));
        };

        let on_key_down = move |e: Event<KeyboardEventData>| {
            if e.key != Key::Enter && e.key != Key::Tab {
                e.stop_propagation();
                editable.process_event(EditableEvent::KeyDown {
                    key: &e.key,
                    code: e.code,
                    modifiers: e.modifiers,
                });
                let text = editable.editor().peek().to_string();

                let apply_change = if let Some(on_validate) = &on_validate {
                    let editor = editable.editor_mut();
                    let mut editor = editor.write();
                    let validator = InputValidator::new(text.clone());
                    on_validate.call(validator.clone());
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

                if apply_change && let Some(onchange) = &on_change {
                    onchange.call(text);
                }
            }
        };

        let on_key_up = move |e: Event<KeyboardEventData>| {
            e.stop_propagation();
            editable.process_event(EditableEvent::KeyUp { code: e.code });
        };

        let on_input_pointer_down = move |e: Event<PointerEventData>| {
            if !display_placeholder {
                editable.process_event(EditableEvent::Down {
                    location: e.element_location(),
                    editor_id: 0,
                    holder: &holder.read(),
                });
            }
            focus.request_focus();
        };

        let on_pointer_down = move |e: Event<PointerEventData>| {
            e.stop_propagation();
            is_dragging.set(true);
            if !display_placeholder {
                editable.process_event(EditableEvent::Down {
                    location: e.element_location(),
                    editor_id: 0,
                    holder: &holder.read(),
                });
            }
            focus.request_focus();
        };

        let on_global_mouse_move = move |e: Event<MouseEventData>| {
            if focus.is_focused() && *is_dragging.read() {
                let mut location = e.global_location;
                location.x -= area.read().min_x() as f64;
                location.y -= area.read().min_y() as f64;
                editable.process_event(EditableEvent::Move {
                    location,
                    editor_id: 0,
                    holder: &holder.read(),
                });
            }
        };

        let on_pointer_enter = move |_| {
            Cursor::set(CursorIcon::Text);
            *status.write() = InputStatus::Hovering;
        };

        let on_pointer_leave = move |_| {
            Cursor::set(CursorIcon::default());
            *status.write() = InputStatus::default();
        };

        let on_global_mouse_up = move |_| {
            match *status.read() {
                InputStatus::Idle if focus.is_focused() => {
                    editable.process_event(EditableEvent::Release);
                }
                InputStatus::Hovering => {
                    editable.process_event(EditableEvent::Release);
                }
                _ => {}
            };

            if focus.is_focused() {
                if *is_dragging.read() {
                    // The input is focused and dragging, but it just clicked so we assume the dragging can stop
                    is_dragging.set(false);
                } else {
                    // The input is focused but not dragging, so the click means it was clicked outside, therefore we can unfocus this input
                    focus.request_unfocus();
                }
            }
        };

        let a11y_id = focus.a11y_id();

        let (background, cursor_index, text_selection) = if focus_status() != FocusStatus::Not {
            (
                theme.hover_background,
                Some(editable.editor().read().cursor_pos()),
                editable.editor().read().get_visible_selection(0),
            )
        } else {
            (theme.background, None, None)
        };

        let border = if focus_status() == FocusStatus::Keyboard {
            Border::new()
                .fill(theme.focus_border_fill)
                .width(2.)
                .alignment(BorderAlignment::Inner)
        } else {
            Border::new()
                .fill(theme.border_fill.mul_if(!self.enabled, 0.85))
                .width(1.)
                .alignment(BorderAlignment::Inner)
        };

        let color = if display_placeholder {
            theme.placeholder_color
        } else {
            theme.color
        };

        let text = match (self.mode.clone(), &self.placeholder) {
            (_, Some(ph)) if display_placeholder => Cow::Borrowed(ph.as_ref()),
            (InputMode::Hidden(ch), _) => Cow::Owned(ch.to_string().repeat(self.value.len())),
            (InputMode::Shown, _) => Cow::Borrowed(self.value.as_ref()),
        };

        let preedit_text = (!display_placeholder)
            .then(|| ime_preedit.read().clone())
            .flatten();

        rect()
            .a11y_id(a11y_id)
            .a11y_focusable(self.enabled)
            // TODO
            // .a11y_auto_focus(self.auto_focus)
            .a11y_alt(text.clone())
            .maybe(self.enabled, |rect| {
                rect.on_key_up(on_key_up)
                    .on_key_down(on_key_down)
                    .on_pointer_down(on_input_pointer_down)
                    .on_pointer_enter(on_pointer_enter)
                    .on_pointer_leave(on_pointer_leave)
                    .on_ime_preedit(on_ime_preedit)
            })
            .width(self.width.clone())
            .background(background.mul_if(!self.enabled, 0.85))
            .border(border)
            .corner_radius(theme.corner_radius)
            .main_align(Alignment::center())
            .cross_align(Alignment::center())
            .child(
                ScrollView::new()
                    .height(Size::Inner)
                    .direction(Direction::Horizontal)
                    .show_scrollbar(false)
                    .child(
                        paragraph()
                            .holder(holder.read().clone())
                            .on_sized(move |e: Event<SizedEventData>| area.set(e.visible_area))
                            .min_width(Size::func(move |context| {
                                Some(context.parent + theme.inner_margin.horizontal())
                            }))
                            .maybe(self.enabled, |rect| {
                                rect.on_pointer_down(on_pointer_down)
                                    .on_global_mouse_up(on_global_mouse_up)
                                    .on_global_mouse_move(on_global_mouse_move)
                            })
                            .margin(theme.inner_margin)
                            .cursor_index(cursor_index)
                            .cursor_color(color)
                            .color(color)
                            .max_lines(1)
                            .highlights(text_selection.map(|h| vec![h]))
                            .span(text.to_string())
                            .map(preedit_text, |el, preedit_text| el.span(preedit_text)),
                    ),
            )
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

#[cfg(test)]
mod test {
    use freya_core::prelude::*;
    use freya_testing::*;

    use crate::input::Input;

    #[test]
    pub fn input_test() {
        fn input_app() -> impl IntoElement {
            let mut value = use_state(String::new);

            rect()
                .spacing(6.)
                .child(
                    Input::new()
                        .placeholder("Type your name")
                        .value(value.read().clone())
                        .onchange(move |v| value.set(v)),
                )
                .child(format!("Your name is {}", value.read()))
        }

        let mut test = launch_test(input_app);

        let placeholder = test.find(|_, element| {
            Paragraph::try_downcast(element)
                .filter(|paragraph| paragraph.spans.iter().any(|s| s.text == "Type your name"))
        });
        assert!(placeholder.is_some());
        let label = test.find(|_, element| {
            Label::try_downcast(element).filter(|label| label.text.as_ref() == "Your name is ")
        });
        assert!(label.is_some());

        // Focus
        test.click_cursor((15.0, 15.0));
        // Type
        test.write_text("Rust");

        let text = test.find(|_, element| {
            Paragraph::try_downcast(element)
                .filter(|paragraph| paragraph.spans.iter().any(|s| s.text == "Rust"))
        });
        assert!(text.is_some());
        let label = test.find(|_, element| {
            Label::try_downcast(element).filter(|label| label.text.as_ref() == "Your name is Rust")
        });
        assert!(label.is_some());
    }
}
