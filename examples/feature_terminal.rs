use freya::{
    clipboard::Clipboard,
    prelude::*,
    terminal::*,
};
use futures_util::FutureExt;

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
        if let Some(terminal_handle) = handle.read().clone() {
            loop {
                futures_util::select! {
                    _ = terminal_handle.closed().fuse() => {
                        let _ = handle.write().take();
                        break;
                    }
                    _ = terminal_handle.title_changed().fuse() => {
                        if let Some(new_title) = terminal_handle.title() {
                            Platform::get().with_window(None, move |window| {
                                window.set_title(&new_title);
                            });
                        }
                    }
                }
            }
        }
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
                        .on_global_pointer_press({
                            let handle = handle.clone();
                            move |_: Event<PointerEventData>| {
                                handle.release();
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
                        })
                        .a11y_id(focus.a11y_id())
                        .a11y_role(AccessibilityRole::Terminal)
                        .a11y_auto_focus(true)
                        .on_key_up({
                            let handle = handle.clone();
                            move |e: Event<KeyboardEventData>| {
                                if e.key == Key::Named(NamedKey::Shift) {
                                    handle.shift_pressed(false);
                                }
                            }
                        })
                        .on_key_down(move |e: Event<KeyboardEventData>| {
                            let ctrl_shift =
                                e.modifiers.contains(Modifiers::CONTROL | Modifiers::SHIFT);

                            match &e.key {
                                Key::Character(ch)
                                    if ctrl_shift && ch.eq_ignore_ascii_case("c") =>
                                {
                                    if let Some(text) = handle.get_selected_text() {
                                        let _ = Clipboard::set(text);
                                    }
                                }
                                Key::Character(ch)
                                    if ctrl_shift && ch.eq_ignore_ascii_case("v") =>
                                {
                                    if let Ok(text) = Clipboard::get() {
                                        let _ = handle.paste(&text);
                                    }
                                }
                                _ => {
                                    let _ = handle.write_key(&e.key, e.modifiers);
                                }
                            }
                        }),
                )
                .expanded()
                .background((10, 10, 10))
                .padding(6.)
                .into_element()
        } else {
            "Terminal exited".into_element()
        })
}
