pub use keyboard_types::{Code, Key, Modifiers};
use winit::keyboard::{KeyCode, ModifiersState, PhysicalKey};

// Map winit modifiers to keyboard_types modifiers
pub fn get_modifiers(modifiers: ModifiersState) -> Modifiers {
    let mut new_modifiers = Modifiers::empty();
    if modifiers.alt_key() {
        new_modifiers.toggle(Modifiers::ALT);
    }
    if modifiers.control_key() {
        new_modifiers.toggle(Modifiers::CONTROL);
    }
    if modifiers.shift_key() {
        new_modifiers.toggle(Modifiers::SHIFT);
    }
    if modifiers.super_key() {
        new_modifiers.toggle(Modifiers::META);
    }
    new_modifiers
}

/// Only return keys that aren't text
pub fn get_non_text_keys(key: &winit::keyboard::Key) -> Key {
    match key {
        winit::keyboard::Key::Character(c) => Key::Character(c.to_string()),
        _ => Key::Unidentified,
    }
}

/// Return the equivalent code of Winit and `keyboard_types`
pub fn from_winit_to_code(key: &PhysicalKey) -> Code {
    if let PhysicalKey::Code(key) = key {
        match key {
            KeyCode::Digit1 => Code::Digit1,
            KeyCode::Digit2 => Code::Digit2,
            KeyCode::Digit3 => Code::Digit3,
            KeyCode::Digit4 => Code::Digit4,
            KeyCode::Digit5 => Code::Digit5,
            KeyCode::Digit6 => Code::Digit6,
            KeyCode::Digit7 => Code::Digit7,
            KeyCode::Digit8 => Code::Digit8,
            KeyCode::Digit9 => Code::Digit9,
            KeyCode::Digit0 => Code::Digit0,
            KeyCode::KeyA => Code::KeyA,
            KeyCode::KeyB => Code::KeyB,
            KeyCode::KeyC => Code::KeyC,
            KeyCode::KeyD => Code::KeyD,
            KeyCode::KeyE => Code::KeyE,
            KeyCode::KeyF => Code::KeyF,
            KeyCode::KeyG => Code::KeyG,
            KeyCode::KeyH => Code::KeyH,
            KeyCode::KeyI => Code::KeyI,
            KeyCode::KeyJ => Code::KeyJ,
            KeyCode::KeyK => Code::KeyK,
            KeyCode::KeyL => Code::KeyL,
            KeyCode::KeyM => Code::KeyM,
            KeyCode::KeyN => Code::KeyN,
            KeyCode::KeyO => Code::KeyO,
            KeyCode::KeyP => Code::KeyP,
            KeyCode::KeyQ => Code::KeyQ,
            KeyCode::KeyR => Code::KeyR,
            KeyCode::KeyS => Code::KeyS,
            KeyCode::KeyT => Code::KeyT,
            KeyCode::KeyU => Code::KeyU,
            KeyCode::KeyV => Code::KeyV,
            KeyCode::KeyW => Code::KeyW,
            KeyCode::KeyX => Code::KeyX,
            KeyCode::KeyY => Code::KeyY,
            KeyCode::KeyZ => Code::KeyZ,
            KeyCode::Escape => Code::Escape,
            KeyCode::F1 => Code::F1,
            KeyCode::F2 => Code::F2,
            KeyCode::F3 => Code::F3,
            KeyCode::F4 => Code::F4,
            KeyCode::F5 => Code::F5,
            KeyCode::F6 => Code::F6,
            KeyCode::F7 => Code::F7,
            KeyCode::F8 => Code::F8,
            KeyCode::F9 => Code::F9,
            KeyCode::F10 => Code::F10,
            KeyCode::F11 => Code::F11,
            KeyCode::F12 => Code::F12,
            KeyCode::F13 => Code::F13,
            KeyCode::F14 => Code::F14,
            KeyCode::F15 => Code::F15,
            KeyCode::F16 => Code::F16,
            KeyCode::F17 => Code::F17,
            KeyCode::F18 => Code::F18,
            KeyCode::F19 => Code::F19,
            KeyCode::F20 => Code::F20,
            KeyCode::F21 => Code::F21,
            KeyCode::F22 => Code::F22,
            KeyCode::F23 => Code::F23,
            KeyCode::F24 => Code::F24,
            KeyCode::Pause => Code::Pause,
            KeyCode::Insert => Code::Insert,
            KeyCode::Home => Code::Home,
            KeyCode::Delete => Code::Delete,
            KeyCode::End => Code::End,
            KeyCode::PageDown => Code::PageDown,
            KeyCode::PageUp => Code::PageUp,
            KeyCode::ArrowLeft => Code::ArrowLeft,
            KeyCode::ArrowUp => Code::ArrowUp,
            KeyCode::ArrowRight => Code::ArrowRight,
            KeyCode::ArrowDown => Code::ArrowDown,
            KeyCode::Backspace => Code::Backspace,
            KeyCode::Enter => Code::Enter,
            KeyCode::Space => Code::Space,
            KeyCode::NumLock => Code::NumLock,
            KeyCode::Numpad0 => Code::Numpad0,
            KeyCode::Numpad1 => Code::Numpad1,
            KeyCode::Numpad2 => Code::Numpad2,
            KeyCode::Numpad3 => Code::Numpad3,
            KeyCode::Numpad4 => Code::Numpad4,
            KeyCode::Numpad5 => Code::Numpad5,
            KeyCode::Numpad6 => Code::Numpad6,
            KeyCode::Numpad7 => Code::Numpad7,
            KeyCode::Numpad8 => Code::Numpad8,
            KeyCode::Numpad9 => Code::Numpad9,
            KeyCode::NumpadAdd => Code::NumpadAdd,
            KeyCode::NumpadDivide => Code::NumpadDivide,
            KeyCode::NumpadDecimal => Code::NumpadDecimal,
            KeyCode::NumpadComma => Code::NumpadComma,
            KeyCode::NumpadEnter => Code::NumpadEnter,
            KeyCode::NumpadEqual => Code::NumpadEqual,
            KeyCode::NumpadMultiply => Code::NumpadMultiply,
            KeyCode::NumpadSubtract => Code::NumpadSubtract,
            KeyCode::Backslash => Code::Backslash,
            KeyCode::Comma => Code::Comma,
            KeyCode::Convert => Code::Convert,
            KeyCode::Equal => Code::Equal,
            KeyCode::BracketLeft => Code::BracketLeft,
            KeyCode::BracketRight => Code::BracketRight,
            KeyCode::ShiftLeft => Code::ShiftLeft,
            KeyCode::Meta => Code::MetaLeft,
            KeyCode::MediaSelect => Code::Unidentified,
            KeyCode::MediaStop => Code::Unidentified,
            KeyCode::Minus => Code::Unidentified,
            KeyCode::Period => Code::Unidentified,
            KeyCode::Power => Code::Unidentified,
            KeyCode::AltRight => Code::AltRight,
            KeyCode::ControlLeft => Code::ControlLeft,
            KeyCode::ControlRight => Code::ControlRight,
            KeyCode::ShiftRight => Code::ShiftRight,
            KeyCode::Semicolon => Code::Semicolon,
            KeyCode::Slash => Code::Unidentified,
            KeyCode::Sleep => Code::Unidentified,
            KeyCode::Tab => Code::Tab,
            KeyCode::AudioVolumeUp => Code::AudioVolumeUp,
            KeyCode::IntlYen => Code::IntlYen,
            KeyCode::Copy => Code::Copy,
            KeyCode::Paste => Code::Paste,
            KeyCode::Cut => Code::Cut,
            _ => Code::Unidentified,
        }
    } else {
        Code::Unidentified
    }
}

/// Data of a Keyboard event.
#[derive(Debug, Clone, PartialEq)]
pub struct KeyboardData {
    pub key: Key,
    pub code: Code,
    pub modifiers: Modifiers,
}

impl KeyboardData {
    pub fn new(key: Key, code: Code, modifiers: Modifiers) -> Self {
        Self {
            key,
            code,
            modifiers,
        }
    }
}

impl KeyboardData {
    /// Try to get the text of the character
    pub fn to_text(&self) -> Option<&str> {
        if let Key::Character(c) = &self.key {
            Some(c)
        } else {
            None
        }
    }
}
