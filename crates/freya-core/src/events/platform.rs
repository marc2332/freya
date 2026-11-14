use keyboard_types::{
    Code,
    Key,
    Modifiers,
};
use torin::prelude::CursorPoint;

use crate::{
    events::{
        data::{
            DragDecision,
            MouseButton,
            WheelSource,
        },
        name::EventName,
    },
    native_drag::DragDataHandle,
    node_id::NodeId,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DragEventName {
    Over,
    Leave,
    Drop,
}

impl From<DragEventName> for EventName {
    fn from(value: DragEventName) -> Self {
        match value {
            DragEventName::Over => EventName::DragOver,
            DragEventName::Leave => EventName::DragLeave,
            DragEventName::Drop => EventName::Drop,
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
    /// The pointer exited the window at its last known position.
    PointerExit { cursor: CursorPoint },
    /// Ends a pointer gesture without clicking an element.
    PointerCancel { cursor: CursorPoint },
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
        phase: TouchPhase,
    },
    /// A Touch Event.
    Touch {
        name: TouchEventName,
        location: CursorPoint,
        finger_id: u64,
        phase: TouchPhase,
        force: Option<Force>,
    },
    /// An IME event.
    ImePreedit {
        name: ImeEventName,
        text: String,
        cursor: Option<(usize, usize)>,
    },
    /// A native data transfer, independent of pointer hover state.
    Drag {
        name: DragEventName,
        cursor: CursorPoint,
        data: DragDataHandle,
        decision: Option<DragDecision>,
        target: Option<NodeId>,
    },
}

#[derive(Clone, Debug, PartialEq, Copy, Eq, Hash)]
pub enum ImeEventName {
    Preedit,
}

impl From<ImeEventName> for EventName {
    fn from(value: ImeEventName) -> Self {
        match value {
            ImeEventName::Preedit => EventName::ImePreedit,
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

impl ragnarok::SourceEvent for PlatformEvent {
    type Name = EventName;
    type Key = NodeId;

    fn drop_target(&self) -> Option<NodeId> {
        match self {
            Self::Drag { target, .. } => *target,
            _ => None,
        }
    }

    fn can_coalesce_with(&self, next: &Self) -> bool {
        match (self, next) {
            (
                Self::Mouse {
                    name: MouseEventName::MouseMove,
                    button: previous,
                    ..
                },
                Self::Mouse {
                    name: MouseEventName::MouseMove,
                    button,
                    ..
                },
            ) => previous == button,
            (
                Self::Wheel {
                    scroll: previous,
                    cursor: previous_cursor,
                    source: WheelSource::Line,
                    phase: TouchPhase::Moved,
                    ..
                },
                Self::Wheel {
                    scroll,
                    cursor,
                    source: WheelSource::Line,
                    phase: TouchPhase::Moved,
                    ..
                },
            ) => {
                previous_cursor == cursor
                    && previous.x.partial_cmp(&0.0) == scroll.x.partial_cmp(&0.0)
                    && previous.y.partial_cmp(&0.0) == scroll.y.partial_cmp(&0.0)
                    && (previous.x + scroll.x).is_finite()
                    && (previous.y + scroll.y).is_finite()
            }
            _ => false,
        }
    }

    fn coalesce_with(&mut self, next: Self) {
        match (self, next) {
            (Self::Wheel { scroll, .. }, Self::Wheel { scroll: next, .. }) => {
                scroll.x += next.x;
                scroll.y += next.y;
            }
            (event, next) => *event = next,
        }
    }

    /// Check if the event means the cursor was moved.
    fn is_moved(&self) -> bool {
        matches!(
            &self,
            Self::Mouse {
                name: MouseEventName::MouseMove,
                ..
            } | Self::Touch {
                phase: TouchPhase::Moved,
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
            } | Self::Touch {
                name: TouchEventName::TouchStart,
                ..
            }
        )
    }

    fn is_pointer_exit(&self) -> bool {
        matches!(self, Self::PointerExit { .. })
    }

    fn is_pointer_cancelled(&self) -> bool {
        matches!(self, Self::PointerCancel { .. })
    }

    /// Check if this event ends or cancels a touch.
    fn is_touch_released(&self) -> bool {
        matches!(
            &self,
            Self::Touch {
                phase: TouchPhase::Ended | TouchPhase::Cancelled,
                ..
            }
        )
    }

    fn as_event_name(&self) -> EventName {
        match self {
            Self::Mouse { name, .. } => (*name).into(),
            Self::PointerExit { .. } => EventName::MouseMove,
            Self::PointerCancel { .. } => EventName::MouseUp,
            Self::Keyboard { name, .. } => (*name).into(),
            Self::Wheel { name, .. } => (*name).into(),
            Self::ImePreedit { name, .. } => (*name).into(),
            Self::Touch { name, .. } => (*name).into(),
            Self::Drag { name, .. } => (*name).into(),
        }
    }

    fn try_location(&self) -> Option<ragnarok::CursorPoint> {
        match self {
            PlatformEvent::Mouse { cursor, .. } => Some(*cursor),
            PlatformEvent::Wheel { cursor, .. } => Some(*cursor),
            PlatformEvent::Touch { location, .. } => Some(*location),
            PlatformEvent::Drag { cursor, .. } => Some(*cursor),
            _ => None,
        }
    }
}
