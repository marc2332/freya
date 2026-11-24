use std::{
    borrow::Cow,
    cell::{
        Ref,
        RefCell,
    },
    hash::{
        Hash,
        Hasher,
    },
    rc::Rc,
};

use freya_core::{
    elements::paragraph::{
        ParagraphCursorExt,
        ParagraphHolderInner,
    },
    prelude::*,
};
use freya_edit::*;
use torin::{
    gaps::Gaps,
    prelude::{
        Alignment,
        Area,
        Content,
        Direction,
        Point2D,
        Vector2D,
    },
    size::Size,
};
use tracing::warn;

use crate::{
    context_menu::ContextMenu,
    cursor_blink::use_cursor_blink,
    define_theme,
    get_theme,
    menu::{
        Menu,
        MenuButton,
    },
    scrollviews::{
        ScrollConfig,
        ScrollView,
        use_scroll_controller,
    },
};

define_theme! {
    for = Input;
    theme_field = theme_layout;

    %[component]
    pub InputLayout {
        %[fields]
        corner_radius: CornerRadius,
        padding: Gaps,
    }
}

define_theme! {
    for = Input;
    theme_field = theme_colors;

    %[component]
    pub InputColors {
        %[fields]
        background: Color,
        focus_background: Color,
        border_fill: Color,
        focus_border_fill: Color,
        color: Color,
        placeholder_color: Color,
    }
}

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

#[derive(Default, Clone, Copy, PartialEq)]
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

