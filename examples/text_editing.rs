#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::{
    prelude::*,
    text_edit::*,
};

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let holder = use_state(ParagraphHolder::default);
    let mut editable = use_editable(|| "Hello, World!".to_string(), EditableConfig::new);
    let focus = use_focus();

    paragraph()
        .a11y_id(focus.a11y_id())
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
                editor_line: EditorLine::SingleParagraph,
                holder: &holder.read(),
            });
        })
        .on_mouse_move(move |e: Event<MouseEventData>| {
            editable.process_event(EditableEvent::Move {
                location: e.element_location,
                editor_line: EditorLine::SingleParagraph,
                holder: &holder.read(),
            });
        })
        .on_global_mouse_up(move |_| editable.process_event(EditableEvent::Release))
        .on_key_down(move |e: Event<KeyboardEventData>| {
            editable.process_event(EditableEvent::KeyDown {
                key: &e.key,
                modifiers: e.modifiers,
            });
        })
        .on_key_up(move |e: Event<KeyboardEventData>| {
            editable.process_event(EditableEvent::KeyUp { key: &e.key });
        })
        .span(editable.editor().read().to_string())
        .holder(holder.read().clone())
}
