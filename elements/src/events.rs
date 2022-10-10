use glutin::event::{ModifiersState, VirtualKeyCode};

pub type KeyCode = VirtualKeyCode;

#[derive(Debug)]
pub struct KeyboardData {
    pub code: VirtualKeyCode,
    pub modifiers: ModifiersState,
}

impl KeyboardData {
    pub fn new(code: VirtualKeyCode, modifiers: ModifiersState) -> Self {
        Self { code, modifiers }
    }
}

impl KeyboardData {
    pub fn char_to_str(&self) -> Option<String> {
        let mut text = match &self.code {
            VirtualKeyCode::Key1 => Some("1".to_string()),
            VirtualKeyCode::Key2 => Some("2".to_string()),
            VirtualKeyCode::Key3 => Some("3".to_string()),
            VirtualKeyCode::Key4 => Some("4".to_string()),
            VirtualKeyCode::Key5 => Some("5".to_string()),
            VirtualKeyCode::Key6 => Some("6".to_string()),
            VirtualKeyCode::Key7 => Some("7".to_string()),
            VirtualKeyCode::Key8 => Some("8".to_string()),
            VirtualKeyCode::Key9 => Some("9".to_string()),
            VirtualKeyCode::Key0 => Some("0".to_string()),

            VirtualKeyCode::A => Some("a".to_string()),
            VirtualKeyCode::B => Some("b".to_string()),
            VirtualKeyCode::C => Some("c".to_string()),
            VirtualKeyCode::D => Some("d".to_string()),
            VirtualKeyCode::E => Some("e".to_string()),
            VirtualKeyCode::F => Some("f".to_string()),
            VirtualKeyCode::G => Some("g".to_string()),
            VirtualKeyCode::H => Some("h".to_string()),
            VirtualKeyCode::I => Some("i".to_string()),
            VirtualKeyCode::J => Some("j".to_string()),
            VirtualKeyCode::K => Some("k".to_string()),
            VirtualKeyCode::L => Some("l".to_string()),
            VirtualKeyCode::M => Some("m".to_string()),
            VirtualKeyCode::N => Some("n".to_string()),
            VirtualKeyCode::O => Some("o".to_string()),
            VirtualKeyCode::P => Some("p".to_string()),
            VirtualKeyCode::Q => Some("q".to_string()),
            VirtualKeyCode::R => Some("r".to_string()),
            VirtualKeyCode::S => Some("s".to_string()),
            VirtualKeyCode::T => Some("t".to_string()),
            VirtualKeyCode::U => Some("u".to_string()),
            VirtualKeyCode::V => Some("v".to_string()),
            VirtualKeyCode::W => Some("w".to_string()),
            VirtualKeyCode::X => Some("x".to_string()),
            VirtualKeyCode::Y => Some("y".to_string()),
            VirtualKeyCode::Z => Some("z".to_string()),

            VirtualKeyCode::Numpad0 => Some("1".to_string()),
            VirtualKeyCode::Numpad1 => Some("1".to_string()),
            VirtualKeyCode::Numpad2 => Some("2".to_string()),
            VirtualKeyCode::Numpad3 => Some("3".to_string()),
            VirtualKeyCode::Numpad4 => Some("4".to_string()),
            VirtualKeyCode::Numpad5 => Some("5".to_string()),
            VirtualKeyCode::Numpad6 => Some("6".to_string()),
            VirtualKeyCode::Numpad7 => Some("7".to_string()),
            VirtualKeyCode::Numpad8 => Some("8".to_string()),
            VirtualKeyCode::Numpad9 => Some("9".to_string()),

            _ => None,
        };

        if let Some(text_char) = &text {
            if self.modifiers == ModifiersState::SHIFT {
                text = Some(text_char.to_uppercase());
            }
        }

        text
    }
}
