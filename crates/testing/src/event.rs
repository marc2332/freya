use std::path::PathBuf;

use freya_core::events::{
    EventName,
    PlatformEvent,
    PlatformEventData,
};
use freya_elements::events::{
    Code,
    Force,
    Key,
    Modifiers,
    MouseButton,
    TouchPhase,
};
use torin::prelude::CursorPoint;

pub enum TestEvent {
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

impl From<TestEvent> for PlatformEvent {
    fn from(val: TestEvent) -> Self {
        let (name, data) = match val {
            TestEvent::File {
                name,
                cursor,
                file_path,
            } => (name, PlatformEventData::File { cursor, file_path }),
            TestEvent::Keyboard {
                name,
                key,
                code,
                modifiers,
            } => (
                name,
                PlatformEventData::Keyboard {
                    key,
                    code,
                    modifiers,
                },
            ),
            TestEvent::Mouse {
                name,
                cursor,
                button,
            } => (name, PlatformEventData::Mouse { cursor, button }),
            TestEvent::Wheel {
                name,
                scroll,
                cursor,
            } => (name, PlatformEventData::Wheel { scroll, cursor }),
            TestEvent::Touch {
                name,
                location,
                finger_id,
                phase,
                force,
            } => (
                name,
                PlatformEventData::Touch {
                    location,
                    finger_id,
                    phase,
                    force,
                },
            ),
        };

        PlatformEvent { name, data }
    }
}
