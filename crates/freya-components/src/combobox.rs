use std::borrow::Cow;

use freya_core::prelude::*;
use freya_edit::{
    EditableConfig,
    TextEditor,
    use_editable,
};
use torin::prelude::*;

use crate::{
    define_theme,
    get_theme,
    icons::{
        IconThemePartialExt,
        arrow::ArrowIcon,
    },
    input::{
        Input,
        InputColorsThemePartialExt,
        InputLayoutThemePartialExt,
        InputValidator,
    },
    menu::{
        MenuGroup,
        MenuItem,
    },
};

define_theme! {
    %[component]
    pub ComboBox {
        %[fields]
        font_size: f32,
        corner_radius: CornerRadius,
        list_corner_radius: CornerRadius,
        width: Size,
        margin: Gaps,
        padding: Gaps,
        list_background: Color,
        background: Color,
        focus_background: Color,
        border_fill: Color,
        focus_border_fill: Color,
        arrow_fill: Color,
        color: Color,
        placeholder_color: Color,
    }
}

/// An editable value with case-insensitive, substring-matched suggestions.
///
/// Click or type to open suggestions. Use ArrowDown/ArrowUp to browse,
/// Enter to select and Escape or Tab to dismiss. Only selecting an item updates
/// the bound value, dismissing restores the current selection.
///
/// Override the field and dropdown styles with [`ComboBox::theme`].
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     let value = use_state(String::new);
///     ComboBox::new(value, vec!["Rust".to_string(), "Ruby".to_string()])
///         .placeholder("Choose a language")
/// }
/// ```
#[derive(Clone, PartialEq)]
pub struct ComboBox {
    value: WritableSetter<String>,
    options: Readable<Vec<String>>,
    placeholder: Option<Cow<'static, str>>,
    theme: Option<ComboBoxThemePartial>,
    key: DiffKey,
}

impl ComboBox {
    pub fn new(
        value: impl Into<WritableSetter<String>>,
        options: impl Into<Readable<Vec<String>>>,
    ) -> Self {
        Self {
            value: value.into(),
            options: options.into(),
            placeholder: None,
            theme: None,
            key: DiffKey::None,
        }
    }

    pub fn placeholder(mut self, placeholder: impl Into<Cow<'static, str>>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn theme(mut self, theme: ComboBoxThemePartial) -> Self {
        self.theme = Some(theme);
        self
    }
}

