use std::path::PathBuf;

use freya_elements::events::keyboard::{
    Code,
    Key,
    Modifiers,
};
use freya_native_core::events::EventName;
use torin::prelude::*;
use winit::event::{
    Force,
    MouseButton,
    TouchPhase,
};

/// Events emitted by a Freya platform, such as desktop or freya-testing.
#[derive(Clone, Debug)]
pub struct PlatformEvent {
    pub name: EventName,
    pub data: PlatformEventData,
}

/// Data for [PlatformEvent].
#[derive(Clone, Debug)]
pub enum PlatformEventData {
    /// A Mouse Event.
    Mouse {
        cursor: CursorPoint,
        button: Option<MouseButton>,
    },
    /// A Wheel event.
    Wheel {
        scroll: CursorPoint,
        cursor: CursorPoint,
    },
    /// A Keyboard event.
    Keyboard {
        key: Key,
        code: Code,
        modifiers: Modifiers,
    },
    /// A Touch event.
    Touch {
        location: CursorPoint,
        finger_id: u64,
        phase: TouchPhase,
        force: Option<Force>,
    },
    /// A File event.
    File {
        cursor: CursorPoint,
        file_path: Option<PathBuf>,
    },
}
