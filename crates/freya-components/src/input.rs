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
    gaps::Gaps,
    prelude::{
        Alignment,
        Area,
        Content,
        Direction,
    },
    size::Size,
};

use crate::{
    cursor_blink::use_cursor_blink,
    define_theme,
    get_theme,
    scrollviews::ScrollView,
};

define_theme! {
    for = Input;
    theme_field = theme_layout;

    %[component]
    pub InputLayout {
        %[fields]
        corner_radius: CornerRadius,
        inner_margin: Gaps,
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
    select_all_on_init: bool,
    caret: Option<Writable<usize>>,
    width: Size,
    height: Size,
    enabled: bool,
    key: DiffKey,
    style_variant: InputStyleVariant,
    layout_variant: InputLayoutVariant,
    multiline: bool,
    min_height: Option<f32>,
    max_height: Option<f32>,
    text_align: TextAlign,
    a11y_id: Option<AccessibilityId>,
    leading: Option<Element>,
    trailing: Option<Element>,
    on_pre_key_down: Callback<Event<KeyboardEventData>, bool>,
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
            select_all_on_init: false,
            caret: None,
            width: Size::px(150.),
            height: Size::Inner,
            enabled: true,
            key: DiffKey::default(),
            style_variant: InputStyleVariant::Normal,
            layout_variant: InputLayoutVariant::Normal,
            multiline: false,
            min_height: None,
            max_height: None,
            text_align: TextAlign::default(),
            a11y_id: None,
            leading: None,
            trailing: None,
            on_pre_key_down: Callback::new(Self::key_down_default),
        }
    }

    /// What the input does with a key press when nothing has claimed it: the rule
    /// [`on_pre_key_down`](Self::on_pre_key_down) applies unless it is replaced.
    ///
    /// A surface that overrides that callback usually wants to claim **two or three** keys and
    /// leave the rest alone, so it is public: hand anything you did not claim to this rather than
    /// re-deciding it, or an override quietly changes what `Tab` does to focus and lets every
    /// keystroke through to the global listeners behind the input.
    pub fn key_down_default(e: Event<KeyboardEventData>) -> bool {
        match &e.key {
            Key::Named(NamedKey::Enter) | Key::Named(NamedKey::Escape) => true,
            Key::Named(NamedKey::Tab) => false,
            _ => {
                e.stop_propagation();
                e.prevent_default();
                true
            }
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

    /// Start with the value selected rather than the cursor at its beginning, so the first
    /// keystroke replaces it.
    ///
    /// For an input that opens over text the user came to replace (a rename affordance seeded
    /// with the current name) this is the difference between typing *over* that name and typing
    /// in front of it. Pair it with [`auto_focus`](Self::auto_focus).
    ///
    /// Applies to the value the input **mounts** with. A value that changes underneath it later
    /// syncs as a plain edit, cursor and selection untouched.
    pub fn select_all_on_init(mut self, select_all_on_init: bool) -> Self {
        self.select_all_on_init = select_all_on_init;
        self
    }

    /// **Two-way binding for the cursor**, in UTF-16 code units, exactly like the offsets
    /// [`freya_edit::TextEditor`] takes.
    ///
    /// The input writes the cursor out whenever it moves, by any means: typing, the arrow keys, a
    /// press, a drag, an IME commit. A value written from *outside* moves the cursor there.
    ///
    /// Bound alongside the value, this is what lets a caller make an edit **on the user's behalf**
    /// land the way the user's own edits do: replace the span you meant to replace, then put the
    /// cursor at the end of what you inserted. Accepting a completion is the case this exists for,
    /// and without it such a caller can only rewrite the whole value and hope the cursor was at
    /// the end of it.
    pub fn caret(mut self, caret: impl Into<Writable<usize>>) -> Self {
        self.caret = Some(caret.into());
        self
    }

    pub fn mode(mut self, mode: InputMode) -> Self {
        self.mode = mode;
        self
    }

    /// Let the value wrap and hold newlines, instead of being one scrolling line.
    ///
    /// The text wraps at the input's width and the box grows with it, so pair this with
    /// [`height`](Self::height) left at its default [`Size::Inner`] and a `max_height` on
    /// whatever contains it: past that the text scrolls vertically rather than the box growing
    /// without end.
    ///
    /// **Enter still submits, and `Shift`+`Enter` inserts a newline.** A multiline input that
    /// took Enter as text would have no way to submit at all, and the shifted variant is what
    /// every composer in the category binds. Without an
    /// [`on_submit`](Self::on_submit) there is nothing to submit *to*, so there Enter inserts a
    /// newline as well and the modifier is simply redundant.
    pub fn multiline(mut self, multiline: bool) -> Self {
        self.multiline = multiline;
        self
    }

    /// The tallest a [`multiline`](Self::multiline) input may grow before its text scrolls
    /// instead.
    ///
    /// Growing with the content is what a composer wants and growing without end is not, so the
    /// two halves are one setting: below this the box is exactly as tall as its wrapped text,
    /// and at it the box stops and the text scrolls inside. Ignored by a single-line input,
    /// which has nothing to grow.
    pub fn max_height(mut self, max_height: f32) -> Self {
        self.max_height = Some(max_height);
        self
    }

    /// The shortest a [`multiline`](Self::multiline) input may be, however little is in it.
    ///
    /// The floor to [`max_height`](Self::max_height)'s ceiling, for a box the surface wants at a
    /// stated size rather than at its text's: an expanded composer is that size whether it holds
    /// one line or forty, which growth alone cannot express: raising only the ceiling gives an
    /// empty box nothing to grow into. Ignored by a single-line input.
    pub fn min_height(mut self, min_height: f32) -> Self {
        self.min_height = Some(min_height);
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

    /// Set the input's height.
    ///
    /// Defaults to [`Size::Inner`], which is the input sized by its own content: the text line
    /// box plus the layout theme's `inner_margin`. Set this when the input has to stand at a
    /// height the surface dictates rather than the one its text happens to produce, for example
    /// beside a control of a fixed size in a form row.
    ///
    /// Prefer this to wrapping the input in a sized container. A wrapper cannot change the
    /// input's own box, so it only centres a differently sized input inside itself, and the
    /// background, border and corner radius stay at the content height.
    pub fn height(mut self, height: impl Into<Size>) -> Self {
        self.height = height.into();
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
        self.on_pre_key_down = on_pre_key_down.into();
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
        let mut status = use_state(InputStatus::default);
        let is_masked = matches!(self.mode, InputMode::Hidden(_));
        let mut editable = use_editable(
            || self.value.read().to_string(),
            move || {
                EditableConfig::new()
                    .with_allow_write_clipboard(!is_masked)
                    .with_select_all_on_double_click(is_masked)
                    .with_select_all_on_init(self.select_all_on_init)
            },
        );
        let mut is_dragging = use_state(|| false);
        let mut value = self.value.clone();

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

        let (mut movement_timeout, cursor_color) =
            use_cursor_blink(focus() != Focus::Not, theme_colors.color);

        let enabled = use_reactive(&self.enabled);
        use_drop(move || {
            if status() == InputStatus::Hovering && enabled() {
                Cursor::set(CursorIcon::default());
            }
        });

        // **What a multiline box grows to.** The paragraph reports its laid-out height and the
        // box takes it, clamped by [`Input::max_height`], so the box is exactly as tall as its
        // text until the cap, and the `ScrollView` inside takes over from there. Written only on
        // an actual change, or each layout pass would schedule the next one.
        let mut content_height = use_state(|| 0.);
        let multiline_layout = self.multiline;
        let resolved_height = match (self.multiline, self.max_height) {
            (true, Some(max)) => Size::px(
                (*content_height.read() + theme_layout.inner_margin.vertical())
                    .clamp(self.min_height.unwrap_or(0.).min(max), max),
            ),
            (true, None) => Size::Inner,
            (false, _) => self.height.clone(),
        };

        let display_placeholder = value.read().is_empty()
            && self.placeholder.is_some()
            && !editable.editor().read().has_preedit();
        let on_validate = self.on_validate.clone();
        let on_submit = self.on_submit.clone();

        if *value.read() != editable.editor().read().committed_text() {
            let mut editor = editable.editor_mut().write();
            editor.clear_preedit();
            editor.set(&value.read());
            editor.editor_history().clear();
            editor.clear_selection();
        }

        // **The caret binding, both ways.** Publishing is an effect over the editor signal rather
        // than a line in each handler, so every way the cursor can move is covered by
        // construction: the arrow keys and a press move it without changing a character, and a
        // publish hung off the change path alone would miss both.
        //
        // `published` is what tells an outside write from this input's own echo. The effect runs
        // *after* the render that consumes, so without it the render following a keystroke would
        // read a caret the effect had not yet caught up to and drag the cursor back a character.
        let mut published = use_state(|| None::<usize>);
        {
            let mut bound = self.caret.clone();
            use_side_effect(move || {
                // Unconditionally registered, and it reads nothing when nothing is bound: a hook
                // must run the same number of times on every render, and an unbound input must
                // not subscribe to its own cursor for an effect with no work to do.
                let Some(caret) = bound.as_mut() else {
                    return;
                };
                let pos = editable.editor().read().cursor_pos();
                if *caret.peek() != pos {
                    caret.set(pos);
                }
                published.set(Some(pos));
            });
        }
        // Consumed here, beside the value sync, because a caller that rewrites the text and moves
        // the cursor writes both in one go and the text has to land first.
        if let Some(caret) = &self.caret {
            let wanted = *caret.read();
            if *published.peek() != Some(wanted) {
                let mut editor = editable.editor_mut().write();
                let wanted = wanted.min(editor.len_utf16_cu());
                editor.move_cursor_to(wanted);
                published.set(Some(wanted));
            }
        }

        let on_ime_preedit = move |e: Event<ImePreeditEventData>| {
            let mut editor = editable.editor_mut().write();
            if e.data().text.is_empty() {
                editor.clear_preedit();
            } else {
                editor.set_preedit(&e.data().text);
            }
        };

        let on_pre_key_down = self.on_pre_key_down.clone();
        let multiline = self.multiline;
        let on_key_down = move |e: Event<KeyboardEventData>| {
            let key = e.key.clone();
            let modifiers = e.modifiers;

            if !on_pre_key_down.call(e) {
                return;
            }

            // A multiline input takes `Shift`+`Enter` as a newline and leaves plain Enter to
            // submit, and with nothing to submit to, takes both as a newline. See
            // [`Input::multiline`].
            let newline = multiline
                && matches!(key, Key::Named(NamedKey::Enter))
                && (modifiers.shift() || on_submit.is_none());

            match &key {
                // On submit
                Key::Named(NamedKey::Enter) if !newline => {
                    if let Some(on_submit) = &on_submit {
                        let text = editable.editor().peek().committed_text();
                        on_submit.call(text);
                    }
                }
                // On unfocus
                Key::Named(NamedKey::Escape) => {
                    a11y_id.request_unfocus();
                    Cursor::set(CursorIcon::default());
                }
                // On change
                _ => {
                    movement_timeout.reset();
                    editable.process_event(EditableEvent::KeyDown {
                        key: &key,
                        modifiers,
                    });
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
        };

        let on_key_up = move |e: Event<KeyboardEventData>| {
            e.stop_propagation();
            editable.process_event(EditableEvent::KeyUp { key: &e.key });
        };

        let on_input_focus_press = move |e: Event<FocusPressEventData>| {
            e.stop_propagation();
            e.prevent_default();
            if cfg!(target_os = "android") {
                if a11y_id.is_focused() {
                    // Require a second press to enabling dragging on Android
                    is_dragging.set_if_modified(true);
                }
            } else {
                is_dragging.set_if_modified(true);
            }
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
            a11y_id.request_focus();
        };

        let on_focus_press = move |e: Event<FocusPressEventData>| {
            e.stop_propagation();
            e.prevent_default();
            if cfg!(target_os = "android") {
                if a11y_id.is_focused() {
                    // Require a second press to enabling dragging on Android
                    is_dragging.set_if_modified(true);
                }
            } else {
                is_dragging.set_if_modified(true);
            }
            movement_timeout.reset();
            if !display_placeholder {
                editable.process_event(EditableEvent::Down {
                    location: e.element_location(),
                    editor_line: EditorLine::SingleParagraph,
                    holder: &holder.read(),
                });
            }
            a11y_id.request_focus();
        };

        let on_global_pointer_move = move |e: Event<PointerEventData>| {
            if a11y_id.is_focused() && *is_dragging.read() {
                let mut location = e.global_location();
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

        let on_global_pointer_press = move |_: Event<PointerEventData>| {
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

        let (background, cursor_index, text_selection) = if enabled() && focus() != Focus::Not {
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

        let border = if focus().is_focused() {
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
        let a11y_text: Cow<str> = match (self.mode.clone(), &self.placeholder) {
            (_, Some(ph)) if display_placeholder => Cow::Borrowed(ph.as_ref()),
            (InputMode::Hidden(ch), _) => Cow::Owned(ch.to_string().repeat(value.len())),
            (InputMode::Shown, _) => Cow::Borrowed(value.as_ref()),
        };

        let a11_role = match self.mode {
            InputMode::Hidden(_) => AccessibilityRole::PasswordInput,
            _ => AccessibilityRole::TextInput,
        };

        rect()
            .a11y_id(a11y_id)
            .a11y_focusable(self.enabled)
            .a11y_auto_focus(self.auto_focus)
            .a11y_alt(a11y_text)
            .a11y_role(a11_role)
            .maybe(self.enabled, |el| {
                el.on_key_up(on_key_up)
                    .on_key_down(on_key_down)
                    .on_focus_press(on_input_focus_press)
                    .on_ime_preedit(on_ime_preedit)
                    .on_pointer_press(on_pointer_press)
                    .on_global_pointer_press(on_global_pointer_press)
                    .on_global_pointer_move(on_global_pointer_move)
            })
            .on_pointer_enter(on_pointer_enter)
            .on_pointer_leave(on_pointer_leave)
            .width(self.width.clone())
            .height(resolved_height)
            .background(background.mul_if(!self.enabled, 0.85))
            .border(border)
            .corner_radius(theme_layout.corner_radius)
            .content(Content::Flex)
            .direction(Direction::Horizontal)
            // A multiline box's leading/trailing sit at the *top*, beside the first line, rather
            // than floating in the middle of a block that may be ten lines tall.
            .cross_align(match self.multiline {
                true => Alignment::Start,
                false => Alignment::center(),
            })
            .maybe_child(
                self.leading
                    .clone()
                    .map(|leading| rect().padding(Gaps::new(0., 0., 0., 8.)).child(leading)),
            )
            .child(
                ScrollView::new()
                    .width(Size::flex(1.))
                    // A multiline input scrolls the way its text runs: down, and with a
                    // scrollbar, because a wrapped block that has outgrown its box gives the
                    // reader no other clue that there is more of it.
                    .height(match self.multiline {
                        true => Size::fill(),
                        false => Size::Inner,
                    })
                    .direction(match self.multiline {
                        true => Direction::Vertical,
                        false => Direction::Horizontal,
                    })
                    .show_scrollbar(self.multiline)
                    .child(
                        paragraph()
                            .holder(holder.read().clone())
                            .on_sized(move |e: Event<SizedEventData>| {
                                area.set(e.visible_area);
                                // **The paragraph's own laid-out height.** `area` is the right
                                // signal here and `inner_sizes` is not: a paragraph's children
                                // are spans rather than laid-out nodes, so its accumulated inner
                                // size measures ~0 and a box following it collapses to its
                                // margins. The `ScrollView` above does not force this paragraph
                                // to fill, so `area` is the text's height rather than the box's,
                                // and the feedback settles rather than ratcheting.
                                if multiline_layout && *content_height.peek() != e.area.height() {
                                    content_height.set(e.area.height());
                                }
                            })
                            // A single-line input's text runs as wide as it likes and the
                            // `ScrollView` carries it sideways, so it takes a *minimum* width and
                            // no maximum. A multiline one has to **wrap**: fill the box, so the
                            // text breaks at its edge and the only axis that ever scrolls is the
                            // one the lines run down.
                            .maybe(!multiline_layout, |el| {
                                el.min_width(Size::func(move |context| {
                                    Some(context.parent - theme_layout.inner_margin.horizontal())
                                }))
                            })
                            .maybe(multiline_layout, |el| el.width(Size::fill()))
                            .maybe(self.enabled, |el| el.on_focus_press(on_focus_press))
                            .margin(theme_layout.inner_margin)
                            .cursor_index(cursor_index)
                            .cursor_color(cursor_color)
                            .color(color)
                            .text_align(self.text_align)
                            // Unset when multiline, so the paragraph wraps at the input's width
                            // rather than running off the end of one line.
                            .maybe(!self.multiline, |el| el.max_lines(1))
                            .highlights(text_selection.map(|h| vec![h]))
                            .maybe(display_placeholder, |el| {
                                el.span(self.placeholder.as_ref().unwrap().to_string())
                            })
                            .maybe(!display_placeholder, |el| {
                                let editor = editable.editor().read();
                                if editor.has_preedit() {
                                    let (b, p, a) = editor.preedit_text_segments();
                                    let (b, p, a) = match self.mode.clone() {
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
                                    let text = match self.mode.clone() {
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
        self.key.clone().or(self.default_key())
    }
}
