use keyboard_types::{
    Code,
    Key,
    Modifiers,
};
use torin::prelude::CursorPoint;

use crate::{
    events::{
        data::{
            MouseButton,
            WheelSource,
        },
        name::EventName,
    },
    prelude::{
        Force,
        TouchPhase,
    },
};

#[derive(Clone, Debug, PartialEq, Copy, Eq, Hash)]
pub enum MouseEventName {
    MouseUp,
    MouseDown,
    MouseMove,
}

impl From<MouseEventName> for EventName {
    fn from(value: MouseEventName) -> Self {
        match value {
            MouseEventName::MouseUp => EventName::MouseUp,
            MouseEventName::MouseMove => EventName::MouseMove,
            MouseEventName::MouseDown => EventName::MouseDown,
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

/// Data for [PlatformEvent].
#[derive(Clone, Debug, PartialEq)]
pub enum PlatformEvent {
    /// A Mouse Event.
    Mouse {
        name: MouseEventName,
        cursor: CursorPoint,
        button: Option<MouseButton>,
    },
    /// A Keyboard Event.
    Keyboard {
        name: KeyboardEventName,
        key: Key,
        code: Code,
        modifiers: Modifiers,
    },
    /// A Wheel Event.
    Wheel {
        name: WheelEventName,
        scroll: CursorPoint,
        cursor: CursorPoint,
        source: WheelSource,
    },
    /// A Touch Event.
    Touch {
        name: TouchEventName,
        location: CursorPoint,
        finger_id: u64,
        phase: TouchPhase,
        force: Option<Force>,
    },
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

impl ragnarok::SourceEvent for PlatformEvent {
    type Name = EventName;

    /// Check if the event means the cursor was moved.
    fn is_moved(&self) -> bool {
        matches!(
            &self,
            Self::Mouse {
                name: MouseEventName::MouseMove,
                ..
            }
        )
    }

    /// Check if this event can press state of a Node.
    fn is_pressed(&self) -> bool {
        matches!(
            &self,
            Self::Mouse {
                name: MouseEventName::MouseDown,
                ..
            }
        )
    }

    fn as_event_name(&self) -> EventName {
        match self {
            Self::Mouse { name, .. } => (*name).into(),
            Self::Keyboard { name, .. } => (*name).into(),
            Self::Wheel { name, .. } => (*name).into(),
            Self::Touch { name, .. } => (*name).into(),
        }
    }

    fn try_location(&self) -> Option<ragnarok::CursorPoint> {
        match self {
            PlatformEvent::Mouse { cursor, .. } => Some(*cursor),
            PlatformEvent::Wheel { cursor, .. } => Some(*cursor),
            PlatformEvent::Touch { location, .. } => Some(*location),
            _ => None,
        }
    }
}
