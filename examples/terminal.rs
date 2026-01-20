use freya::prelude::*;
use freya_terminal::prelude::*;
use keyboard_types::Modifiers;
use portable_pty::CommandBuilder;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let handle = use_state(|| {
        let mut cmd = CommandBuilder::new("bash");
        cmd.env("TERM", "xterm-256color");
        cmd.env("COLORTERM", "truecolor");
        cmd.env("LANG", "en_GB.UTF-8");
        TerminalHandle::new(cmd).unwrap()
    });

    let focus = use_focus();

    rect().expanded().background((30, 30, 30)).child(
        rect()
            .expanded()
            .background((10, 10, 10))
            .padding(6.)
            .a11y_id(focus.a11y_id())
            .a11y_auto_focus(true)
            .on_mouse_down(move |_| focus.request_focus())
            .on_key_down(move |e: Event<KeyboardEventData>| {
                if e.modifiers.contains(Modifiers::CONTROL)
                    && matches!(&e.key, Key::Character(ch) if ch.len() == 1)
                {
                    if let Key::Character(ch) = &e.key {
                        let _ = handle.read().write(&[ch.as_bytes()[0] & 0x1f]);
                    }
                } else if let Some(ch) = e.try_as_str() {
                    let _ = handle.read().write(ch.as_bytes());
                } else {
                    let _ = handle.read().write(match &e.key {
                        Key::Named(NamedKey::Enter) => b"\r",
                        Key::Named(NamedKey::Backspace) => &[0x7f],
                        Key::Named(NamedKey::Delete) => b"\x1b[3~",
                        Key::Named(NamedKey::Tab) => b"\t",
                        Key::Named(NamedKey::Escape) => &[0x1b],
                        Key::Named(NamedKey::ArrowUp) => b"\x1b[A",
                        Key::Named(NamedKey::ArrowDown) => b"\x1b[B",
                        Key::Named(NamedKey::ArrowLeft) => b"\x1b[D",
                        Key::Named(NamedKey::ArrowRight) => b"\x1b[C",
                        _ => return,
                    });
                };
            })
            .child(Terminal::new(handle.read().clone())),
    )
}
