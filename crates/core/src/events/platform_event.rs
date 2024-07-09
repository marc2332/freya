use std::path::PathBuf;

use freya_elements::events::keyboard::{Code, Key, Modifiers};
use torin::prelude::*;
use winit::event::{Force, MouseButton, TouchPhase};

use crate::prelude::EventName;

/// Events emitted in Freya.
#[derive(Clone, Debug)]
pub enum PlatformEvent {
    /// A Mouse Event.
    Mouse {
        name: EventName,
        cursor: CursorPoint,
        button: Option<MouseButton>,
    },
    /// A Wheel event.
    Wheel {
        name: EventName,
        scroll: CursorPoint,
        cursor: CursorPoint,
    },
    /// A Keyboard event.
    Keyboard {
        name: EventName,
        key: Key,
        code: Code,
        modifiers: Modifiers,
    },
    /// A Touch event.
    Touch {
        name: EventName,
        location: CursorPoint,
        finger_id: u64,
        phase: TouchPhase,
        force: Option<Force>,
    },
    /// A File event.
    File {
        name: EventName,
        cursor: CursorPoint,
        file_path: Option<PathBuf>,
    },
}

impl PlatformEvent {
    pub fn get_name(&self) -> EventName {
        match self {
            Self::Mouse { name, .. } => *name,
            Self::Wheel { name, .. } => *name,
            Self::Keyboard { name, .. } => *name,
            Self::Touch { name, .. } => *name,
            Self::File { name, .. } => *name,
        }
    }

    pub fn set_name(&mut self, new_name: EventName) {
        match self {
            Self::Mouse { name, .. } => *name = new_name,
            Self::Wheel { name, .. } => *name = new_name,
            Self::Keyboard { name, .. } => *name = new_name,
            Self::Touch { name, .. } => *name = new_name,
            Self::File { name, .. } => *name = new_name,
        }
    }
}
