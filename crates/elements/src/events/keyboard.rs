pub use keyboard_types::{
    Code,
    Key,
    Modifiers,
};
use winit::keyboard::{
    self,
    NamedKey,
};

use crate::definitions::PlatformEventData;

// Return the equivalent of Winit `ModifiersState` in keyboard_types
pub fn map_winit_modifiers(modifiers: keyboard::ModifiersState) -> Modifiers {
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

/// Return the equivalent Winit's `Key` in keyboard_types
pub fn map_winit_key(key: &keyboard::Key) -> Key {
    match key {
        keyboard::Key::Character(c) => Key::Character(c.to_string()),
        keyboard::Key::Named(named_key) => match named_key {
            NamedKey::Space => Key::Character(" ".to_string()),
            NamedKey::Delete => Key::Delete,
            NamedKey::Backspace => Key::Backspace,
            NamedKey::ArrowDown => Key::ArrowDown,
            NamedKey::ArrowLeft => Key::ArrowLeft,
            NamedKey::ArrowRight => Key::ArrowRight,
            NamedKey::ArrowUp => Key::ArrowUp,
            NamedKey::End => Key::End,
            NamedKey::Home => Key::Home,
            NamedKey::PageDown => Key::PageDown,
            NamedKey::PageUp => Key::PageUp,
            NamedKey::Tab => Key::Tab,
            NamedKey::Enter => Key::Enter,
            NamedKey::Escape => Key::Escape,
            NamedKey::F1 => Key::F1,
            NamedKey::F2 => Key::F2,
            NamedKey::F3 => Key::F3,
            NamedKey::F4 => Key::F4,
            NamedKey::F5 => Key::F5,
            NamedKey::F6 => Key::F6,
            NamedKey::F7 => Key::F7,
            NamedKey::F8 => Key::F8,
            NamedKey::F9 => Key::F9,
            NamedKey::F10 => Key::F10,
            NamedKey::F11 => Key::F11,
            NamedKey::F12 => Key::F12,
            NamedKey::F13 => Key::F13,
            NamedKey::F14 => Key::F14,
            NamedKey::F15 => Key::F15,
            NamedKey::F16 => Key::F16,
            NamedKey::F17 => Key::F17,
            NamedKey::F18 => Key::F18,
            NamedKey::F19 => Key::F19,
            NamedKey::F20 => Key::F20,
            NamedKey::F21 => Key::F21,
            NamedKey::F22 => Key::F22,
            NamedKey::F23 => Key::F23,
            NamedKey::F24 => Key::F24,
            NamedKey::Pause => Key::Pause,
            NamedKey::Insert => Key::Insert,
            NamedKey::ContextMenu => Key::ContextMenu,
            NamedKey::BrowserBack => Key::BrowserBack,
            NamedKey::BrowserFavorites => Key::BrowserFavorites,
            NamedKey::BrowserForward => Key::BrowserForward,
            NamedKey::BrowserHome => Key::BrowserHome,
            NamedKey::BrowserRefresh => Key::BrowserRefresh,
            NamedKey::BrowserSearch => Key::BrowserSearch,
            NamedKey::BrowserStop => Key::BrowserStop,
            NamedKey::MediaTrackNext => Key::MediaTrackNext,
            NamedKey::MediaPlayPause => Key::MediaPlayPause,
            NamedKey::MediaTrackPrevious => Key::MediaTrackPrevious,
            NamedKey::MediaStop => Key::MediaStop,
            NamedKey::AudioVolumeDown => Key::AudioVolumeDown,
            NamedKey::AudioVolumeMute => Key::AudioVolumeMute,
            NamedKey::AudioVolumeUp => Key::AudioVolumeUp,
            NamedKey::LaunchApplication2 => Key::LaunchApplication2,
            NamedKey::LaunchMail => Key::LaunchMail,
            NamedKey::Convert => Key::Convert,
            NamedKey::Alt => Key::Alt,
            NamedKey::AltGraph => Key::AltGraph,
            NamedKey::CapsLock => Key::CapsLock,
            NamedKey::Control => Key::Control,
            NamedKey::Fn => Key::Fn,
            NamedKey::FnLock => Key::FnLock,
            NamedKey::NumLock => Key::NumLock,
            NamedKey::ScrollLock => Key::ScrollLock,
            NamedKey::Shift => Key::Shift,
            NamedKey::Symbol => Key::Symbol,
            NamedKey::SymbolLock => Key::SymbolLock,
            NamedKey::Meta => Key::Meta,
            NamedKey::Hyper => Key::Hyper,
            NamedKey::Super => Key::Super,
            NamedKey::Clear => Key::Clear,
            NamedKey::Copy => Key::Copy,
            NamedKey::CrSel => Key::CrSel,
            NamedKey::Cut => Key::Cut,
            NamedKey::EraseEof => Key::EraseEof,
            NamedKey::ExSel => Key::ExSel,
            NamedKey::Paste => Key::Paste,
            NamedKey::Redo => Key::Redo,
            NamedKey::Undo => Key::Undo,
            NamedKey::Accept => Key::Accept,
            NamedKey::Again => Key::Again,
            NamedKey::Attn => Key::Attn,
            NamedKey::Cancel => Key::Cancel,
            NamedKey::Execute => Key::Execute,
            NamedKey::Find => Key::Find,
            NamedKey::Help => Key::Help,
            NamedKey::Play => Key::Play,
            NamedKey::Props => Key::Props,
            NamedKey::Select => Key::Select,
            NamedKey::ZoomIn => Key::ZoomIn,
            NamedKey::ZoomOut => Key::ZoomOut,
            NamedKey::BrightnessDown => Key::BrightnessDown,
            NamedKey::BrightnessUp => Key::BrightnessUp,
            NamedKey::Eject => Key::Eject,
            NamedKey::LogOff => Key::LogOff,
            NamedKey::Power => Key::Power,
            NamedKey::PowerOff => Key::PowerOff,
            NamedKey::PrintScreen => Key::PrintScreen,
            NamedKey::Hibernate => Key::Hibernate,
            NamedKey::Standby => Key::Standby,
            NamedKey::WakeUp => Key::WakeUp,
            NamedKey::AllCandidates => Key::AllCandidates,
            NamedKey::Alphanumeric => Key::Alphanumeric,
            NamedKey::CodeInput => Key::CodeInput,
            NamedKey::Compose => Key::Compose,
            NamedKey::FinalMode => Key::FinalMode,
            NamedKey::GroupFirst => Key::GroupFirst,
            NamedKey::GroupLast => Key::GroupLast,
            NamedKey::GroupNext => Key::GroupNext,
            NamedKey::GroupPrevious => Key::GroupPrevious,
            NamedKey::ModeChange => Key::ModeChange,
            NamedKey::NextCandidate => Key::NextCandidate,
            NamedKey::NonConvert => Key::NonConvert,
            NamedKey::PreviousCandidate => Key::PreviousCandidate,
            NamedKey::Process => Key::Process,
            NamedKey::SingleCandidate => Key::SingleCandidate,
            NamedKey::HangulMode => Key::HangulMode,
            NamedKey::HanjaMode => Key::HanjaMode,
            NamedKey::JunjaMode => Key::JunjaMode,
            NamedKey::Eisu => Key::Eisu,
            NamedKey::Hankaku => Key::Hankaku,
            NamedKey::Hiragana => Key::Hiragana,
            NamedKey::HiraganaKatakana => Key::HiraganaKatakana,
            NamedKey::KanaMode => Key::KanaMode,
            NamedKey::KanjiMode => Key::KanjiMode,
            NamedKey::Katakana => Key::Katakana,
            NamedKey::Romaji => Key::Romaji,
            NamedKey::Zenkaku => Key::Zenkaku,
            NamedKey::ZenkakuHankaku => Key::ZenkakuHankaku,
            NamedKey::Soft1 => Key::Soft1,
            NamedKey::Soft2 => Key::Soft2,
            NamedKey::Soft3 => Key::Soft3,
            NamedKey::Soft4 => Key::Soft4,
            NamedKey::ChannelDown => Key::ChannelDown,
            NamedKey::ChannelUp => Key::ChannelUp,
            NamedKey::Close => Key::Close,
            NamedKey::MailForward => Key::MailForward,
            NamedKey::MailReply => Key::MailReply,
            NamedKey::MailSend => Key::MailSend,
            NamedKey::MediaClose => Key::MediaClose,
            NamedKey::MediaFastForward => Key::MediaFastForward,
            NamedKey::MediaPause => Key::MediaPause,
            NamedKey::MediaPlay => Key::MediaPlay,
            NamedKey::MediaRecord => Key::MediaRecord,
            _ => Key::Unidentified,
        },
        _ => Key::Unidentified,
    }
}

/// Return the equivalent of Winit's `PhysicalKey` in keyboard_types
pub fn map_winit_physical_key(key: &keyboard::PhysicalKey) -> Code {
    if let keyboard::PhysicalKey::Code(key) = key {
        match key {
            keyboard::KeyCode::Digit1 => Code::Digit1,
            keyboard::KeyCode::Digit2 => Code::Digit2,
            keyboard::KeyCode::Digit3 => Code::Digit3,
            keyboard::KeyCode::Digit4 => Code::Digit4,
            keyboard::KeyCode::Digit5 => Code::Digit5,
            keyboard::KeyCode::Digit6 => Code::Digit6,
            keyboard::KeyCode::Digit7 => Code::Digit7,
            keyboard::KeyCode::Digit8 => Code::Digit8,
            keyboard::KeyCode::Digit9 => Code::Digit9,
            keyboard::KeyCode::Digit0 => Code::Digit0,
            keyboard::KeyCode::KeyA => Code::KeyA,
            keyboard::KeyCode::KeyB => Code::KeyB,
            keyboard::KeyCode::KeyC => Code::KeyC,
            keyboard::KeyCode::KeyD => Code::KeyD,
            keyboard::KeyCode::KeyE => Code::KeyE,
            keyboard::KeyCode::KeyF => Code::KeyF,
            keyboard::KeyCode::KeyG => Code::KeyG,
            keyboard::KeyCode::KeyH => Code::KeyH,
            keyboard::KeyCode::KeyI => Code::KeyI,
            keyboard::KeyCode::KeyJ => Code::KeyJ,
            keyboard::KeyCode::KeyK => Code::KeyK,
            keyboard::KeyCode::KeyL => Code::KeyL,
            keyboard::KeyCode::KeyM => Code::KeyM,
            keyboard::KeyCode::KeyN => Code::KeyN,
            keyboard::KeyCode::KeyO => Code::KeyO,
            keyboard::KeyCode::KeyP => Code::KeyP,
            keyboard::KeyCode::KeyQ => Code::KeyQ,
            keyboard::KeyCode::KeyR => Code::KeyR,
            keyboard::KeyCode::KeyS => Code::KeyS,
            keyboard::KeyCode::KeyT => Code::KeyT,
            keyboard::KeyCode::KeyU => Code::KeyU,
            keyboard::KeyCode::KeyV => Code::KeyV,
            keyboard::KeyCode::KeyW => Code::KeyW,
            keyboard::KeyCode::KeyX => Code::KeyX,
            keyboard::KeyCode::KeyY => Code::KeyY,
            keyboard::KeyCode::KeyZ => Code::KeyZ,
            keyboard::KeyCode::Escape => Code::Escape,
            keyboard::KeyCode::F1 => Code::F1,
            keyboard::KeyCode::F2 => Code::F2,
            keyboard::KeyCode::F3 => Code::F3,
            keyboard::KeyCode::F4 => Code::F4,
            keyboard::KeyCode::F5 => Code::F5,
            keyboard::KeyCode::F6 => Code::F6,
            keyboard::KeyCode::F7 => Code::F7,
            keyboard::KeyCode::F8 => Code::F8,
            keyboard::KeyCode::F9 => Code::F9,
            keyboard::KeyCode::F10 => Code::F10,
            keyboard::KeyCode::F11 => Code::F11,
            keyboard::KeyCode::F12 => Code::F12,
            keyboard::KeyCode::F13 => Code::F13,
            keyboard::KeyCode::F14 => Code::F14,
            keyboard::KeyCode::F15 => Code::F15,
            keyboard::KeyCode::F16 => Code::F16,
            keyboard::KeyCode::F17 => Code::F17,
            keyboard::KeyCode::F18 => Code::F18,
            keyboard::KeyCode::F19 => Code::F19,
            keyboard::KeyCode::F20 => Code::F20,
            keyboard::KeyCode::F21 => Code::F21,
            keyboard::KeyCode::F22 => Code::F22,
            keyboard::KeyCode::F23 => Code::F23,
            keyboard::KeyCode::F24 => Code::F24,
            keyboard::KeyCode::Pause => Code::Pause,
            keyboard::KeyCode::Insert => Code::Insert,
            keyboard::KeyCode::Home => Code::Home,
            keyboard::KeyCode::Delete => Code::Delete,
            keyboard::KeyCode::End => Code::End,
            keyboard::KeyCode::PageDown => Code::PageDown,
            keyboard::KeyCode::PageUp => Code::PageUp,
            keyboard::KeyCode::ArrowLeft => Code::ArrowLeft,
            keyboard::KeyCode::ArrowUp => Code::ArrowUp,
            keyboard::KeyCode::ArrowRight => Code::ArrowRight,
            keyboard::KeyCode::ArrowDown => Code::ArrowDown,
            keyboard::KeyCode::Backspace => Code::Backspace,
            keyboard::KeyCode::Enter => Code::Enter,
            keyboard::KeyCode::Space => Code::Space,
            keyboard::KeyCode::NumLock => Code::NumLock,
            keyboard::KeyCode::Numpad0 => Code::Numpad0,
            keyboard::KeyCode::Numpad1 => Code::Numpad1,
            keyboard::KeyCode::Numpad2 => Code::Numpad2,
            keyboard::KeyCode::Numpad3 => Code::Numpad3,
            keyboard::KeyCode::Numpad4 => Code::Numpad4,
            keyboard::KeyCode::Numpad5 => Code::Numpad5,
            keyboard::KeyCode::Numpad6 => Code::Numpad6,
            keyboard::KeyCode::Numpad7 => Code::Numpad7,
            keyboard::KeyCode::Numpad8 => Code::Numpad8,
            keyboard::KeyCode::Numpad9 => Code::Numpad9,
            keyboard::KeyCode::NumpadAdd => Code::NumpadAdd,
            keyboard::KeyCode::NumpadDivide => Code::NumpadDivide,
            keyboard::KeyCode::NumpadDecimal => Code::NumpadDecimal,
            keyboard::KeyCode::NumpadComma => Code::NumpadComma,
            keyboard::KeyCode::NumpadEnter => Code::NumpadEnter,
            keyboard::KeyCode::NumpadEqual => Code::NumpadEqual,
            keyboard::KeyCode::NumpadMultiply => Code::NumpadMultiply,
            keyboard::KeyCode::NumpadSubtract => Code::NumpadSubtract,
            keyboard::KeyCode::Backslash => Code::Backslash,
            keyboard::KeyCode::Comma => Code::Comma,
            keyboard::KeyCode::Convert => Code::Convert,
            keyboard::KeyCode::Equal => Code::Equal,
            keyboard::KeyCode::BracketLeft => Code::BracketLeft,
            keyboard::KeyCode::BracketRight => Code::BracketRight,
            keyboard::KeyCode::ShiftLeft => Code::ShiftLeft,
            keyboard::KeyCode::Meta => Code::MetaLeft,
            keyboard::KeyCode::MediaSelect => Code::Unidentified,
            keyboard::KeyCode::MediaStop => Code::Unidentified,
            keyboard::KeyCode::Minus => Code::Unidentified,
            keyboard::KeyCode::Period => Code::Unidentified,
            keyboard::KeyCode::Power => Code::Unidentified,
            keyboard::KeyCode::AltRight => Code::AltRight,
            keyboard::KeyCode::ControlLeft => Code::ControlLeft,
            keyboard::KeyCode::ControlRight => Code::ControlRight,
            keyboard::KeyCode::ShiftRight => Code::ShiftRight,
            keyboard::KeyCode::Semicolon => Code::Semicolon,
            keyboard::KeyCode::Slash => Code::Unidentified,
            keyboard::KeyCode::Sleep => Code::Unidentified,
            keyboard::KeyCode::Tab => Code::Tab,
            keyboard::KeyCode::AudioVolumeUp => Code::AudioVolumeUp,
            keyboard::KeyCode::IntlYen => Code::IntlYen,
            keyboard::KeyCode::Copy => Code::Copy,
            keyboard::KeyCode::Paste => Code::Paste,
            keyboard::KeyCode::Cut => Code::Cut,
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

impl From<&PlatformEventData> for KeyboardData {
    fn from(val: &PlatformEventData) -> Self {
        val.downcast::<KeyboardData>().cloned().unwrap()
    }
}
