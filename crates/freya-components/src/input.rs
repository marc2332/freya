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
    cursor_blink::use_cursor_blink,
    get_theme,
    scrollviews::ScrollView,
    theming::component_themes::{
        InputColorsThemePartial,
        InputLayoutThemePartial,
        InputLayoutThemePartialExt,
    },
};

#[derive(Clone, PartialEq)]
pub enum InputStyleVariant {
    Normal,
    Filled,
    Flat,
}

#[derive(Clone, PartialEq)]
pub enum InputLayoutVariant {
    Normal,
    Compact,
    Expanded,
}

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
/// ## **Normal**
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     let value = use_state(String::new);
///     Input::new(value).placeholder("Type here")
/// }
/// # use freya_testing::prelude::*;
/// # launch_doc(|| {
/// #   rect().center().expanded().child(app())
/// # }, "./images/gallery_input.png").render();
/// ```
/// ## **Filled**
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     let value = use_state(String::new);
///     Input::new(value).placeholder("Type here").filled()
/// }
/// # use freya_testing::prelude::*;
/// # launch_doc(|| {
/// #   rect().center().expanded().child(app())
/// # }, "./images/gallery_filled_input.png").render();
/// ```
/// ## **Flat**
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     let value = use_state(String::new);
///     Input::new(value).placeholder("Type here").flat()
/// }
/// # use freya_testing::prelude::*;
/// # launch_doc(|| {
/// #   rect().center().expanded().child(app())
/// # }, "./images/gallery_flat_input.png").render();
/// ```
///
/// # Preview
/// ![Input Preview][input]
/// ![Filled Input Preview][filled_input]
/// ![Flat Input Preview][flat_input]
#[cfg_attr(feature = "docs",
    doc = embed_doc_image::embed_image!("input", "images/gallery_input.png"),
    doc = embed_doc_image::embed_image!("filled_input", "images/gallery_filled_input.png"),
    doc = embed_doc_image::embed_image!("flat_input", "images/gallery_flat_input.png"),
)]
#[derive(Clone, PartialEq)]
pub struct Input {
    pub(crate) theme_colors: Option<InputColorsThemePartial>,
    pub(crate) theme_layout: Option<InputLayoutThemePartial>,
    value: Writable<String>,
    placeholder: Option<Cow<'static, str>>,
    on_validate: Option<EventHandler<InputValidator>>,
    on_submit: Option<EventHandler<String>>,
    mode: InputMode,
    auto_focus: bool,
    width: Size,
    enabled: bool,
    key: DiffKey,
    style_variant: InputStyleVariant,
    layout_variant: InputLayoutVariant,
    text_align: TextAlign,
    a11y_id: Option<AccessibilityId>,
}

impl KeyExt for Input {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Input {
    pub fn new(value: impl Into<Writable<String>>) -> Self {
        Input {
            theme_colors: None,
            theme_layout: None,
            value: value.into(),
            placeholder: None,
            on_validate: None,
            on_submit: None,
            mode: InputMode::default(),
            auto_focus: false,
            width: Size::px(150.),
            enabled: true,
            key: DiffKey::default(),
            style_variant: InputStyleVariant::Normal,
            layout_variant: InputLayoutVariant::Normal,
            text_align: TextAlign::default(),
            a11y_id: None,
        }
    }

    pub fn enabled(mut self, enabled: impl Into<bool>) -> Self {
        self.enabled = enabled.into();
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<Cow<'static, str>>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn on_validate(mut self, on_validate: impl Into<EventHandler<InputValidator>>) -> Self {
        self.on_validate = Some(on_validate.into());
        self
    }

    pub fn on_submit(mut self, on_submit: impl Into<EventHandler<String>>) -> Self {
        self.on_submit = Some(on_submit.into());
        self
    }