impl KeyExt for ComboBox {
    fn key_mut(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl ComponentOwned for ComboBox {
    fn render(self) -> impl IntoElement {
        let a11y_id = use_a11y();
        let mut open = use_state(|| true);
        let mut highlighted = use_state(|| None::<usize>);
        let mut editable = use_editable(|| self.value.peek().clone(), EditableConfig::new);
        let mut input_area = use_state(|| None::<Area>);
        let mut list_size = use_state(|| None::<Size2D>);
        let theme = get_theme!(&self.theme, ComboBoxThemePreference, "combobox");
        use_provide_context(|| MenuGroup { group_id: a11y_id });
        let focus = use_focus(a11y_id);
        let is_focused = focus().is_focused();
        if !is_focused {
            open.set_if_modified(true);
            highlighted.set_if_modified(None);
        }
        let visible = is_focused && open();
        if !visible {
            let value = self.value.read();
            if editable.editor().peek().committed_text() != *value {
                let mut editor = editable.editor_mut().write();
                editor.clear_preedit();
                editor.set(&value);
                editor.editor_history_mut().clear();
                editor.clear_selection();
            }
        }
        let offset_y = match (input_area(), list_size()) {
            (Some(input), Some(list)) => {
                let root_height = Platform::get().current_window().root_size.peek().height;
                if list.height > root_height - input.max_y() && list.height <= input.min_y() {
                    -(input.height() + list.height)
                } else {
                    0.
                }
            }
            _ => 0.,
        };
        let query = editable.editor().read().committed_text().to_lowercase();
        let suggestions: Vec<String> = self
            .options
            .read()
            .iter()
            .filter(|option| option.to_lowercase().contains(&query))
            .cloned()
            .collect();

        let select_option = {
            let mut value = self.value.clone();
            move |option| {
                value.set(option);
                let value = value.peek();
                let mut editor = editable.editor_mut().write();
                editor.clear_preedit();
                if editor.committed_text() != *value {
                    editor.set(&value);
                    editor.editor_history_mut().clear();
                }
                editor.clear_selection();
                let end = editor.len_utf16_cu();
                editor.move_cursor_to(end);
                open.set(false);
                highlighted.set(None);
            }
        };

        rect()
            .margin(theme.margin)
            .on_sized(move |event: Event<SizedEventData>| {
                input_area.set_if_modified(Some(event.area));
            })
            .on_capture_global_pointer_up(move |event: Event<PointerEventData>| {
                if input_area
                    .peek()
                    .is_some_and(|area| area.contains(event.global_location().to_f32()))
                {
                    a11y_id.request_focus();
                    highlighted.set(None);
                    open.set(true);
                }
            })
            .on_global_pointer_up(move |event: Event<PointerEventData>| {
                if focus().is_focused()
                    && !input_area
                        .peek()
                        .is_some_and(|area| area.contains(event.global_location().to_f32()))
                {
                    open.set_if_modified(false);
                }
            })
            .child(
                Input::new_with_editor(editable)
                    .on_validate({
                        let value = self.value.clone();
                        move |validator: InputValidator| {
                            if *validator.text() != *value.peek() {
                                open.set_if_modified(true);
                                highlighted.set_if_modified(None);
                            }
                        }
                    })
                    .a11y_id(a11y_id)
                    .width(theme.width)
                    .font_size(theme.font_size)
                    .corner_radius(theme.corner_radius)
                    .padding(theme.padding)
                    .background(theme.background)
                    .focus_background(theme.focus_background)
                    .border_fill(theme.border_fill)
                    .focus_border_fill(theme.focus_border_fill)
                    .color(theme.color)
                    .placeholder_color(theme.placeholder_color)
                    .map(self.placeholder, |el, placeholder| {
                        el.placeholder(placeholder)
                    })
                    .trailing(
                        rect()
                            .padding(4.)
                            .on_press(move |event: Event<PressEventData>| {
                                event.prevent_default();
                                event.stop_propagation();
                                a11y_id.request_focus();
                                open.set(true);
                            })
                            .child(ArrowIcon::new().fill(theme.arrow_fill).rotate(if visible {
                                180.
                            } else {
                                0.
                            })),
                    )
                    .on_pre_key_down({
                        let suggestions = suggestions.clone();
                        let mut select_option = select_option.clone();
                        move |event: Event<KeyboardEventData>| {
                            match event.key {
                                Key::Named(NamedKey::ArrowDown | NamedKey::ArrowUp) => {
                                    let count = suggestions.len();
                                    if count > 0 {
                                        let next = match (*highlighted.peek(), &event.key) {
                                            (Some(index), Key::Named(NamedKey::ArrowUp)) => {
                                                index.min(count).checked_sub(1).unwrap_or(count - 1)
                                            }
                                            (Some(index), _) => (index + 1) % count,
                                            (None, Key::Named(NamedKey::ArrowUp)) => count - 1,
                                            (None, _) => 0,
                                        };
                                        highlighted.set(Some(next));
                                    }
                                    open.set(true);
                                }
                                Key::Named(NamedKey::Enter) if open() => {
                                    let selected = *highlighted.peek();
                                    if let Some(option) =
                                        selected.and_then(|index| suggestions.get(index))
                                    {
                                        select_option(option.clone());
                                    } else {
                                        open.set(false);
                                        highlighted.set(None);
                                    }
                                }
                                Key::Named(NamedKey::Escape) if open() => {
                                    open.set(false);
                                    highlighted.set(None);
                                }
                                Key::Named(NamedKey::Tab) => {
                                    open.set(false);
                                    return false;
                                }
                                Key::Named(
                                    NamedKey::Enter | NamedKey::Escape | NamedKey::Shift,
                                ) => {
                                    return true;
                                }
                                _ => {
                                    event.prevent_default();
                                    event.stop_propagation();
                                    return true;
                                }
                            }
                            event.prevent_default();
                            event.stop_propagation();
                            false
                        }
                    }),
            )
            .maybe_child((visible && !suggestions.is_empty()).then(|| {
                rect().height(Size::px(0.)).width(Size::px(0.)).child(
                    rect()
                        .width(Size::px(input_area().map_or(0., |area| area.width())))
                        .margin((4., 0., 4., 0.))
                        .offset_y(offset_y)
                        .on_sized(move |event: Event<SizedEventData>| {
                            list_size.set_if_modified(Some(event.area.size));
                        })
                        .child(
                            rect()
                                .layer(Layer::Overlay)
                                .width(Size::fill())
                                .min_width(Size::px(input_area().map_or(0., |area| area.width())))
                                .shadow(Shadow::new().y(6.).blur(18.).color((0, 0, 0, 0.25)))
                                .border(
                                    Border::new()
                                        .fill(theme.border_fill)
                                        .width(1.)
                                        .alignment(BorderAlignment::Inner),
                                )
                                .overflow(Overflow::Clip)
                                .corner_radius(theme.list_corner_radius)
                                .background(theme.list_background)
                                .padding(6.)
                                .opacity(if list_size().is_some() { 1. } else { 0. })
                                .children(suggestions.into_iter().enumerate().map(
                                    |(index, option)| {
                                        MenuItem::new()
                                            .key(index)
                                            .selected(*highlighted.read() == Some(index))
                                            .child(option.clone())
                                            .on_press({
                                                let mut select_option = select_option.clone();
                                                move |_| {
                                                    select_option(option.clone());
                                                    a11y_id.request_focus();
                                                }
                                            })
                                    },
                                )),
                        ),
                )
            }))
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
