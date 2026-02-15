use freya::{
    clipboard::Clipboard,
    prelude::*,
    terminal::*,
};

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let mut handle = use_state(|| {
        let mut cmd = CommandBuilder::new("bash");
        cmd.env("TERM", "xterm-256color");
        cmd.env("COLORTERM", "truecolor");
        cmd.env("LANG", "en_GB.UTF-8");
        TerminalHandle::new(TerminalId::new(), cmd, None).ok()
    });

    use_future(move || async move {
        // Stops rendering the terminal once the pty closes on its own
        let terminal_handle = handle.read().clone().unwrap();
        terminal_handle.closed().await;
        let _ = handle.write().take();
    });

    let focus = use_focus();
    let mut dimensions = use_state(|| (0.0, 0.0));

    rect()
        .expanded()
        .center()
        .background((30, 30, 30))
        .color((245, 245, 245))
        .child(if let Some(handle) = handle.read().clone() {
            rect()
                .child(
                    Terminal::new(handle.clone())
                        .on_measured(move |(char_width, line_height)| {
                            dimensions.set((char_width, line_height));
                        })
                        .on_mouse_down({
                            let handle = handle.clone();
                            move |e: Event<MouseEventData>| {
                                focus.request_focus();
                                let (char_width, line_height) = dimensions();
                                let col =
                                    (e.element_location.x / char_width as f64).floor() as usize;
                                let row =
                                    (e.element_location.y / line_height as f64).floor() as usize;
                                let button = match e.button {
                                    Some(MouseButton::Middle) => TerminalMouseButton::Middle,
                                    Some(MouseButton::Right) => TerminalMouseButton::Right,
                                    _ => TerminalMouseButton::Left,
                                };
                                handle.mouse_down(row, col, button);
                            }
                        })
                        .on_mouse_move({
                            let handle = handle.clone();
                            move |e: Event<MouseEventData>| {
                                let (char_width, line_height) = dimensions();
                                let col =
                                    (e.element_location.x / char_width as f64).floor() as usize;
                                let row =
                                    (e.element_location.y / line_height as f64).floor() as usize;
                                handle.update_selection(row, col);
                                handle.mouse_move(row, col);
                            }
                        })
                        .on_mouse_up({
                            let handle = handle.clone();
                            move |e: Event<MouseEventData>| {
                                let (char_width, line_height) = dimensions();
                                let col =
                                    (e.element_location.x / char_width as f64).floor() as usize;
                                let row =
                                    (e.element_location.y / line_height as f64).floor() as usize;
                                let button = match e.button {
                                    Some(MouseButton::Middle) => TerminalMouseButton::Middle,
                                    Some(MouseButton::Right) => TerminalMouseButton::Right,
                                    _ => TerminalMouseButton::Left,
                                };
                                handle.mouse_up(row, col, button);
                            }
                        })
                        .on_wheel({
                            let handle = handle.clone();
                            move |e: Event<WheelEventData>| {
                                let (char_width, line_height) = dimensions();
                                let (mouse_x, mouse_y) = e.element_location.to_tuple();
                                let col = (mouse_x / char_width as f64).floor() as usize;
                                let row = (mouse_y / line_height as f64).floor() as usize;
                                handle.wheel(e.delta_y, row, col);
                            }
                        }),
                )
                .expanded()
                .background((10, 10, 10))
                .padding(6.)
                .a11y_id(focus.a11y_id())
                .a11y_auto_focus(true)
                .on_key_down(move |e: Event<KeyboardEventData>| {
                    let mods = e.modifiers;
                    let ctrl_shift = mods.contains(Modifiers::CONTROL | Modifiers::SHIFT);
                    let ctrl = mods.contains(Modifiers::CONTROL);

                    match &e.key {
                        Key::Character(ch) if ctrl_shift && ch.eq_ignore_ascii_case("c") => {
                            if let Some(text) = handle.get_selected_text() {
                                let _ = Clipboard::set(text);
                            }
                        }
                        Key::Character(ch) if ctrl_shift && ch.eq_ignore_ascii_case("v") => {
                            if let Ok(text) = Clipboard::get() {
                                let _ = handle.write(text.as_bytes());
                            }
                        }
                        Key::Character(ch) if ctrl && ch.len() == 1 => {
                            let _ = handle.write(&[ch.as_bytes()[0] & 0x1f]);
                        }
                        Key::Named(NamedKey::Enter) => {
                            let _ = handle.write(b"\r");
                        }
                        Key::Named(NamedKey::Backspace) => {
                            let _ = handle.write(&[0x7f]);
                        }
                        Key::Named(NamedKey::Delete) => {
                            let _ = handle.write(b"\x1b[3~");
                        }
                        Key::Named(NamedKey::Tab) => {
                            let _ = handle.write(b"\t");
                        }
                        Key::Named(NamedKey::Escape) => {
                            let _ = handle.write(&[0x1b]);
                        }
                        Key::Named(NamedKey::ArrowUp) => {
                            let _ = handle.write(b"\x1b[A");
                        }
                        Key::Named(NamedKey::ArrowDown) => {
                            let _ = handle.write(b"\x1b[B");
                        }
                        Key::Named(NamedKey::ArrowLeft) => {
                            let _ = handle.write(b"\x1b[D");
                        }
                        Key::Named(NamedKey::ArrowRight) => {
                            let _ = handle.write(b"\x1b[C");
                        }
                        _ => {
                            if let Some(ch) = e.try_as_str() {
                                let _ = handle.write(ch.as_bytes());
                            }
                        }
                    }
                })
                .into_element()
        } else {
            "Terminal exited".into_element()
        })
}