    pub fn mode(mut self, mode: InputMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn auto_focus(mut self, auto_focus: impl Into<bool>) -> Self {
        self.auto_focus = auto_focus.into();
        self
    }

    pub fn width(mut self, width: impl Into<Size>) -> Self {
        self.width = width.into();
        self
    }

    pub fn theme_colors(mut self, theme: InputColorsThemePartial) -> Self {
        self.theme_colors = Some(theme);
        self
    }

    pub fn theme_layout(mut self, theme: InputLayoutThemePartial) -> Self {
        self.theme_layout = Some(theme);
        self
    }

    pub fn text_align(mut self, text_align: impl Into<TextAlign>) -> Self {
        self.text_align = text_align.into();
        self
    }

    pub fn key(mut self, key: impl Into<DiffKey>) -> Self {
        self.key = key.into();
        self
    }

    pub fn style_variant(mut self, style_variant: impl Into<InputStyleVariant>) -> Self {
        self.style_variant = style_variant.into();
        self
    }

    pub fn layout_variant(mut self, layout_variant: impl Into<InputLayoutVariant>) -> Self {
        self.layout_variant = layout_variant.into();
        self
    }

    /// Shortcut for [Self::style_variant] with [InputStyleVariant::Filled].
    pub fn filled(self) -> Self {
        self.style_variant(InputStyleVariant::Filled)
    }

    /// Shortcut for [Self::style_variant] with [InputStyleVariant::Flat].
    pub fn flat(self) -> Self {
        self.style_variant(InputStyleVariant::Flat)
    }

    /// Shortcut for [Self::layout_variant] with [InputLayoutVariant::Compact].
    pub fn compact(self) -> Self {
        self.layout_variant(InputLayoutVariant::Compact)
    }

    /// Shortcut for [Self::layout_variant] with [InputLayoutVariant::Expanded].
    pub fn expanded(self) -> Self {
        self.layout_variant(InputLayoutVariant::Expanded)
    }

    pub fn a11y_id(mut self, a11y_id: impl Into<AccessibilityId>) -> Self {
        self.a11y_id = Some(a11y_id.into());
        self
    }
}

impl CornerRadiusExt for Input {
    fn with_corner_radius(self, corner_radius: f32) -> Self {
        self.corner_radius(corner_radius)
    }
}

impl Component for Input {
    fn render(&self) -> impl IntoElement {
        let focus = use_hook(|| Focus::new_for_id(self.a11y_id.unwrap_or_else(Focus::new_id)));
        let focus_status = use_focus_status(focus);
        let holder = use_state(ParagraphHolder::default);
        let mut area = use_state(Area::default);
        let mut status = use_state(InputStatus::default);
        let mut editable = use_editable(|| self.value.read().to_string(), EditableConfig::new);
        let mut is_dragging = use_state(|| false);
        let mut ime_preedit = use_state(|| None);
        let mut value = self.value.clone();

        let theme_colors = match self.style_variant {
            InputStyleVariant::Normal => get_theme!(&self.theme_colors, input),
            InputStyleVariant::Filled => get_theme!(&self.theme_colors, filled_input),
            InputStyleVariant::Flat => get_theme!(&self.theme_colors, flat_input),
        };
        let theme_layout = match self.layout_variant {
            InputLayoutVariant::Normal => get_theme!(&self.theme_layout, input_layout),
            InputLayoutVariant::Compact => get_theme!(&self.theme_layout, compact_input_layout),
            InputLayoutVariant::Expanded => get_theme!(&self.theme_layout, expanded_input_layout),
        };

        let (mut movement_timeout, cursor_color) =
            use_cursor_blink(focus_status() != FocusStatus::Not, theme_colors.color);

        let enabled = use_reactive(&self.enabled);
        use_drop(move || {
            if status() == InputStatus::Hovering && enabled() {
                Cursor::set(CursorIcon::default());
            }
        });

        let display_placeholder = value.read().is_empty() && self.placeholder.is_some();
        let on_validate = self.on_validate.clone();
        let on_submit = self.on_submit.clone();

        if &*value.read() != editable.editor().read().rope() {
            editable.editor_mut().write().set(&value.read());
            editable.editor_mut().write().editor_history().clear();
        }

        let on_ime_preedit = move |e: Event<ImePreeditEventData>| {
            ime_preedit.set(Some(e.data().text.clone()));
        };

        let on_key_down = move |e: Event<KeyboardEventData>| {
            match &e.key {
                // On submit
                Key::Named(NamedKey::Enter) => {
                    if let Some(on_submit) = &on_submit {
                        let text = editable.editor().peek().to_string();
                        on_submit.call(text);
                    }
                }
                // On unfocus
                Key::Named(NamedKey::Escape) => {
                    focus.request_unfocus();
                    Cursor::set(CursorIcon::default());
                }
                // On change
                key => {
                    if *key != Key::Named(NamedKey::Enter) && *key != Key::Named(NamedKey::Tab) {
                        e.stop_propagation();
                        movement_timeout.reset();
                        editable.process_event(EditableEvent::KeyDown {
                            key: &e.key,
                            modifiers: e.modifiers,
                        });
                        let text = editable.editor().read().rope().to_string();

                        let apply_change = match &on_validate {
                            Some(on_validate) => {
                                let mut editor = editable.editor_mut().write();
                                let validator = InputValidator::new(text.clone());
                                on_validate.call(validator.clone());
                                if !validator.is_valid() {
                                    if let Some(selection) = editor.undo() {
                                        *editor.selection_mut() = selection;
                                    }
                                    editor.editor_history().clear_redos();
                                }
                                validator.is_valid()
                            }
                            None => true,
                        };

                        if apply_change {
                            *value.write() = text;
                        }
                    }
                }
            }
        };

        let on_key_up = move |e: Event<KeyboardEventData>| {
            e.stop_propagation();
            editable.process_event(EditableEvent::KeyUp { key: &e.key });
        };

        let on_input_pointer_down = move |e: Event<PointerEventData>| {
            e.stop_propagation();
            is_dragging.set(true);
            movement_timeout.reset();
            if !display_placeholder {
                let area = area.read().to_f64();
                let global_location = e.global_location().clamp(area.min(), area.max());
                let location = (global_location - area.min()).to_point();
                editable.process_event(EditableEvent::Down {
                    location,
                    editor_line: EditorLine::SingleParagraph,
                    holder: &holder.read(),
                });
            }
            focus.request_focus();
        };

        let on_pointer_down = move |e: Event<PointerEventData>| {
            e.stop_propagation();
            is_dragging.set(true);
            movement_timeout.reset();
            if !display_placeholder {
                editable.process_event(EditableEvent::Down {
                    location: e.element_location(),
                    editor_line: EditorLine::SingleParagraph,
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
                    editor_line: EditorLine::SingleParagraph,
                    holder: &holder.read(),
                });
            }
        };

        let on_pointer_enter = move |_| {
            *status.write() = InputStatus::Hovering;
            if enabled() {
                Cursor::set(CursorIcon::Text);
            } else {
                Cursor::set(CursorIcon::NotAllowed);
            }
        };

        let on_pointer_leave = move |_| {
            if status() == InputStatus::Hovering {
                Cursor::set(CursorIcon::default());
                *status.write() = InputStatus::default();
            }
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

        let on_pointer_press = move |e: Event<PointerEventData>| {
            e.stop_propagation();
            e.prevent_default();
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
                is_dragging.set_if_modified(false);
            }
        };

        let a11y_id = focus.a11y_id();

        let (background, cursor_index, text_selection) =
            if enabled() && focus_status() != FocusStatus::Not {
                (
                    theme_colors.hover_background,
                    Some(editable.editor().read().cursor_pos()),
                    editable
                        .editor()
                        .read()
                        .get_visible_selection(EditorLine::SingleParagraph),
                )
            } else {
                (theme_colors.background, None, None)
            };

        let border = if focus_status() == FocusStatus::Keyboard {
            Border::new()
                .fill(theme_colors.focus_border_fill)
                .width(2.)
                .alignment(BorderAlignment::Inner)
        } else {
            Border::new()
                .fill(theme_colors.border_fill.mul_if(!self.enabled, 0.85))
                .width(1.)
                .alignment(BorderAlignment::Inner)
        };

        let color = if display_placeholder {
            theme_colors.placeholder_color
        } else {
            theme_colors.color
        };

        let value = self.value.read();
        let text = match (self.mode.clone(), &self.placeholder) {
            (_, Some(ph)) if display_placeholder => Cow::Borrowed(ph.as_ref()),
            (InputMode::Hidden(ch), _) => Cow::Owned(ch.to_string().repeat(value.len())),
            (InputMode::Shown, _) => Cow::Borrowed(value.as_ref()),
        };

        let preedit_text = (!display_placeholder)
            .then(|| ime_preedit.read().clone())
            .flatten();

        let a11_role = match self.mode {
            InputMode::Hidden(_) => AccessibilityRole::PasswordInput,
            _ => AccessibilityRole::TextInput,
        };

        rect()
            .a11y_id(a11y_id)
            .a11y_focusable(self.enabled)
            .a11y_auto_focus(self.auto_focus)
            .a11y_alt(text.clone())
            .a11y_role(a11_role)
            .maybe(self.enabled, |el| {
                el.on_key_up(on_key_up)
                    .on_key_down(on_key_down)
                    .on_pointer_down(on_input_pointer_down)
                    .on_ime_preedit(on_ime_preedit)
                    .on_pointer_press(on_pointer_press)
                    .on_global_mouse_up(on_global_mouse_up)
                    .on_global_mouse_move(on_global_mouse_move)
            })
            .on_pointer_enter(on_pointer_enter)
            .on_pointer_leave(on_pointer_leave)
            .width(self.width.clone())
            .background(background.mul_if(!self.enabled, 0.85))
            .border(border)
            .corner_radius(theme_layout.corner_radius)
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
                                Some(context.parent + theme_layout.inner_margin.horizontal())
                            }))
                            .maybe(self.enabled, |el| el.on_pointer_down(on_pointer_down))
                            .margin(theme_layout.inner_margin)
                            .cursor_index(cursor_index)
                            .cursor_color(cursor_color)
                            .color(color)
                            .text_align(self.text_align)
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
