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
        cmd.env("LANG", "en_US.UTF-8");
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
                let to_write = if e.modifiers.contains(Modifiers::CONTROL)
                    && matches!(&e.key, Key::Character(ch) if ch.len() == 1)
                {
                    if let Key::Character(ch) = &e.key {
                        vec![ch.as_bytes()[0] & 0x1f]
                    } else {
                        return;
                    }
                } else if let Some(ch) = e.try_as_str() {
                    ch.as_bytes().to_vec()
                } else {
                    match &e.key {
                        Key::Named(NamedKey::Enter) => b"\r".to_vec(),
                        Key::Named(NamedKey::Backspace) => vec![0x7f],
                        Key::Named(NamedKey::Delete) => b"\x1b[3~".to_vec(),
                        Key::Named(NamedKey::Tab) => b"\t".to_vec(),
                        Key::Named(NamedKey::Escape) => vec![0x1b],
                        Key::Named(NamedKey::ArrowUp) => b"\x1b[A".to_vec(),
                        Key::Named(NamedKey::ArrowDown) => b"\x1b[B".to_vec(),
                        Key::Named(NamedKey::ArrowLeft) => b"\x1b[D".to_vec(),
                        Key::Named(NamedKey::ArrowRight) => b"\x1b[C".to_vec(),
                        _ => return,
                    }
                };

                let _ = handle.read().write(&to_write);
            })
            .child(Terminal::with_handle(handle.read().clone())),
    )
}
