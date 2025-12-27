use freya_core::prelude::{
    Force,
    MouseButton,
    TouchPhase,
};
use keyboard_types::{
    Code,
    NamedKey,
};
pub use keyboard_types::{
    Key,
    Modifiers,
};

pub fn map_winit_mouse_button(event: winit::event::MouseButton) -> MouseButton {
    match event {
        winit::event::MouseButton::Left => MouseButton::Left,
        winit::event::MouseButton::Right => MouseButton::Right,
        winit::event::MouseButton::Middle => MouseButton::Middle,
        winit::event::MouseButton::Back => MouseButton::Back,
        winit::event::MouseButton::Forward => MouseButton::Forward,
        winit::event::MouseButton::Other(o) => MouseButton::Other(o),
    }
}

// Return the equivalent of Winit `ModifiersState` in keyboard_types
pub fn map_winit_modifiers(modifiers: winit::keyboard::ModifiersState) -> Modifiers {
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
pub fn map_winit_key(key: &winit::keyboard::Key) -> Key {
    match key {
        winit::keyboard::Key::Character(c) => Key::Character(c.to_string()),

        winit::keyboard::Key::Named(named_key) => match named_key {
            winit::keyboard::NamedKey::Space => Key::Character(" ".to_string()),

            winit::keyboard::NamedKey::Delete => Key::Named(NamedKey::Delete),
            winit::keyboard::NamedKey::Backspace => Key::Named(NamedKey::Backspace),
            winit::keyboard::NamedKey::ArrowDown => Key::Named(NamedKey::ArrowDown),
            winit::keyboard::NamedKey::ArrowLeft => Key::Named(NamedKey::ArrowLeft),
            winit::keyboard::NamedKey::ArrowRight => Key::Named(NamedKey::ArrowRight),
            winit::keyboard::NamedKey::ArrowUp => Key::Named(NamedKey::ArrowUp),
            winit::keyboard::NamedKey::End => Key::Named(NamedKey::End),
            winit::keyboard::NamedKey::Home => Key::Named(NamedKey::Home),
            winit::keyboard::NamedKey::PageDown => Key::Named(NamedKey::PageDown),
            winit::keyboard::NamedKey::PageUp => Key::Named(NamedKey::PageUp),
            winit::keyboard::NamedKey::Tab => Key::Named(NamedKey::Tab),
            winit::keyboard::NamedKey::Enter => Key::Named(NamedKey::Enter),
            winit::keyboard::NamedKey::Escape => Key::Named(NamedKey::Escape),

            winit::keyboard::NamedKey::F1 => Key::Named(NamedKey::F1),
            winit::keyboard::NamedKey::F2 => Key::Named(NamedKey::F2),
            winit::keyboard::NamedKey::F3 => Key::Named(NamedKey::F3),
            winit::keyboard::NamedKey::F4 => Key::Named(NamedKey::F4),
            winit::keyboard::NamedKey::F5 => Key::Named(NamedKey::F5),
            winit::keyboard::NamedKey::F6 => Key::Named(NamedKey::F6),
            winit::keyboard::NamedKey::F7 => Key::Named(NamedKey::F7),
            winit::keyboard::NamedKey::F8 => Key::Named(NamedKey::F8),
            winit::keyboard::NamedKey::F9 => Key::Named(NamedKey::F9),
            winit::keyboard::NamedKey::F10 => Key::Named(NamedKey::F10),
            winit::keyboard::NamedKey::F11 => Key::Named(NamedKey::F11),
            winit::keyboard::NamedKey::F12 => Key::Named(NamedKey::F12),
            winit::keyboard::NamedKey::F13 => Key::Named(NamedKey::F13),
            winit::keyboard::NamedKey::F14 => Key::Named(NamedKey::F14),
            winit::keyboard::NamedKey::F15 => Key::Named(NamedKey::F15),
            winit::keyboard::NamedKey::F16 => Key::Named(NamedKey::F16),
            winit::keyboard::NamedKey::F17 => Key::Named(NamedKey::F17),
            winit::keyboard::NamedKey::F18 => Key::Named(NamedKey::F18),
            winit::keyboard::NamedKey::F19 => Key::Named(NamedKey::F19),
            winit::keyboard::NamedKey::F20 => Key::Named(NamedKey::F20),
            winit::keyboard::NamedKey::F21 => Key::Named(NamedKey::F21),
            winit::keyboard::NamedKey::F22 => Key::Named(NamedKey::F22),
            winit::keyboard::NamedKey::F23 => Key::Named(NamedKey::F23),
            winit::keyboard::NamedKey::F24 => Key::Named(NamedKey::F24),

            winit::keyboard::NamedKey::Pause => Key::Named(NamedKey::Pause),
            winit::keyboard::NamedKey::Insert => Key::Named(NamedKey::Insert),
            winit::keyboard::NamedKey::ContextMenu => Key::Named(NamedKey::ContextMenu),

            winit::keyboard::NamedKey::BrowserBack => Key::Named(NamedKey::BrowserBack),
            winit::keyboard::NamedKey::BrowserFavorites => Key::Named(NamedKey::BrowserFavorites),
            winit::keyboard::NamedKey::BrowserForward => Key::Named(NamedKey::BrowserForward),
            winit::keyboard::NamedKey::BrowserHome => Key::Named(NamedKey::BrowserHome),
            winit::keyboard::NamedKey::BrowserRefresh => Key::Named(NamedKey::BrowserRefresh),
            winit::keyboard::NamedKey::BrowserSearch => Key::Named(NamedKey::BrowserSearch),
            winit::keyboard::NamedKey::BrowserStop => Key::Named(NamedKey::BrowserStop),

            winit::keyboard::NamedKey::MediaTrackNext => Key::Named(NamedKey::MediaTrackNext),
            winit::keyboard::NamedKey::MediaPlayPause => Key::Named(NamedKey::MediaPlayPause),
            winit::keyboard::NamedKey::MediaTrackPrevious => {
                Key::Named(NamedKey::MediaTrackPrevious)
            }
            winit::keyboard::NamedKey::MediaStop => Key::Named(NamedKey::MediaStop),

            winit::keyboard::NamedKey::AudioVolumeDown => Key::Named(NamedKey::AudioVolumeDown),
            winit::keyboard::NamedKey::AudioVolumeMute => Key::Named(NamedKey::AudioVolumeMute),
            winit::keyboard::NamedKey::AudioVolumeUp => Key::Named(NamedKey::AudioVolumeUp),

            winit::keyboard::NamedKey::LaunchApplication2 => {
                Key::Named(NamedKey::LaunchApplication2)
            }
            winit::keyboard::NamedKey::LaunchMail => Key::Named(NamedKey::LaunchMail),

            winit::keyboard::NamedKey::Convert => Key::Named(NamedKey::Convert),

            winit::keyboard::NamedKey::Alt => Key::Named(NamedKey::Alt),
            winit::keyboard::NamedKey::AltGraph => Key::Named(NamedKey::AltGraph),
            winit::keyboard::NamedKey::CapsLock => Key::Named(NamedKey::CapsLock),
            winit::keyboard::NamedKey::Control => Key::Named(NamedKey::Control),
            winit::keyboard::NamedKey::Fn => Key::Named(NamedKey::Fn),
            winit::keyboard::NamedKey::FnLock => Key::Named(NamedKey::FnLock),
            winit::keyboard::NamedKey::NumLock => Key::Named(NamedKey::NumLock),
            winit::keyboard::NamedKey::ScrollLock => Key::Named(NamedKey::ScrollLock),
            winit::keyboard::NamedKey::Shift => Key::Named(NamedKey::Shift),
            winit::keyboard::NamedKey::Symbol => Key::Named(NamedKey::Symbol),
            winit::keyboard::NamedKey::SymbolLock => Key::Named(NamedKey::SymbolLock),
            winit::keyboard::NamedKey::Meta => Key::Named(NamedKey::Meta),
            winit::keyboard::NamedKey::Hyper => Key::Named(NamedKey::Meta),
            winit::keyboard::NamedKey::Super => Key::Named(NamedKey::Meta),

            winit::keyboard::NamedKey::Clear => Key::Named(NamedKey::Clear),
            winit::keyboard::NamedKey::Copy => Key::Named(NamedKey::Copy),
            winit::keyboard::NamedKey::CrSel => Key::Named(NamedKey::CrSel),
            winit::keyboard::NamedKey::Cut => Key::Named(NamedKey::Cut),
            winit::keyboard::NamedKey::EraseEof => Key::Named(NamedKey::EraseEof),
            winit::keyboard::NamedKey::ExSel => Key::Named(NamedKey::ExSel),
            winit::keyboard::NamedKey::Paste => Key::Named(NamedKey::Paste),
            winit::keyboard::NamedKey::Redo => Key::Named(NamedKey::Redo),
            winit::keyboard::NamedKey::Undo => Key::Named(NamedKey::Undo),

            winit::keyboard::NamedKey::Accept => Key::Named(NamedKey::Accept),
            winit::keyboard::NamedKey::Again => Key::Named(NamedKey::Again),
            winit::keyboard::NamedKey::Attn => Key::Named(NamedKey::Attn),
            winit::keyboard::NamedKey::Cancel => Key::Named(NamedKey::Cancel),
            winit::keyboard::NamedKey::Execute => Key::Named(NamedKey::Execute),
            winit::keyboard::NamedKey::Find => Key::Named(NamedKey::Find),
            winit::keyboard::NamedKey::Help => Key::Named(NamedKey::Help),
            winit::keyboard::NamedKey::Play => Key::Named(NamedKey::Play),
            winit::keyboard::NamedKey::Props => Key::Named(NamedKey::Props),
            winit::keyboard::NamedKey::Select => Key::Named(NamedKey::Select),
            winit::keyboard::NamedKey::ZoomIn => Key::Named(NamedKey::ZoomIn),
            winit::keyboard::NamedKey::ZoomOut => Key::Named(NamedKey::ZoomOut),

            winit::keyboard::NamedKey::BrightnessDown => Key::Named(NamedKey::BrightnessDown),
            winit::keyboard::NamedKey::BrightnessUp => Key::Named(NamedKey::BrightnessUp),

            winit::keyboard::NamedKey::Eject => Key::Named(NamedKey::Eject),
            winit::keyboard::NamedKey::LogOff => Key::Named(NamedKey::LogOff),
            winit::keyboard::NamedKey::Power => Key::Named(NamedKey::Power),
            winit::keyboard::NamedKey::PowerOff => Key::Named(NamedKey::PowerOff),
            winit::keyboard::NamedKey::PrintScreen => Key::Named(NamedKey::PrintScreen),
            winit::keyboard::NamedKey::Hibernate => Key::Named(NamedKey::Hibernate),
            winit::keyboard::NamedKey::Standby => Key::Named(NamedKey::Standby),
            winit::keyboard::NamedKey::WakeUp => Key::Named(NamedKey::WakeUp),

            winit::keyboard::NamedKey::AllCandidates => Key::Named(NamedKey::AllCandidates),
            winit::keyboard::NamedKey::Alphanumeric => Key::Named(NamedKey::Alphanumeric),
            winit::keyboard::NamedKey::CodeInput => Key::Named(NamedKey::CodeInput),
            winit::keyboard::NamedKey::Compose => Key::Named(NamedKey::Compose),
            winit::keyboard::NamedKey::FinalMode => Key::Named(NamedKey::FinalMode),
            winit::keyboard::NamedKey::GroupFirst => Key::Named(NamedKey::GroupFirst),
            winit::keyboard::NamedKey::GroupLast => Key::Named(NamedKey::GroupLast),
            winit::keyboard::NamedKey::GroupNext => Key::Named(NamedKey::GroupNext),
            winit::keyboard::NamedKey::GroupPrevious => Key::Named(NamedKey::GroupPrevious),
            winit::keyboard::NamedKey::ModeChange => Key::Named(NamedKey::ModeChange),
            winit::keyboard::NamedKey::NextCandidate => Key::Named(NamedKey::NextCandidate),
            winit::keyboard::NamedKey::NonConvert => Key::Named(NamedKey::NonConvert),
            winit::keyboard::NamedKey::PreviousCandidate => Key::Named(NamedKey::PreviousCandidate),
            winit::keyboard::NamedKey::Process => Key::Named(NamedKey::Process),
            winit::keyboard::NamedKey::SingleCandidate => Key::Named(NamedKey::SingleCandidate),

            winit::keyboard::NamedKey::HangulMode => Key::Named(NamedKey::HangulMode),
            winit::keyboard::NamedKey::HanjaMode => Key::Named(NamedKey::HanjaMode),
            winit::keyboard::NamedKey::JunjaMode => Key::Named(NamedKey::JunjaMode),
            winit::keyboard::NamedKey::Eisu => Key::Named(NamedKey::Eisu),
            winit::keyboard::NamedKey::Hankaku => Key::Named(NamedKey::Hankaku),
            winit::keyboard::NamedKey::Hiragana => Key::Named(NamedKey::Hiragana),
            winit::keyboard::NamedKey::HiraganaKatakana => Key::Named(NamedKey::HiraganaKatakana),
            winit::keyboard::NamedKey::KanaMode => Key::Named(NamedKey::KanaMode),
            winit::keyboard::NamedKey::KanjiMode => Key::Named(NamedKey::KanjiMode),
            winit::keyboard::NamedKey::Katakana => Key::Named(NamedKey::Katakana),
            winit::keyboard::NamedKey::Romaji => Key::Named(NamedKey::Romaji),
            winit::keyboard::NamedKey::Zenkaku => Key::Named(NamedKey::Zenkaku),
            winit::keyboard::NamedKey::ZenkakuHankaku => Key::Named(NamedKey::ZenkakuHankaku),

            winit::keyboard::NamedKey::Soft1 => Key::Named(NamedKey::Soft1),
            winit::keyboard::NamedKey::Soft2 => Key::Named(NamedKey::Soft2),
            winit::keyboard::NamedKey::Soft3 => Key::Named(NamedKey::Soft3),
            winit::keyboard::NamedKey::Soft4 => Key::Named(NamedKey::Soft4),

            winit::keyboard::NamedKey::ChannelDown => Key::Named(NamedKey::ChannelDown),
            winit::keyboard::NamedKey::ChannelUp => Key::Named(NamedKey::ChannelUp),
            winit::keyboard::NamedKey::Close => Key::Named(NamedKey::Close),

            winit::keyboard::NamedKey::MailForward => Key::Named(NamedKey::MailForward),
            winit::keyboard::NamedKey::MailReply => Key::Named(NamedKey::MailReply),
            winit::keyboard::NamedKey::MailSend => Key::Named(NamedKey::MailSend),

            winit::keyboard::NamedKey::MediaClose => Key::Named(NamedKey::MediaClose),
            winit::keyboard::NamedKey::MediaFastForward => Key::Named(NamedKey::MediaFastForward),
            winit::keyboard::NamedKey::MediaPause => Key::Named(NamedKey::MediaPause),
            winit::keyboard::NamedKey::MediaPlay => Key::Named(NamedKey::MediaPlay),
            winit::keyboard::NamedKey::MediaRecord => Key::Named(NamedKey::MediaRecord),

            _ => Key::Named(NamedKey::Unidentified),
        },

        _ => Key::Named(NamedKey::Unidentified),
    }
}

/// Return the equivalent of Winit's `PhysicalKey` in keyboard_types
pub fn map_winit_physical_key(key: &winit::keyboard::PhysicalKey) -> Code {
    if let winit::keyboard::PhysicalKey::Code(key) = key {
        match key {
            winit::keyboard::KeyCode::Digit1 => Code::Digit1,
            winit::keyboard::KeyCode::Digit2 => Code::Digit2,
            winit::keyboard::KeyCode::Digit3 => Code::Digit3,
            winit::keyboard::KeyCode::Digit4 => Code::Digit4,
            winit::keyboard::KeyCode::Digit5 => Code::Digit5,
            winit::keyboard::KeyCode::Digit6 => Code::Digit6,
            winit::keyboard::KeyCode::Digit7 => Code::Digit7,
            winit::keyboard::KeyCode::Digit8 => Code::Digit8,
            winit::keyboard::KeyCode::Digit9 => Code::Digit9,
            winit::keyboard::KeyCode::Digit0 => Code::Digit0,
            winit::keyboard::KeyCode::KeyA => Code::KeyA,
            winit::keyboard::KeyCode::KeyB => Code::KeyB,
            winit::keyboard::KeyCode::KeyC => Code::KeyC,
            winit::keyboard::KeyCode::KeyD => Code::KeyD,
            winit::keyboard::KeyCode::KeyE => Code::KeyE,
            winit::keyboard::KeyCode::KeyF => Code::KeyF,
            winit::keyboard::KeyCode::KeyG => Code::KeyG,
            winit::keyboard::KeyCode::KeyH => Code::KeyH,
            winit::keyboard::KeyCode::KeyI => Code::KeyI,
            winit::keyboard::KeyCode::KeyJ => Code::KeyJ,
            winit::keyboard::KeyCode::KeyK => Code::KeyK,
            winit::keyboard::KeyCode::KeyL => Code::KeyL,
            winit::keyboard::KeyCode::KeyM => Code::KeyM,
            winit::keyboard::KeyCode::KeyN => Code::KeyN,
            winit::keyboard::KeyCode::KeyO => Code::KeyO,
            winit::keyboard::KeyCode::KeyP => Code::KeyP,
            winit::keyboard::KeyCode::KeyQ => Code::KeyQ,
            winit::keyboard::KeyCode::KeyR => Code::KeyR,
            winit::keyboard::KeyCode::KeyS => Code::KeyS,
            winit::keyboard::KeyCode::KeyT => Code::KeyT,
            winit::keyboard::KeyCode::KeyU => Code::KeyU,
            winit::keyboard::KeyCode::KeyV => Code::KeyV,
            winit::keyboard::KeyCode::KeyW => Code::KeyW,
            winit::keyboard::KeyCode::KeyX => Code::KeyX,
            winit::keyboard::KeyCode::KeyY => Code::KeyY,
            winit::keyboard::KeyCode::KeyZ => Code::KeyZ,
            winit::keyboard::KeyCode::Escape => Code::Escape,
            winit::keyboard::KeyCode::F1 => Code::F1,
            winit::keyboard::KeyCode::F2 => Code::F2,
            winit::keyboard::KeyCode::F3 => Code::F3,
            winit::keyboard::KeyCode::F4 => Code::F4,
            winit::keyboard::KeyCode::F5 => Code::F5,
            winit::keyboard::KeyCode::F6 => Code::F6,
            winit::keyboard::KeyCode::F7 => Code::F7,
            winit::keyboard::KeyCode::F8 => Code::F8,
            winit::keyboard::KeyCode::F9 => Code::F9,
            winit::keyboard::KeyCode::F10 => Code::F10,
            winit::keyboard::KeyCode::F11 => Code::F11,
            winit::keyboard::KeyCode::F12 => Code::F12,
            winit::keyboard::KeyCode::F13 => Code::F13,
            winit::keyboard::KeyCode::F14 => Code::F14,
            winit::keyboard::KeyCode::F15 => Code::F15,
            winit::keyboard::KeyCode::F16 => Code::F16,
            winit::keyboard::KeyCode::F17 => Code::F17,
            winit::keyboard::KeyCode::F18 => Code::F18,
            winit::keyboard::KeyCode::F19 => Code::F19,
            winit::keyboard::KeyCode::F20 => Code::F20,
            winit::keyboard::KeyCode::F21 => Code::F21,
            winit::keyboard::KeyCode::F22 => Code::F22,
            winit::keyboard::KeyCode::F23 => Code::F23,
            winit::keyboard::KeyCode::F24 => Code::F24,
            winit::keyboard::KeyCode::Pause => Code::Pause,
            winit::keyboard::KeyCode::Insert => Code::Insert,
            winit::keyboard::KeyCode::Home => Code::Home,
            winit::keyboard::KeyCode::Delete => Code::Delete,
            winit::keyboard::KeyCode::End => Code::End,
            winit::keyboard::KeyCode::PageDown => Code::PageDown,
            winit::keyboard::KeyCode::PageUp => Code::PageUp,
            winit::keyboard::KeyCode::ArrowLeft => Code::ArrowLeft,
            winit::keyboard::KeyCode::ArrowUp => Code::ArrowUp,
            winit::keyboard::KeyCode::ArrowRight => Code::ArrowRight,
            winit::keyboard::KeyCode::ArrowDown => Code::ArrowDown,
            winit::keyboard::KeyCode::Backspace => Code::Backspace,
            winit::keyboard::KeyCode::Enter => Code::Enter,
            winit::keyboard::KeyCode::Space => Code::Space,
            winit::keyboard::KeyCode::NumLock => Code::NumLock,
            winit::keyboard::KeyCode::Numpad0 => Code::Numpad0,
            winit::keyboard::KeyCode::Numpad1 => Code::Numpad1,
            winit::keyboard::KeyCode::Numpad2 => Code::Numpad2,
            winit::keyboard::KeyCode::Numpad3 => Code::Numpad3,
            winit::keyboard::KeyCode::Numpad4 => Code::Numpad4,
            winit::keyboard::KeyCode::Numpad5 => Code::Numpad5,
            winit::keyboard::KeyCode::Numpad6 => Code::Numpad6,
            winit::keyboard::KeyCode::Numpad7 => Code::Numpad7,
            winit::keyboard::KeyCode::Numpad8 => Code::Numpad8,
            winit::keyboard::KeyCode::Numpad9 => Code::Numpad9,
            winit::keyboard::KeyCode::NumpadAdd => Code::NumpadAdd,
            winit::keyboard::KeyCode::NumpadDivide => Code::NumpadDivide,
            winit::keyboard::KeyCode::NumpadDecimal => Code::NumpadDecimal,
            winit::keyboard::KeyCode::NumpadComma => Code::NumpadComma,
            winit::keyboard::KeyCode::NumpadEnter => Code::NumpadEnter,
            winit::keyboard::KeyCode::NumpadEqual => Code::NumpadEqual,
            winit::keyboard::KeyCode::NumpadMultiply => Code::NumpadMultiply,
            winit::keyboard::KeyCode::NumpadSubtract => Code::NumpadSubtract,
            winit::keyboard::KeyCode::Backslash => Code::Backslash,
            winit::keyboard::KeyCode::Comma => Code::Comma,
            winit::keyboard::KeyCode::Convert => Code::Convert,
            winit::keyboard::KeyCode::Equal => Code::Equal,
            winit::keyboard::KeyCode::BracketLeft => Code::BracketLeft,
            winit::keyboard::KeyCode::BracketRight => Code::BracketRight,
            winit::keyboard::KeyCode::ShiftLeft => Code::ShiftLeft,
            winit::keyboard::KeyCode::Meta => Code::MetaLeft,
            winit::keyboard::KeyCode::MediaSelect => Code::MediaSelect,
            winit::keyboard::KeyCode::MediaStop => Code::MediaStop,
            winit::keyboard::KeyCode::Minus => Code::Minus,
            winit::keyboard::KeyCode::Period => Code::Period,
            winit::keyboard::KeyCode::Power => Code::Power,
            winit::keyboard::KeyCode::AltRight => Code::AltRight,
            winit::keyboard::KeyCode::ControlLeft => Code::ControlLeft,
            winit::keyboard::KeyCode::ControlRight => Code::ControlRight,
            winit::keyboard::KeyCode::ShiftRight => Code::ShiftRight,
            winit::keyboard::KeyCode::Semicolon => Code::Semicolon,
            winit::keyboard::KeyCode::Slash => Code::Slash,
            winit::keyboard::KeyCode::Sleep => Code::Sleep,
            winit::keyboard::KeyCode::Tab => Code::Tab,
            winit::keyboard::KeyCode::AudioVolumeUp => Code::AudioVolumeUp,
            winit::keyboard::KeyCode::IntlYen => Code::IntlYen,
            winit::keyboard::KeyCode::Copy => Code::Copy,
            winit::keyboard::KeyCode::Paste => Code::Paste,
            winit::keyboard::KeyCode::Cut => Code::Cut,
            _ => Code::Unidentified,
        }
    } else {
        Code::Unidentified
    }
}

pub fn map_winit_touch_phase(event: winit::event::TouchPhase) -> TouchPhase {
    match event {
        winit::event::TouchPhase::Started => TouchPhase::Started,
        winit::event::TouchPhase::Moved => TouchPhase::Moved,
        winit::event::TouchPhase::Ended => TouchPhase::Ended,
        winit::event::TouchPhase::Cancelled => TouchPhase::Cancelled,
    }
}

pub fn map_winit_touch_force(event: winit::event::Force) -> Force {
    match event {
        winit::event::Force::Calibrated {
            force,
            max_possible_force,
            altitude_angle,
        } => Force::Calibrated {
            force,
            max_possible_force,
            altitude_angle,
        },
        winit::event::Force::Normalized(f) => Force::Normalized(f),
    }
}
