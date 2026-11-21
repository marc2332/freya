use freya_core::prelude::{
    Force,
    MouseButton,
    TouchPhase,
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
        button => MouseButton::Other(button as u16),
    }
}

pub fn map_winit_modifiers(modifiers: winit::keyboard::ModifiersState) -> Modifiers {
    let mut mapped = Modifiers::empty();
    mapped.set(Modifiers::ALT, modifiers.alt_key());
    mapped.set(Modifiers::CONTROL, modifiers.control_key());
    mapped.set(Modifiers::SHIFT, modifiers.shift_key());
    mapped.set(Modifiers::META, modifiers.meta_key());
    mapped
}

pub fn map_winit_key(key: &winit::keyboard::Key) -> Key {
    match key {
        winit::keyboard::Key::Character(character) => Key::Character(character.to_string()),
        winit::keyboard::Key::Unidentified(winit::keyboard::NativeKey::Xkb(0xFE50))
        | winit::keyboard::Key::Dead(Some('`')) => Key::Character("`".to_string()),
        winit::keyboard::Key::Named(key) => Key::Named(*key),
        winit::keyboard::Key::Dead(_) => Key::Named(keyboard_types::NamedKey::Dead),
        _ => Key::Named(keyboard_types::NamedKey::Unidentified),
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
        } => Force::Calibrated {
            force,
            max_possible_force,
            altitude_angle: None,
        },
        winit::event::Force::Normalized(force) => Force::Normalized(force),
    }
}
