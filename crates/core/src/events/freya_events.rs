use freya_elements::events::keyboard::{Code, Key, Modifiers};
use torin::prelude::*;
use winit::event::{Force, MouseButton, TouchPhase};

/// Events emitted in Freya.
#[derive(Clone, Debug)]
pub enum FreyaEvent {
    /// A Mouse Event.
    Mouse {
        name: String,
        cursor: CursorPoint,
        button: Option<MouseButton>,
    },
    /// A Wheel event.
    Wheel {
        name: String,
        scroll: CursorPoint,
        cursor: CursorPoint,
    },
    /// A Keyboard event.
    Keyboard {
        name: String,
        key: Key,
        code: Code,
        modifiers: Modifiers,
    },
    /// A Touch event.
    Touch {
        name: String,
        location: CursorPoint,
        finger_id: u64,
        phase: TouchPhase,
        force: Option<Force>,
    },
}

impl FreyaEvent {
    pub fn get_name(&self) -> &str {
        match self {
            Self::Mouse { name, .. } => name,
            Self::Wheel { name, .. } => name,
            Self::Keyboard { name, .. } => name,
            Self::Touch { name, .. } => name,
        }
    }

    pub fn set_name(&mut self, new_name: String) {
        match self {
            Self::Mouse { name, .. } => *name = new_name,
            Self::Wheel { name, .. } => *name = new_name,
            Self::Keyboard { name, .. } => *name = new_name,
            Self::Touch { name, .. } => *name = new_name,
        }
    }

    pub fn is_pointer_event(&self) -> bool {
        self.get_name().starts_with("point")
    }

    pub fn is_keyboard_event(&self) -> bool {
        matches!(self, Self::Keyboard { .. })
    }
}
