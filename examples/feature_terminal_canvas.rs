#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::{
    clipboard::Clipboard,
    icons,
    prelude::*,
    radio::*,
    terminal::*,
};
use portable_pty::CommandBuilder;

#[derive(Default, Clone)]
struct AppState {
    panels: Vec<PanelData>,
}

#[derive(PartialEq, Clone)]
struct PanelData {
    id: usize,
    x: f64,
    y: f64,
}

#[derive(PartialEq, Eq, Clone, Debug, Copy, Hash)]
enum AppChannel {
    Panels,
}

impl RadioChannel<AppState> for AppChannel {}

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    use_init_root_theme(|| DARK_THEME);
    use_init_radio_station::<AppState, AppChannel>(AppState::default);
    let mut radio = use_radio::<AppState, AppChannel>(AppChannel::Panels);

    let on_press = move |_| {
        let offset = radio.read().panels.len() as f64 * 50.0;
        radio.write().panels.push(PanelData {
            id: UseId::<PanelData>::get_in_hook(),
            x: 100.0 + offset,
            y: 100.0 + offset,
        });
    };

    rect()
        .expanded()
        .background((20, 20, 20))
        .child(
            DraggableCanvas::new()
                .expanded()
                .children(radio.read().panels.iter().map(|panel| {
                    TerminalPanel {
                        data: panel.clone(),
                    }
                    .into()
                })),
        )
        .child(
            rect()
                .position(Position::new_absolute().top(20.0).left(20.0))
                .child(
                    Button::new()
                        .on_press(on_press)
                        .width(Size::px(48.))
                        .height(Size::px(48.))
                        .corner_radius(24.)
                        .background((40, 40, 40))
                        .focusable(false)
                        .child(
                            svg(icons::lucide::plus())
                                .width(Size::px(24.))
                                .height(Size::px(24.))
                                .color((200, 200, 200)),
                        ),
                ),
        )
}

#[derive(PartialEq, Clone)]
struct TerminalPanel {
    data: PanelData,
}

impl Component for TerminalPanel {
    fn render(&self) -> impl IntoElement {
        let panel_id = self.data.id;
        let mut radio = use_radio::<AppState, AppChannel>(AppChannel::Panels);

        let on_close = move |_| {
            radio.write().panels.retain(|p| p.id != panel_id);
        };

        let initial_position = CursorPoint::new(self.data.x, self.data.y);

        let handle = use_state(move || {
            let mut cmd = CommandBuilder::new("bash");
            cmd.env("TERM", "xterm-256color");
            cmd.env("COLORTERM", "truecolor");
            cmd.env("LANG", "en_GB.UTF-8");
            TerminalHandle::new(TerminalId::new(), cmd, None).ok()
        });

        use_future(move || async move {
            if let Some(ref terminal_handle) = *handle.read() {
                terminal_handle.closed().await;
            }
        });

        let focus = use_focus();
        let focus_status = use_focus_status(focus);
        let mut dimensions = use_state(|| (0.0, 0.0));

        let background = if focus_status.read().is_focused() {
            (25, 25, 25)
        } else {
            (10, 10, 10)
        };

        ResizableDraggable::new((250., 300.))
            .initial_position(initial_position)
            .child(
                rect()
                    .expanded()
                    .background((30, 30, 30))
                    .corner_radius(8.)
                    .border(
                        Border::new()
                            .fill((60, 60, 60))
                            .width(1.0)
                            .alignment(BorderAlignment::Inner),
                    )
                    .child(
                        rect()
                            .horizontal()
                            .padding(10.)
                            .main_align(Alignment::End)
                            .child(
                                Button::new()
                                    .on_press(on_close)
                                    .padding(4.)
                                    .rounded_full()
                                    .child(
                                        svg(icons::lucide::x())
                                            .width(Size::px(16.))
                                            .height(Size::px(16.))
                                            .color((200, 200, 200)),
                                    ),
                            ),
                    )
                    .child(if let Some(handle) = handle.read().clone() {
                        rect()
                            .expanded()
                            .background(background)
                            .padding(6.)
                            .a11y_id(focus.a11y_id())
                            .a11y_auto_focus(true)
                            .a11y_focusable(true)
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
                                            let col = (e.element_location.x / char_width as f64)
                                                .floor()
                                                as usize;
                                            let row = (e.element_location.y / line_height as f64)
                                                .floor()
                                                as usize;
                                            let button = match e.button {
                                                Some(MouseButton::Middle) => {
                                                    TerminalMouseButton::Middle
                                                }
                                                Some(MouseButton::Right) => {
                                                    TerminalMouseButton::Right
                                                }
                                                _ => TerminalMouseButton::Left,
                                            };
                                            handle.mouse_down(row, col, button);
                                            e.stop_propagation();
                                            e.prevent_default();
                                        }
                                    })
                                    .on_mouse_move({
                                        let handle = handle.clone();
                                        move |e: Event<MouseEventData>| {
                                            let (char_width, line_height) = dimensions();
                                            let col = (e.element_location.x / char_width as f64)
                                                .floor()
                                                as usize;
                                            let row = (e.element_location.y / line_height as f64)
                                                .floor()
                                                as usize;
                                            handle.mouse_move(row, col);
                                        }
                                    })
                                    .on_mouse_up({
                                        let handle = handle.clone();
                                        move |e: Event<MouseEventData>| {
                                            let (char_width, line_height) = dimensions();
                                            let col = (e.element_location.x / char_width as f64)
                                                .floor()
                                                as usize;
                                            let row = (e.element_location.y / line_height as f64)
                                                .floor()
                                                as usize;
                                            let button = match e.button {
                                                Some(MouseButton::Middle) => {
                                                    TerminalMouseButton::Middle
                                                }
                                                Some(MouseButton::Right) => {
                                                    TerminalMouseButton::Right
                                                }
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
                                            let col =
                                                (mouse_x / char_width as f64).floor() as usize;
                                            let row =
                                                (mouse_y / line_height as f64).floor() as usize;
                                            handle.wheel(e.delta_y, row, col);
                                        }
                                    }),
                            )
                            .on_key_up({
                                let handle = handle.clone();
                                move |e: Event<KeyboardEventData>| {
                                    if e.key == Key::Named(NamedKey::Shift) {
                                        handle.shift_pressed(false);
                                    }
                                }
                            })
                            .on_key_down(move |e: Event<KeyboardEventData>| {
                                let mods = e.modifiers;
                                let ctrl_shift =
                                    mods.contains(Modifiers::CONTROL | Modifiers::SHIFT);
                                let ctrl = mods.contains(Modifiers::CONTROL);

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
                                    Key::Named(NamedKey::Shift) => {
                                        handle.shift_pressed(true);
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
                    }),
            )
    }

    fn render_key(&self) -> DiffKey {
        DiffKey::from(&self.data.id)
    }
}
