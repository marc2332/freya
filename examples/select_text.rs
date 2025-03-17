#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::rc::Rc;

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Selectable Text", (900.0, 650.0));
}

/// Enum to declare is [`Input`] hidden.
#[derive(Default, Clone, PartialEq)]
pub enum InputMode {
    /// The input text is shown
    #[default]
    Shown,
    /// The input text is obfuscated with a character
    Hidden(char),
}

#[component]
fn SelectableText(value: ReadOnlySignal<String>) -> Element {
    let platform = use_platform();
    let mut editable = use_editable(
        move || EditableConfig::new(value()).with_allow_changes(false),
        EditableMode::MultipleLinesSingleEditor,
    );
    let mut status = use_signal(InputStatus::default);
    let mut focus = use_focus();
    let mut drag_origin = use_signal(|| None);

    if &*value.read() != editable.editor().read().rope() {
        editable.editor_mut().write().set(&value.read());
        editable.editor_mut().write().editor_history().clear();
    }

    use_drop(move || {
        if *status.peek() == InputStatus::Hovering {
            platform.set_cursor(CursorIcon::default());
        }
    });

    let a11y_id = focus.attribute();
    let cursor_reference = editable.cursor_attr();
    let highlights = editable.highlights_attr(0);
    let editor = editable.editor().read();
    let cursor_char = editor.cursor_pos();

    let onmousedown = move |e: MouseEvent| {
        e.stop_propagation();
        drag_origin.set(Some(e.get_screen_coordinates() - e.element_coordinates));
        editable.process_event(&EditableEvent::MouseDown(e.data, 0));
        focus.focus();
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

    let onclick = move |_: MouseEvent| {
        editable.process_event(&EditableEvent::Click);
    };

    let onglobalkeydown = move |e: KeyboardEvent| {
        editable.process_event(&EditableEvent::KeyDown(e.data));
    };

    let onglobalkeyup = move |e: KeyboardEvent| {
        editable.process_event(&EditableEvent::KeyUp(e.data));
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

        // Unfocus text when this:
        // + is focused
        // + it has not just being dragged
        // + a global click happened
        if focus.is_focused() {
            if drag_origin.read().is_some() {
                drag_origin.set(None);
            } else {
                editable.editor_mut().write().clear_selection();
                focus.unfocus();
            }
        }
    };

    rsx!(
        paragraph {
            a11y_focusable: "true",
            a11y_id,
            cursor_id: "0",
            cursor_mode: "editable",
            cursor_color: "black",
            highlights,
            cursor_reference,
            onclick,
            onglobalmousemove,
            onmousedown,
            onmouseenter,
            onmouseleave,
            onglobalclick,
            onglobalkeydown,
            onglobalkeyup,
            text {
                "{editable.editor()}"
            }
        }
    )
}

fn app() -> Element {
    rsx!(
        rect {
            padding: "25",
            spacing: "10",
            label {
                font_size: "35",
                "Select the text from below"
            }
            rect {
                font_size: "25",
                color: "rgb(20, 20, 20)",
                SelectableText {
                    value: "heyyyy"
                }
            }
        }
    )
}
