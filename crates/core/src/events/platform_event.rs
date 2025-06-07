use std::path::PathBuf;

use freya_elements::{
    events::keyboard::{
        Code,
        Key,
        Modifiers,
    },
};
use freya_native_core::events::EventName;
use torin::prelude::*;
use winit::event::{
    Force,
    MouseButton,
    TouchPhase,
};

#[derive(Clone, Debug, PartialEq, Copy, Eq, Hash)]
pub enum MouseEventName {
    MouseMove,
    MouseDown,
    MouseUp,

    Click,
    MiddleClick,
    RightClick,
}

impl From<MouseEventName> for EventName {
    fn from(value: MouseEventName) -> Self {
        match value {
            MouseEventName::MouseMove => EventName::MouseMove,
            MouseEventName::MouseDown => EventName::MouseDown,
            MouseEventName::MouseUp => EventName::MouseUp,
            MouseEventName::Click => EventName::Click,
            MouseEventName::MiddleClick => EventName::MiddleClick,
            MouseEventName::RightClick => EventName::RightClick,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Copy, Eq, Hash)]
pub enum WheelEventName {
    Wheel,
}

impl From<WheelEventName> for EventName {
    fn from(value: WheelEventName) -> Self {
        match value {
            WheelEventName::Wheel => EventName::Wheel,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Copy, Eq, Hash)]
pub enum KeyboardEventName {
    KeyDown,
    KeyUp,
}

impl From<KeyboardEventName> for EventName {
    fn from(value: KeyboardEventName) -> Self {
        match value {
            KeyboardEventName::KeyDown => EventName::KeyDown,
            KeyboardEventName::KeyUp => EventName::KeyUp,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Copy, Eq, Hash)]
pub enum TouchEventName {
    TouchStart,
    TouchMove,
    TouchEnd,
    TouchCancel,
}

impl From<TouchEventName> for EventName {
    fn from(value: TouchEventName) -> Self {
        match value {
            TouchEventName::TouchStart => EventName::TouchStart,
            TouchEventName::TouchMove => EventName::TouchMove,
            TouchEventName::TouchEnd => EventName::TouchEnd,
            TouchEventName::TouchCancel => EventName::TouchCancel,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Copy, Eq, Hash)]
pub enum FileEventName {
    FileDrop,
    FileHover,
    FileHoverCancelled,
}
impl From<FileEventName> for EventName {
    fn from(value: FileEventName) -> Self {
        match value {
            FileEventName::FileDrop => EventName::FileDrop,
            FileEventName::FileHover => EventName::GlobalFileHover,
            FileEventName::FileHoverCancelled => EventName::GlobalFileHoverCancelled,
        }
    }
}

impl PlatformEvent {
    /// Check if the event means the cursor was moved.
    pub fn is_moved(&self) -> bool {
        matches!(
            &self,
            Self::Mouse {
                name: MouseEventName::MouseMove,
                ..
            }
        )
    }

    /// Check if this event can press state of a Node.
    pub fn is_pressed(&self) -> bool {
        matches!(
            &self,
            Self::Mouse {
                name: MouseEventName::MouseDown,
                ..
            } | Self::Touch {
                name: TouchEventName::TouchStart,
                ..
            }
        )
    }

    pub fn event_name(&self) -> EventName {
        match self {
            Self::Mouse { name, .. } => (*name).into(),
            Self::Wheel { name, .. } => (*name).into(),
            Self::Keyboard { name, .. } => (*name).into(),
            Self::Touch { name, .. } => (*name).into(),
            Self::File { name, .. } => (*name).into(),
        }
    }
}

/// Data for [PlatformEvent].
#[derive(Clone, Debug, PartialEq)]
pub enum PlatformEvent {
    /// A Mouse Event.
    Mouse {
        name: MouseEventName,
        cursor: CursorPoint,
        button: Option<MouseButton>,
    },
    /// A Wheel event.
    Wheel {
        name: WheelEventName,
        scroll: CursorPoint,
        cursor: CursorPoint,
    },
    /// A Keyboard event.
    Keyboard {
        name: KeyboardEventName,
        key: Key,
        code: Code,
        modifiers: Modifiers,
    },
    /// A Touch event.
    Touch {
        name: TouchEventName,
        location: CursorPoint,
        finger_id: u64,
        phase: TouchPhase,
        force: Option<Force>,
    },
    /// A File event.
    File {
        name: FileEventName,
        cursor: CursorPoint,
        file_path: Option<PathBuf>,
    },
}
