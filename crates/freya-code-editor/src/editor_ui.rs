use freya_components::scrollviews::{
    ScrollController,
    ScrollEvent,
    VirtualScrollView,
};
use freya_core::prelude::*;
use freya_edit::EditableEvent;

use crate::{
    editor_data::CodeEditorData,
    editor_line::EditorLineUI,
};

#[derive(PartialEq, Clone)]
pub struct CodeEditor {
    editor: Writable<CodeEditorData>,
    font_size: f32,
    line_height: f32,
    a11y_id: AccessibilityId,
}

impl CodeEditor {
    /// Creates a new editor UI component with the given writable data.
    ///
    /// Default values are applied for font size and line height.
    pub fn new(editor: impl Into<Writable<CodeEditorData>>, a11y_id: AccessibilityId) -> Self {
        Self {
            editor: editor.into(),
            font_size: 14.0,
            line_height: 1.4,
            a11y_id,
        }
    }

    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    /// Sets the line height multiplier (relative to font size).
    pub fn line_height(mut self, height: f32) -> Self {
        self.line_height = height;
        self
    }
}

impl Component for CodeEditor {
    fn render(&self) -> impl IntoElement {
        let CodeEditor {
            editor,
            font_size,
            line_height,
            a11y_id,
        } = self.clone();
        let editor_tab = editor.read();

        let editor_data = &editor_tab;

        let focus = Focus::new_for_id(a11y_id);

        let mut pressing_shift = use_state(|| false);
        let mut pressing_alt = use_state(|| false);

        let scroll_controller = use_hook(|| {
            let notifier = State::create(());
            let requests = State::create(vec![]);
            ScrollController::managed(
                notifier,
                requests,
                State::create(Callback::new({
                    let mut editor = editor.clone();
                    move |ev| {
                        editor.write_if(|mut editor| {
                            let current = editor.scrolls;
                            match ev {
                                ScrollEvent::X(x) => {
                                    editor.scrolls.0 = x;
                                }
                                ScrollEvent::Y(y) => {
                                    editor.scrolls.1 = y;
                                }
                            }
                            current != editor.scrolls
                        })
                    }
                })),
                State::create(Callback::new({
                    let editor = editor.clone();
                    move |_| {
                        let editor = editor.read();
                        editor.scrolls
                    }
                })),
            )
        });

        let line_height = (font_size * line_height).floor();
        let lines_len = editor_data.metrics.syntax_blocks.len();

        let on_mouse_down = move |_| {
            focus.request_focus();
        };

        let on_key_up = {
            let mut editor = editor.clone();
            move |e: Event<KeyboardEventData>| {
                match &e.key {
                    Key::Named(NamedKey::Shift) => {
                        pressing_shift.set(false);
                    }
                    Key::Named(NamedKey::Alt) => {
                        pressing_alt.set(false);
                    }
                    _ => {}
                };

                editor.write_if(|mut editor| {
                    editor.process(font_size, EditableEvent::KeyUp { key: &e.key })
                });
            }
        };

        let on_key_down = {
            let mut editor = editor.clone();
            move |e: Event<KeyboardEventData>| {
                e.stop_propagation();

                match &e.key {
                    Key::Named(NamedKey::Shift) => {
                        pressing_shift.set(true);
                    }
                    Key::Named(NamedKey::Alt) => {
                        pressing_alt.set(true);
                    }
                    Key::Named(NamedKey::Tab) => {
                        e.prevent_default();
                    }
                    _ => {}
                };

                const LINES_JUMP_ALT: usize = 5;
                const LINES_JUMP_CONTROL: usize = 3;

                editor.write_if(|mut editor| {
                    let lines_jump = (line_height * LINES_JUMP_ALT as f32).ceil() as i32;
                    let min_height = -(lines_len as f32 * line_height) as i32;
                    let max_height = 0; // TODO, this should be the height of the viewport
                    let current_scroll = editor.scrolls.1;

                    let events = match &e.key {
                        Key::Named(NamedKey::ArrowUp) if e.modifiers.contains(Modifiers::ALT) => {
                            let jump = (current_scroll + lines_jump).clamp(min_height, max_height);
                            editor.scrolls.1 = jump;
                            (0..LINES_JUMP_ALT)
                                .map(|_| EditableEvent::KeyDown {
                                    key: &e.key,
                                    modifiers: e.modifiers,
                                })
                                .collect::<Vec<EditableEvent>>()
                        }
                        Key::Named(NamedKey::ArrowDown) if e.modifiers.contains(Modifiers::ALT) => {
                            let jump = (current_scroll - lines_jump).clamp(min_height, max_height);
                            editor.scrolls.1 = jump;
                            (0..LINES_JUMP_ALT)
                                .map(|_| EditableEvent::KeyDown {
                                    key: &e.key,
                                    modifiers: e.modifiers,
                                })
                                .collect::<Vec<EditableEvent>>()
                        }
                        Key::Named(NamedKey::ArrowDown) | Key::Named(NamedKey::ArrowUp)
                            if e.modifiers.contains(Modifiers::CONTROL) =>
                        {
                            (0..LINES_JUMP_CONTROL)
                                .map(|_| EditableEvent::KeyDown {
                                    key: &e.key,
                                    modifiers: e.modifiers,
                                })
                                .collect::<Vec<EditableEvent>>()
                        }
                        _ if e.code == Code::Escape
                            || e.modifiers.contains(Modifiers::ALT)
                            || (e.modifiers.contains(Modifiers::CONTROL)
                                && e.code == Code::KeyS) =>
                        {
                            Vec::new()
                        }
                        _ => {
                            vec![EditableEvent::KeyDown {
                                key: &e.key,
                                modifiers: e.modifiers,
                            }]
                        }
                    };

                    let mut changed = false;

                    for event in events {
                        changed |= editor.process(font_size, event);
                    }

                    changed
                });
            }
        };

        rect().expanded().background((29, 32, 33)).child(
            rect()
                .a11y_auto_focus(true)
                .a11y_focusable(true)
                .a11y_id(focus.a11y_id())
                .on_key_down(on_key_down)
                .on_key_up(on_key_up)
                .on_mouse_down(on_mouse_down)
                .child(
                    VirtualScrollView::new(move |line_index, _| {
                        EditorLineUI {
                            editor: editor.clone(),
                            font_size,
                            line_height,
                            line_index,
                        }
                        .into()
                    })
                    .scroll_controller(scroll_controller)
                    .length(lines_len as i32)
                    .item_size(line_height),
                ),
        )
    }
}
