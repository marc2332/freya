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
    pub value: Cow<'static, str>,
}

impl SelectableText {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl Render for SelectableText {
    fn render(&self) -> impl IntoElement {
        let holder = use_state(ParagraphHolder::default);
        let mut editable = use_editable(
            || self.value.to_string(),
            move || EditableConfig::new().with_allow_changes(false),
            EditableMode::MultipleLinesSingleEditor,
        );
        let mut status = use_state(SelectableTextStatus::default);
        let focus = use_focus();
        let mut drag_origin = use_state(|| None);

        if self.value.as_ref() != editable.editor().read().rope() {
            editable.editor_mut().write().set(self.value.as_ref());
            editable.editor_mut().write().editor_history().clear();
        }

        let highlights = editable.editor().read().get_visible_selection(0);

        let on_pointer_down = move |e: Event<PointerEventData>| {
            e.stop_propagation();
            drag_origin.set(Some(e.global_location() - e.element_location()));
            editable.process_event(EditableEvent::Down {
                location: e.element_location(),
                editor_id: 0,
                holder: &holder.read(),
            });
            focus.request_focus();
        };

        let on_global_mouse_move = move |e: Event<MouseEventData>| {
            if focus.is_focused()
                && let Some(drag_origin) = drag_origin()
            {
                let mut element_location = e.element_location;
                element_location.x -= drag_origin.x;
                element_location.y -= drag_origin.y;
                editable.process_event(EditableEvent::Move {
                    location: e.element_location,
                    editor_id: 0,
                    holder: &holder.read(),
                });
            }
        };

        let on_global_mouse_down = move |_| {
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
                code: e.code,
                modifiers: e.modifiers,
            });
        };

        let on_key_up = move |e: Event<KeyboardEventData>| {
            editable.process_event(EditableEvent::KeyUp { code: e.code });
        };

        let on_global_mouse_up = move |_| {
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
            .holder(holder.read().clone())
            .a11y_focusable(true)
            .cursor_color(Color::BLACK)
            .a11y_id(focus.a11y_id())
            .highlights(highlights.map(|h| vec![h]))
            .on_mouse_up(on_mouse_up)
            .on_global_mouse_move(on_global_mouse_move)
            .on_global_mouse_down(on_global_mouse_down)
            .on_pointer_down(on_pointer_down)
            .on_pointer_enter(on_pointer_enter)
            .on_pointer_leave(on_pointer_leave)
            .on_global_mouse_up(on_global_mouse_up)
            .on_key_down(on_key_down)
            .on_key_up(on_key_up)
            .span(Span::new(editable.editor().read().to_string()))
    }
}
