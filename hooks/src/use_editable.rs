use dioxus::{core::UiEvent, prelude::*};
use freya_elements::events::{KeyCode, KeyboardData};
use xi_rope::Rope;

pub fn use_editable(
    cx: &ScopeState,
) -> (
    &UseState<Rope>,
    &UseState<(usize, usize)>,
    impl Fn(UiEvent<KeyboardData>) + '_,
) {
    let content = use_state(cx, || {
        Rope::from("Lorem ipsum dolor sit amet,\nconsectetur adipiscing elit,\nsed do eiusmod tempor incididunt\nut labore et dolore magna aliqua.\nUt enim ad minim veniam,\nquis nostrud exercitation\nullamco laboris nisi ut \naliquip ex ea commodo consequat.\n\n\n\n\n\n\n\n\n\n")
    });
    let cursor = use_state(cx, || (0, 0));

    let process_keyevent = move |e: UiEvent<KeyboardData>| match &e.code {
        KeyCode::ArrowDown => {
            let total_lines = content.lines(..).count() - 1;
            if cursor.1 < total_lines {
                let next_line = content.get().lines(..).nth(cursor.1 + 1).unwrap();
                let cursor_indexolumn = if cursor.0 <= next_line.len() {
                    cursor.0
                } else {
                    next_line.len()
                };
                cursor.set((cursor_indexolumn, cursor.1 + 1))
            }
        }
        KeyCode::ArrowLeft => {
            if cursor.0 > 0 {
                cursor.set((cursor.0 - 1, cursor.1));
            } else {
                let prev_line = content.get().lines(..).nth(cursor.1 - 1);
                if let Some(prev_line) = prev_line {
                    if cursor.0 == 0 && cursor.1 > 0 {
                        let len = if prev_line.len() > 0 {
                            prev_line.len() - 1
                        } else {
                            0
                        };
                        cursor.set((len, cursor.1 - 1));
                    } else if cursor.0 > 0 {
                        cursor.set((cursor.0 - 1, cursor.1));
                    }
                }
            }
        }
        KeyCode::ArrowRight => {
            let total_lines = content.lines(..).count() - 1;
            let current_line = content.get().lines(..).nth(cursor.1).unwrap();
            if cursor.0 < current_line.len() {
                cursor.set((cursor.0 + 1, cursor.1));
            } else if cursor.0 == current_line.len() && cursor.1 < total_lines {
                cursor.set((0, cursor.1 + 1));
            }
        }
        KeyCode::ArrowUp => {
            if cursor.1 > 0 {
                let prev_line = content.get().lines(..).nth(cursor.1 - 1).unwrap();
                let cursor_indexolumn = if cursor.0 <= prev_line.len() {
                    cursor.0
                } else {
                    prev_line.len()
                };
                cursor.set((cursor_indexolumn, cursor.1 - 1))
            }
        }
        KeyCode::Space => {
            let char_idx = content.get().offset_of_line(cursor.1) + cursor.0;
            content.with_mut(|code| {
                code.edit(char_idx..char_idx, " ");
            });
            cursor.set((cursor.0 + 1, cursor.1))
        }
        KeyCode::Backspace => {
            if cursor.0 > 0 {
                let char_idx = content.get().offset_of_line(cursor.1) + cursor.0;
                content.with_mut(|code| {
                    code.edit(char_idx - 1..char_idx, "");
                });

                cursor.set((cursor.0 - 1, cursor.1))
            } else if cursor.1 > 0 {
                let prev_line = content.get().lines(..).nth(cursor.1 - 1).unwrap();
                let current_line = content.get().lines(..).nth(cursor.1);

                if let Some(current_line) = current_line {
                    let prev_char_idx =
                        content.get().offset_of_line(cursor.1 - 1) + prev_line.len();
                    let char_idx = content.get().offset_of_line(cursor.1) + current_line.len();

                    content.with_mut(|code| {
                        code.edit(prev_char_idx..prev_char_idx, current_line.clone());
                        code.edit(char_idx..char_idx + current_line.len(), "");
                    });
                }

                cursor.set((prev_line.len(), cursor.1 - 1));
            }
        }
        KeyCode::Enter => {
            let total_lines = content.lines(..).count();
            let char_idx = content.get().offset_of_line(cursor.1) + cursor.0;
            let current_line = content.get().lines(..).nth(cursor.1).unwrap();
            content.with_mut(|code| {
                let break_line = if cursor.1 == total_lines - 1 && current_line.len() > 0 {
                    "\n\n"
                } else {
                    "\n"
                };
                code.edit(char_idx..char_idx, break_line);
            });

            cursor.set((0, cursor.1 + 1))
        }
        character => {
            if let Some(character) = character.to_text() {
                let char_idx = content.get().offset_of_line(cursor.1) + cursor.0;

                content.with_mut(|code| {
                    code.edit(char_idx..char_idx, character);
                });

                cursor.set((cursor.0 + 1, cursor.1))
            }
        }
    };

    (content, cursor, process_keyevent)
}
