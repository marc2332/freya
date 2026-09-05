use freya::{
    prelude::*,
    terminal::*,
};

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

const PROMPT: &str = "> ";

#[derive(Default)]
struct CustomBackend {
    output: Option<TerminalOutput>,
    line: String,
}

impl CustomBackend {
    fn print(&self, text: impl Into<Vec<u8>>) {
        if let Some(output) = &self.output {
            let _ = output.write(text);
        }
    }

    fn run(&mut self, line: String) {
        match line.trim() {
            "" => {}
            "help" => self.print("Commands: help, clear, exit. Anything else is echoed back.\r\n"),
            "clear" => self.print("\x1b[2J\x1b[H"),
            "exit" => {
                self.output = None;
                return;
            }
            other => self.print(format!("{other}\r\n")),
        }
        self.print(PROMPT);
    }
}

impl TerminalBackend for CustomBackend {
    fn start(&mut self, output: TerminalOutput) -> Result<(), TerminalError> {
        self.output = Some(output);
        self.print("Custom terminal backend. Type \x1b[1mhelp\x1b[0m to list the commands.\r\n");
        self.print(PROMPT);
        Ok(())
    }

    fn write(&mut self, data: &[u8]) -> Result<(), TerminalError> {
        if data.first() == Some(&0x1b) {
            return Ok(());
        }
        for character in String::from_utf8_lossy(data).chars() {
            match character {
                '\r' => {
                    self.print("\r\n");
                    let line = std::mem::take(&mut self.line);
                    self.run(line);
                }
                '\u{7f}' => {
                    if self.line.pop().is_some() {
                        self.print("\x08 \x08");
                    }
                }
                '\u{3}' => {
                    self.line.clear();
                    self.print("^C\r\n");
                    self.print(PROMPT);
                }
                character if !character.is_control() => {
                    self.line.push(character);
                    self.print(character.to_string());
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn resize(&mut self, _rows: u16, _cols: u16) {}
}

fn app() -> impl IntoElement {
    let mut handle =
        use_state(|| TerminalHandle::new(TerminalId::new(), CustomBackend::default(), None).ok());

    use_future(move || async move {
        let terminal_handle = handle.read().clone();
        let Some(terminal_handle) = terminal_handle else {
            return;
        };
        terminal_handle.closed().await;
        let _ = handle.write().take();
    });

    let a11y_id = use_a11y();
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
                        .a11y_id(a11y_id)
                        .a11y_auto_focus(true)
                        .on_measured(move |(char_width, line_height)| {
                            dimensions.set((char_width, line_height));
                        })
                        .on_mouse_down(move |_| a11y_id.request_focus())
                        .on_wheel({
                            let handle = handle.clone();
                            move |e: Event<WheelEventData>| {
                                let (char_width, line_height) = dimensions();
                                let (mouse_x, mouse_y) = e.element_location.to_tuple();
                                let col = (mouse_x / char_width as f64) as f32;
                                let row = (mouse_y / line_height as f64) as f32;
                                handle.wheel(e.delta_y, row, col);
                            }
                        })
                        .on_key_down(move |e: Event<KeyboardEventData>| {
                            let _ = handle.write_key(&e.key, e.modifiers);
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
