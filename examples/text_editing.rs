use freya::{
    prelude::*,
    text_edit::*,
};

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> Element {
    let holder = use_state(ParagraphHolder::default);
    let mut editable = use_editable(
        || "Hello, World!".to_string(),
        EditableConfig::new,
        EditableMode::MultipleLinesSingleEditor,
    );
    let focus = use_focus();

    paragraph()
        .a11y_id(focus.a11y_id())
        .color((0, 0, 0))
        .cursor_index(editable.editor().read().cursor_pos())
        .highlights(
            editable
                .editor()
                .read()
                .get_selection()
                .map(|selection| vec![selection])
                .unwrap_or_default(),
        )
        .on_mouse_down(move |e: Event<MouseEventData>| {
            focus.request_focus();
            editable.process_event(EditableEvent::Down {
                location: e.element_location,
                editor_id: 0,
                holder: &holder.read(),
            });
        })
        .on_mouse_move(move |e: Event<MouseEventData>| {
            editable.process_event(EditableEvent::Move {
                location: e.element_location,
                editor_id: 0,
                holder: &holder.read(),
            });
        })
        .on_mouse_up(move |_| editable.process_event(EditableEvent::Release))
        .on_key_down(move |e: Event<KeyboardEventData>| {
            editable.process_event(EditableEvent::KeyDown {
                key: &e.key,
                code: e.code,
                modifiers: e.modifiers,
                holder: None,
            });
        })
        .on_key_up(move |e: Event<KeyboardEventData>| {
            editable.process_event(EditableEvent::KeyUp { code: e.code });
        })
        .span(editable.editor().read().to_string())
        .holder(holder.read().clone())
        .into()
}