#[derive(Clone, PartialEq)]
enum InputValue {
    Value(WritableSetter<String>),
    Editor(UseEditable),
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
/// ```
/// ## **Filled**
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     let value = use_state(String::new);
///     Input::new(value).placeholder("Type here").filled()
/// }
/// ```
/// ## **Flat**
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     let value = use_state(String::new);
///     Input::new(value).placeholder("Type here").flat()
/// }
/// ```
///
/// See the [interactive components demo](https://freyaui.dev/demo).
#[derive(Clone, PartialEq)]
pub struct Input {
    pub(crate) theme_colors: Option<InputColorsThemePartial>,
    pub(crate) theme_layout: Option<InputLayoutThemePartial>,
    value: InputValue,
    placeholder: Option<Cow<'static, str>>,
    on_validate: Option<EventHandler<InputValidator>>,
    on_submit: Option<EventHandler<String>>,
    mode: InputMode,
    auto_focus: bool,
    width: Size,
    height: Size,
    multiline: bool,
    enabled: bool,
    key: DiffKey,
    style_variant: InputStyleVariant,
    layout_variant: InputLayoutVariant,
    text_align: TextAlign,
    a11y_id: Option<AccessibilityId>,
    leading: Option<Element>,
    trailing: Option<Element>,
    on_pre_key_down: Option<Callback<Event<KeyboardEventData>, bool>>,
}

impl KeyExt for Input {
    fn key_mut(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Input {
    pub fn new(value: impl Into<WritableSetter<String>>) -> Self {
        Self::with_inner(InputValue::Value(value.into()))
    }

    /// Use an external editor to control text, cursor, selection and history.
    ///
    /// ```rust
    /// # use freya::{prelude::*, text_edit::*};
    /// fn app() -> impl IntoElement {
    ///     let editor = use_editable(String::new, EditableConfig::new);
    ///     Input::new_with_editor(editor)
    /// }
    /// ```
    pub fn new_with_editor(editor: UseEditable) -> Self {
        Self::with_inner(InputValue::Editor(editor))
    }

    fn with_inner(value: InputValue) -> Self {
        Input {
            theme_colors: None,
            theme_layout: None,
            value,
            placeholder: None,
            on_validate: None,
            on_submit: None,
            mode: InputMode::default(),
            auto_focus: false,
            width: Size::px(150.),
            height: Size::default(),
            multiline: false,
            enabled: true,
            key: DiffKey::default(),
            style_variant: InputStyleVariant::Normal,
            layout_variant: InputLayoutVariant::Normal,
            text_align: TextAlign::default(),
            a11y_id: None,
            leading: None,
            trailing: None,
            on_pre_key_down: None,
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

    pub fn height(mut self, height: impl Into<Size>) -> Self {
        self.height = height.into();
        self
    }

    /// Allow writing multiple lines, `Enter` inserts a line break instead of submitting.
    /// Combine it with [Self::height] to get a scrollable input of a bounded size.
    pub fn multiline(mut self, multiline: impl Into<bool>) -> Self {
        self.multiline = multiline.into();
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

    /// Optional element rendered before the text input.
    pub fn leading(mut self, leading: impl Into<Element>) -> Self {
        self.leading = Some(leading.into());
        self
    }

    /// Optional element rendered after the text input.
    pub fn trailing(mut self, trailing: impl Into<Element>) -> Self {
        self.trailing = Some(trailing.into());
        self
    }

    /// Sets a pre-handler called for each key event. Return `true` to let the input process it,
    /// `false` to skip. The callback may call `stop_propagation()` / `prevent_default()` directly.
    pub fn on_pre_key_down(
        mut self,
        on_pre_key_down: impl Into<Callback<Event<KeyboardEventData>, bool>>,
    ) -> Self {
        self.on_pre_key_down = Some(on_pre_key_down.into());
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
        let a11y_id = use_hook(|| self.a11y_id.unwrap_or_else(AccessibilityId::new_unique));
        let focus = use_focus(a11y_id);
        let holder = use_state(ParagraphHolder::default);
        let mut area = use_state(Area::default);
        let mut viewport_area = use_state(Area::default);
        let mut scroll_controller = use_scroll_controller(ScrollConfig::default);
        let mut status = use_state(InputStatus::default);
        let is_masked = matches!(self.mode, InputMode::Hidden(_));
        let (mut editable, value) = match &self.value {
            InputValue::Value(value) => (
                use_editable(
                    || value.read().to_string(),
                    || {
                        EditableConfig::new()
                            .with_allow_write_clipboard(!is_masked)
                            .with_select_all_on_double_click(is_masked)
                    },
                ),
                Some(value.clone()),
            ),
            InputValue::Editor(editor) => (*editor, None),
        };
        let mut is_dragging = use_state(|| false);

        let theme_colors = match self.style_variant {
            InputStyleVariant::Normal => {
                get_theme!(&self.theme_colors, InputColorsThemePreference, "input")
            }
            InputStyleVariant::Filled => get_theme!(
                &self.theme_colors,
                InputColorsThemePreference,
                "filled_input"
            ),
            InputStyleVariant::Flat => {
                get_theme!(&self.theme_colors, InputColorsThemePreference, "flat_input")
            }
        };
        let theme_layout = match self.layout_variant {
            InputLayoutVariant::Normal => get_theme!(
                &self.theme_layout,
                InputLayoutThemePreference,
                "input_layout"
            ),
            InputLayoutVariant::Compact => get_theme!(
                &self.theme_layout,
                InputLayoutThemePreference,
                "compact_input_layout"
            ),
            InputLayoutVariant::Expanded => get_theme!(
                &self.theme_layout,
                InputLayoutThemePreference,
                "expanded_input_layout"
            ),
        };

        let enabled = use_reactive(&self.enabled);
        let focused = enabled() && focus().is_focused();
        let (mut movement_timeout, cursor_color) = use_cursor_blink(focused, theme_colors.color);

        let on_validate = self.on_validate.clone();
        let on_submit = self.on_submit.clone();

        if let Some(value) = &value
            && *value.read() != editable.editor().read().committed_text()
        {
            let mut editor = editable.editor_mut().write();
            editor.clear_preedit();
            editor.set(&value.read());
            editor.editor_history_mut().clear();
            editor.clear_selection();
        } else if !a11y_id.is_focused() && editable.editor().peek().has_preedit() {
            editable.editor_mut().write().clear_preedit();
        }

        let display_placeholder = editable.editor().read().committed_text().is_empty()
            && self.placeholder.is_some()
            && !editable.editor().read().has_preedit();
        let mode = self.mode;
        let text_align = self.text_align;
        let padding = theme_layout.padding;
        let multiline = self.multiline;
        let mut follow_cursor = move || {
            if !a11y_id.is_focused() || display_placeholder {
                return;
            }

            let holder = holder.peek();
            let holder = holder.0.borrow();
            let Some(ParagraphHolderInner {
                paragraph,
                scale_factor,
                ..
            }) = holder.as_ref()
            else {
                warn!("Paragraph should be build by now.");
                return;
            };

            let viewport = viewport_area();
            if viewport.width() == 0. {
                return;
            }

            // Text as currently displayed
            let editor = editable.editor().peek();
            let text = match mode {
                InputMode::Hidden(character) => {
                    character.to_string().repeat(editor.rope().len_chars())
                }
                InputMode::Shown => editor.rope().to_string(),
            };

            let cursor_rect = paragraph.cursor_rect(&text, editor.cursor_pos(), text_align);
            let cursor_location =
                Point2D::new(cursor_rect.left, cursor_rect.top) / (*scale_factor as f32);
            let visible_cursor_location =
                cursor_location + Vector2D::new(padding.left(), padding.top());
            // Visible window start
            let visible_start_x = viewport.min_x() - area.peek().min_x();

            // Minimally reveal the cursor
            if visible_cursor_location.x < visible_start_x {
                scroll_controller.scroll_to_x(cursor_location.x.into());
            } else if visible_cursor_location.x > visible_start_x + viewport.width() {
                scroll_controller.scroll_to_x(
                    (visible_cursor_location.x + padding.right() - viewport.width()).into(),
                );
            }

            if multiline {
                let cursor_bottom = cursor_rect.bottom / (*scale_factor as f32) + padding.bottom();
                let visible_start_y = viewport.min_y() - area.peek().min_y();

                if visible_cursor_location.y < visible_start_y {
                    scroll_controller.scroll_to_y(cursor_location.y.into());
                } else if cursor_bottom > visible_start_y + viewport.height() {
                    scroll_controller.scroll_to_y((cursor_bottom - viewport.height()).into());
                }
            }
        };

        let on_ime_preedit = move |e: Event<ImePreeditEventData>| {
            let data = e.data();
            let mut editor = editable.editor_mut().write();
            if data.text.is_empty() {
                editor.clear_preedit();
            } else {
                let previous_cursor = editor.cursor_pos();
                let text_unchanged = editor
                    .preedit_text()
                    .is_some_and(|text| text == data.text.as_str());
                editor.set_preedit(&data.text);

                if let Some((_, end)) = data.cursor
                    && let Some(suffix) = data.text.get(end..)
                {
                    let cursor = editor.cursor_pos() - suffix.encode_utf16().count();
                    editor.move_cursor_to(cursor);
                }

                if text_unchanged && editor.cursor_pos() != previous_cursor {
                    drop(editor);
                    follow_cursor();
                }
            }
            movement_timeout.reset();
        };

        let on_pre_key_down = self.on_pre_key_down.clone().unwrap_or_else(|| {
            Callback::new(move |e: Event<KeyboardEventData>| match &e.key {
                Key::Named(NamedKey::Enter) if !multiline => true,
                Key::Named(NamedKey::Escape) | Key::Named(NamedKey::Shift) => true,
                Key::Named(NamedKey::Tab) => false,
                _ => {
                    e.stop_propagation();
                    e.prevent_default();
                    true
                }
            })
        });
        let process_edit = EventHandler::new_current(move |(key, modifiers): (Key, Modifiers)| {
            movement_timeout.reset();
            let previous_history_version = editable.editor().peek().editor_history().version;
            let mut value = value.clone();
            let on_validate = on_validate.clone();
            editable.process_event_and_then(
                EditableEvent::KeyDown {
                    key: &key,
                    modifiers,
                    editor_line: Some(EditorLine::SingleParagraph),
                    holder: Some(&holder.read()),
                },
                move || {
                    let text = editable.editor().read().committed_text();

                    let apply_change = match &on_validate {
                        Some(on_validate) => {
                            let mut editor = editable.editor_mut().write();
                            let validator = InputValidator::new(text.clone());
                            on_validate.call(validator.clone());
                            if !validator.is_valid() {
                                if let Some(selection) = editor.undo() {
                                    *editor.selection_mut() = selection;
                                }
                                editor.editor_history_mut().clear_redos();
                            }
                            validator.is_valid()
                        }
                        None => true,
                    };

                    if apply_change && let Some(value) = &mut value {
                        value.set(text);
                    }
                    if editable.editor().peek().editor_history().version == previous_history_version
                    {
                        follow_cursor();
                    }
                },
            );
        });

        let keyboard_edit = process_edit.clone();
        let on_key_down = move |e: Event<KeyboardEventData>| {
            let key = e.key.clone();
            let modifiers = e.modifiers;

            if !on_pre_key_down.call(e) {
                return;
            }

            match &key {
                // On submit
                Key::Named(NamedKey::Enter) if !multiline => {
                    if let Some(on_submit) = &on_submit {
                        let text = editable.editor().peek().committed_text();
                        on_submit.call(text);
                    }
                }
                // On unfocus
                Key::Named(NamedKey::Escape) => {
                    a11y_id.request_unfocus();
                }
                // On change
                _ => {
                    keyboard_edit.call((key, modifiers));
                }
            }
        };

        let mut on_secondary_down = move |e: Event<PointerEventData>| {
            e.stop_propagation();
            a11y_id.request_focus();
            is_dragging.set_if_modified(false);
            movement_timeout.reset();
            let has_selection = editable.editor().peek().get_selection_range().is_some();
            let menu_button = |title: &'static str, key: Key| {
                let process_edit = process_edit.clone();
                let select_all = matches!(&key, Key::Character(shortcut) if shortcut == "a");
                MenuButton::new()
                    .child(title)
                    .on_press(move |event: Event<PressEventData>| {
                        if select_all {
                            event.stop_propagation();
                            event.prevent_default();
                        }
                        process_edit.call((key.clone(), Modifiers::ctrl_or_meta()));
                        ContextMenu::close();
                        a11y_id.request_focus();
                    })
            };
            ContextMenu::open_from_down(
                Menu::new()
                    .children(
                        [
                            (
                                "Cut",
                                Key::Character("x".into()),
                                has_selection && !is_masked,
                            ),
                            ("Copy", Key::Character("c".into()), !is_masked),
                            ("Paste", Key::Character("v".into()), true),
                            ("Delete", Key::Named(NamedKey::Delete), has_selection),
                        ]
                        .into_iter()
                        .filter(|(_, _, visible)| *visible)
                        .map(|(title, key, _)| menu_button(title, key)),
                    )
                    .child(
                        rect()
                            .margin(Gaps::new(4., 0., 4., 0.))
                            .height(Size::px(1.))
                            .width(Size::fill_minimum())
                            .background(theme_colors.border_fill),
                    )
                    .child(menu_button("Select all", Key::Character("a".into()))),
            );
        };

        let on_key_up = move |e: Event<KeyboardEventData>| {
            e.stop_propagation();
            editable.process_event(EditableEvent::KeyUp { key: &e.key });
        };

        let mut focus_input = move |location| {
            ContextMenu::close();
            if !cfg!(target_os = "android") || a11y_id.is_focused() {
                is_dragging.set_if_modified(true);
            }
            movement_timeout.reset();

            if !display_placeholder {
                editable.process_event(EditableEvent::Down {
                    location,
                    editor_line: EditorLine::SingleParagraph,
                    holder: &holder.read(),
                });
            }
            a11y_id.request_focus();
        };

        let mut on_input_focus_press = move |e: Event<PointerEventData>| {
            e.stop_propagation();
            e.prevent_default();
            let text_area = area.read().to_f64();
            let global_location = e.global_location().clamp(text_area.min(), text_area.max());
            focus_input((global_location - text_area.min()).to_point());
        };

        let on_focus_press = move |e: Event<FocusPressEventData>| {
            e.stop_propagation();
            e.prevent_default();
            focus_input(e.element_location());
        };

        let on_global_pointer_move = move |e: Event<PointerEventData>| {
            if a11y_id.is_focused() && *is_dragging.read() {
                let text_area = area.read().to_f64();
                let location = (e.global_location() - text_area.min()).to_point();
                editable.process_event(EditableEvent::Move {
                    location,
                    editor_line: EditorLine::SingleParagraph,
                    holder: &holder.read(),
                });
                follow_cursor();
            }
        };

        let on_pointer_enter = move |_| {
            *status.write() = InputStatus::Hovering;
        };

        let on_pointer_leave = move |_| {
            if status() == InputStatus::Hovering {
                *status.write() = InputStatus::default();
            }
        };

        let on_global_pointer_up = move |_: Event<PointerEventData>| {
            match *status.read() {
                InputStatus::Idle if a11y_id.is_focused() => {
                    editable.process_event(EditableEvent::Release);
                }
                InputStatus::Hovering => {
                    editable.process_event(EditableEvent::Release);
                }
                _ => {}
            };

            if a11y_id.is_focused() {
                if *is_dragging.read() {
                    // The input is focused and dragging, but it just clicked so we assume the dragging can stop
                    is_dragging.set(false);
                } else {
                    // The input is focused but not dragging, so the click means it was clicked outside, therefore we can unfocus this input
                    a11y_id.request_unfocus();
                }
            }
        };

        let on_pointer_press = move |e: Event<PointerEventData>| {
            e.stop_propagation();
            e.prevent_default();
            match *status.read() {
                InputStatus::Idle if a11y_id.is_focused() => {
                    editable.process_event(EditableEvent::Release);
                }
                InputStatus::Hovering => {
                    editable.process_event(EditableEvent::Release);
                }
                _ => {}
            };

            if a11y_id.is_focused() {
                is_dragging.set_if_modified(false);
            }
        };

        let on_paragraph_sized = move |e: Event<SizedEventData>| {
            let viewport = viewport_area();
            if viewport.width() > 0. {
                scroll_controller.apply_layout(e.area.size.to_f64(), viewport.size.to_f64());
            }

            let text_size_changed = area.peek().size != e.area.size;
            area.set_if_modified(e.area);
            if text_size_changed {
                follow_cursor();
            }
        };

        let (background, cursor_index, text_selection) = if focused {
            (
                theme_colors.focus_background,
                Some(editable.editor().read().cursor_pos()),
                editable
                    .editor()
                    .read()
                    .get_visible_selection(EditorLine::SingleParagraph),
            )
        } else {
            (theme_colors.background, None, None)
        };

        let border = if focused {
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

        let value = editable.editor().read().committed_text();
        let a11y_text: Cow<str> = match (self.mode, &self.placeholder) {
            (_, Some(ph)) if display_placeholder => Cow::Borrowed(ph.as_ref()),
            (InputMode::Hidden(ch), _) => Cow::Owned(ch.to_string().repeat(value.len())),
            (InputMode::Shown, _) => Cow::Borrowed(value.as_ref()),
        };

        let a11_role = match self.mode {
            InputMode::Hidden(_) => AccessibilityRole::PasswordInput,
            _ if self.multiline => AccessibilityRole::MultilineTextInput,
            _ => AccessibilityRole::TextInput,
        };

        rect()
            .a11y_id(a11y_id)
            .a11y_enabled(self.enabled)
            .a11y_focusable(self.enabled)
            .a11y_auto_focus(self.auto_focus)
            .a11y_alt(a11y_text)
            .a11y_role(a11_role)
            .maybe(self.enabled, |el| {
                el.on_key_up(on_key_up)
                    .on_key_down(on_key_down)
                    .on_pointer_down(move |e: Event<PointerEventData>| {
                        if matches!(e.data(), PointerEventData::Mouse(mouse) if mouse.button == Some(MouseButton::Right)) {
                            on_secondary_down(e);
                        } else if !cfg!(target_os = "android") && match e.data() {
                            PointerEventData::Mouse(mouse) => mouse.button == Some(MouseButton::Left),
                            PointerEventData::Touch(_) => true,
                        } {
                            on_input_focus_press(e);
                        }
                    })
                    .on_ime_preedit(on_ime_preedit)
                    .on_pointer_press(on_pointer_press)
                    .on_global_pointer_up(on_global_pointer_up)
                    .on_global_pointer_move(on_global_pointer_move)
            })
            .on_pointer_enter(on_pointer_enter)
            .on_pointer_leave(on_pointer_leave)
            .cursor(if self.enabled {
                CursorIcon::Text
            } else {
                CursorIcon::NotAllowed
            })
            .width(self.width.clone())
            .height(self.height.clone())
            .background(background.mul_if(!self.enabled, 0.85))
            .border(border)
            .corner_radius(theme_layout.corner_radius)
            .content(Content::Flex)
            .direction(Direction::Horizontal)
            .cross_align(Alignment::center())
            .maybe_child(
                self.leading
                    .clone()
                    .map(|leading| rect().padding(Gaps::new(0., 0., 0., 8.)).child(leading)),
            )
            .child(
                ScrollView::new_controlled(scroll_controller)
                    .width(Size::flex(1.))
                    .height(if self.multiline && !matches!(self.height, Size::Inner) {
                        Size::fill()
                    } else {
                        Size::Inner
                    })
                    .direction(if self.multiline {
                        Direction::Vertical
                    } else {
                        Direction::Horizontal
                    })
                    .show_scrollbar(self.multiline)
                    .on_sized(move |e: Event<SizedEventData>| viewport_area.set_if_modified(e.area))
                    .child(
                        paragraph()
                            .holder(holder.read().clone())
                            .on_sized(on_paragraph_sized)
                            .min_width(Size::percent(100.))
                            .maybe(self.multiline, |el| el.max_width(Size::percent(100.)))
                            .maybe(self.enabled, |el| el.on_focus_press(on_focus_press))
                            .padding(theme_layout.padding)
                            .cursor_index(cursor_index)
                            .cursor_color(cursor_color)
                            .color(color)
                            .text_align(self.text_align)
                            .maybe(!self.multiline, |el| el.max_lines(1))
                            .highlights(text_selection.map(|h| vec![h]))
                            .maybe(display_placeholder, |el| {
                                el.span(self.placeholder.as_ref().unwrap().to_string())
                            })
                            .maybe(!display_placeholder, |el| {
                                let editor = editable.editor().read();
                                if editor.has_preedit() {
                                    let (b, p, a) = editor.preedit_text_segments();
                                    let (b, p, a) = match self.mode {
                                        InputMode::Hidden(ch) => {
                                            let ch = ch.to_string();
                                            (
                                                ch.repeat(b.chars().count()),
                                                ch.repeat(p.chars().count()),
                                                ch.repeat(a.chars().count()),
                                            )
                                        }
                                        InputMode::Shown => (b, p, a),
                                    };
                                    el.span(b)
                                        .span(
                                            Span::new(p).text_decoration(TextDecoration::Underline),
                                        )
                                        .span(a)
                                } else {
                                    let text = match self.mode {
                                        InputMode::Hidden(ch) => {
                                            ch.to_string().repeat(editor.rope().len_chars())
                                        }
                                        InputMode::Shown => editor.rope().to_string(),
                                    };
                                    el.span(text)
                                }
                            }),
                    ),
            )
            .maybe_child(
                self.trailing
                    .clone()
                    .map(|trailing| rect().padding(Gaps::new(0., 8., 0., 0.)).child(trailing)),
            )
    }

    fn render_key(&self) -> DiffKey {
        let key = self.key.clone().or(self.default_key());
        let mut hasher = std::hash::DefaultHasher::new();
        key.hash(&mut hasher);
        std::mem::discriminant(&self.value).hash(&mut hasher);
        match key {
            DiffKey::DefaultU64(_) => DiffKey::DefaultU64(hasher.finish()),
            _ => DiffKey::U64(hasher.finish()),
        }
    }
}
