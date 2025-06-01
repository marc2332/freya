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
#[derive(Clone, Debug, PartialEq)]
pub struct PlatformEvent {
    pub platform_name: PlatformEventName,
    pub platform_data: PlatformEventData,
}

/// Name for [PlatformEvent].
#[derive(Clone, Debug, PartialEq, Copy, Eq, Hash)]
pub enum PlatformEventName {
    MouseMove,
    MouseDown,
    MouseUp,

    Click,
    MiddleClick,
    RightClick,

    Wheel,

    KeyDown,
    KeyUp,

    TouchStart,
    TouchMove,
    TouchEnd,
    TouchCancel,

    FileDrop,
    FileHover,
    FileHoverCancelled,
}

impl From<PlatformEventName> for EventName {
    fn from(value: PlatformEventName) -> Self {
        match value {
            PlatformEventName::MouseMove => EventName::MouseMove,
            PlatformEventName::MouseDown => EventName::MouseDown,
            PlatformEventName::MouseUp => EventName::MouseUp,
            PlatformEventName::Click => EventName::Click,
            PlatformEventName::MiddleClick => EventName::MiddleClick,
            PlatformEventName::RightClick => EventName::RightClick,
            PlatformEventName::Wheel => EventName::Wheel,
            PlatformEventName::KeyDown => EventName::KeyDown,
            PlatformEventName::KeyUp => EventName::KeyUp,
            PlatformEventName::TouchStart => EventName::TouchStart,
            PlatformEventName::TouchMove => EventName::TouchMove,
            PlatformEventName::TouchEnd => EventName::TouchEnd,
            PlatformEventName::TouchCancel => EventName::TouchCancel,
            PlatformEventName::FileDrop => EventName::FileDrop,
            PlatformEventName::FileHover => EventName::GlobalFileHover,
            PlatformEventName::FileHoverCancelled => EventName::GlobalFileHoverCancelled,
        }
    }
}

impl PlatformEventName {
    /// Check if the event means the cursor was moved.
    pub fn is_moved(&self) -> bool {
        matches!(&self, Self::MouseMove)
    }

    /// Check if this event can press state of a Node.
    pub fn is_pressed(&self) -> bool {
        matches!(self, Self::MouseDown | Self::TouchStart)
    }
}

/// Data for [PlatformEvent].
#[derive(Clone, Debug, PartialEq)]
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
