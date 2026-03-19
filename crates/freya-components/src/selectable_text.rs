use std::borrow::Cow;

use freya_core::prelude::*;
use freya_edit::*;

/// Current status of the SelectableText.
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum SelectableTextStatus {
    /// Default state.
    #[default]
    Idle,
    /// Mouse is hovering the text.
    Hovering,
}

#[derive(Clone, PartialEq)]
pub struct SelectableText {
    value: Cow<'static, str>,
    layout: LayoutData,
    accessibility: AccessibilityData,
    text_style_data: TextStyleData,
    max_lines: Option<usize>,
    line_height: Option<f32>,
    key: DiffKey,
}

impl KeyExt for SelectableText {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl LayoutExt for SelectableText {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.layout
    }
}

impl ContainerExt for SelectableText {}

impl AccessibilityExt for SelectableText {
    fn get_accessibility_data(&mut self) -> &mut AccessibilityData {
        &mut self.accessibility
    }
}

impl TextStyleExt for SelectableText {
    fn get_text_style_data(&mut self) -> &mut TextStyleData {
        &mut self.text_style_data
    }
}

impl SelectableText {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            layout: LayoutData::default(),
            accessibility: AccessibilityData::default(),
            text_style_data: TextStyleData::default(),
            max_lines: None,
            line_height: None,
            key: DiffKey::None,
        }
    }

    pub fn max_lines(mut self, max_lines: impl Into<Option<usize>>) -> Self {
        self.max_lines = max_lines.into();
        self
    }

    pub fn line_height(mut self, line_height: impl Into<Option<f32>>) -> Self {
        self.line_height = line_height.into();
        self
    }
}

impl Component for SelectableText {
    fn render(&self) -> impl IntoElement {
        let holder = use_state(ParagraphHolder::default);
        let mut editable = use_editable(
            || self.value.to_string(),
            move || EditableConfig::new().with_allow_changes(false),
        );
        let mut status = use_state(SelectableTextStatus::default);
        let focus = use_focus();
        let mut drag_origin = use_state(|| None);

        if self.value.as_ref() != editable.editor().read().rope() {
            editable.editor_mut().write().set(self.value.as_ref());
            editable.editor_mut().write().editor_history().clear();
        }

        let highlights = editable
            .editor()
            .read()
            .get_visible_selection(EditorLine::SingleParagraph);

        let on_pointer_down = move |e: Event<PointerEventData>| {
            e.stop_propagation();
            drag_origin.set(Some(e.global_location() - e.element_location()));
            editable.process_event(EditableEvent::Down {
                location: e.element_location(),
                editor_line: EditorLine::SingleParagraph,
                holder: &holder.read(),
            });
            focus.request_focus();
        };

        let on_global_pointer_move = move |e: Event<PointerEventData>| {
            if focus.is_focused()
                && let Some(drag_origin) = drag_origin()
            {
                let mut element_location = e.element_location();
                element_location.x -= drag_origin.x;
                element_location.y -= drag_origin.y;
                editable.process_event(EditableEvent::Move {
                    location: element_location,
                    editor_line: EditorLine::SingleParagraph,
                    holder: &holder.read(),
                });
            }
        };

        let on_global_pointer_down = move |_: Event<PointerEventData>| {
            editable.editor_mut().write().clear_selection();
        };

        let on_pointer_enter = move |_| {
            *status.write() = SelectableTextStatus::Hovering;
        };

        let on_pointer_leave = move |_| {
            *status.write() = SelectableTextStatus::default();
        };

        let on_mouse_up = move |_| {
            editable.process_event(EditableEvent::Release);
        };

        let on_key_down = move |e: Event<KeyboardEventData>| {
            editable.process_event(EditableEvent::KeyDown {
                key: &e.key,
                modifiers: e.modifiers,
            });
        };

        let on_key_up = move |e: Event<KeyboardEventData>| {
            editable.process_event(EditableEvent::KeyUp { key: &e.key });
        };

        let on_global_pointer_press = move |_: Event<PointerEventData>| {
            match *status.read() {
                SelectableTextStatus::Idle if focus.is_focused() => {
                    editable.process_event(EditableEvent::Release);
                }
                SelectableTextStatus::Hovering => {
                    editable.process_event(EditableEvent::Release);
                }
                _ => {}
            };

            if drag_origin.read().is_some() {
                drag_origin.set(None);
            } else if focus.is_focused() {
                focus.request_unfocus();
            }
        };

        paragraph()
            .layout(self.layout.clone())
            .accessibility(self.accessibility.clone())
            .text_style(self.text_style_data.clone())
            .max_lines(self.max_lines)
            .line_height(self.line_height)
            .a11y_id(focus.a11y_id())
            .a11y_focusable(true)
            .holder(holder.read().clone())
            .cursor_color(Color::BLACK)
            .highlights(highlights.map(|h| vec![h]))
            .on_mouse_up(on_mouse_up)
            .on_global_pointer_move(on_global_pointer_move)
            .on_global_pointer_down(on_global_pointer_down)
            .on_pointer_down(on_pointer_down)
            .on_pointer_enter(on_pointer_enter)
            .on_pointer_leave(on_pointer_leave)
            .on_global_pointer_press(on_global_pointer_press)
            .on_key_down(on_key_down)
            .on_key_up(on_key_up)
            .span(Span::new(editable.editor().read().to_string()))
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
