#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum EventName {
    // Platform Mouse
    MouseUp,
    MouseDown,
    MouseMove,

    // Platform Mouse or Touch
    PointerPress,
    PointerDown,
    PointerEnter,
    PointerLeave,
    PointerOver,
    PointerOut,

    // Platform Keyboard
    KeyDown,
    KeyUp,

    // Platform Touch
    TouchCancel,
    TouchStart,
    TouchMove,
    TouchEnd,

    GlobalPointerMove,
    GlobalPointerPress,
    GlobalPointerDown,

    GlobalKeyDown,
    GlobalKeyUp,

    GlobalFileHover,
    GlobalFileHoverCancelled,

    CaptureGlobalPointerMove,
    CaptureGlobalPointerPress,

    Wheel,

    Sized,

    FileDrop,

    ImePreedit,
}

use std::collections::HashSet;

impl PartialOrd for EventName {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EventName {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            // Capture events have max priority
            e if e.is_capture() => std::cmp::Ordering::Less,
            // Left events have more priority over non-left
            e if e.is_left() => std::cmp::Ordering::Less,
            // Exclusive left events have more priority over non-exclusive-left
            e if e.is_exclusive_left() => std::cmp::Ordering::Less,
            // Over events have priority over enter events
            e if e.is_non_exclusive_enter() => std::cmp::Ordering::Less,
            e => {
                if e == other {
                    std::cmp::Ordering::Equal
                } else {
                    std::cmp::Ordering::Greater
                }
            }
        }
    }
}

impl EventName {
    /// Check if this even captures others or not
    pub fn is_capture(&self) -> bool {
        matches!(
            &self,
            Self::CaptureGlobalPointerMove | Self::CaptureGlobalPointerPress
        )
    }

    /// Check if this is a global pointer event
    pub fn is_global_pointer(&self) -> bool {
        matches!(
            self,
            Self::GlobalPointerMove
                | Self::GlobalPointerPress
                | Self::GlobalPointerDown
                | Self::CaptureGlobalPointerMove
                | Self::CaptureGlobalPointerPress
        )
    }

    pub fn is_left(&self) -> bool {
        matches!(&self, Self::PointerLeave | Self::PointerOut)
    }

    pub fn is_exclusive_left(&self) -> bool {
        matches!(&self, Self::PointerLeave)
    }

    pub fn is_non_exclusive_enter(&self) -> bool {
        matches!(&self, Self::PointerOver)
    }

    pub fn is_down(&self) -> bool {
        matches!(self, Self::PointerDown)
    }

    pub fn is_press(&self) -> bool {
        matches!(self, Self::PointerPress)
    }
}

impl ragnarok::NameOfEvent for EventName {
    fn get_global_events(&self) -> HashSet<Self> {
        match self {
            Self::MouseUp | Self::TouchEnd => {
                HashSet::from([Self::GlobalPointerPress, Self::CaptureGlobalPointerPress])
            }
            Self::MouseDown | Self::TouchStart => HashSet::from([Self::GlobalPointerDown]),
            Self::MouseMove | Self::TouchMove => {
                HashSet::from([Self::GlobalPointerMove, Self::CaptureGlobalPointerMove])
            }

            Self::KeyDown => HashSet::from([Self::GlobalKeyDown]),
            Self::KeyUp => HashSet::from([Self::GlobalKeyUp]),

            Self::GlobalFileHover => HashSet::from([Self::GlobalFileHover]),
            Self::GlobalFileHoverCancelled => HashSet::from([Self::GlobalFileHoverCancelled]),
            _ => HashSet::new(),
        }
    }

    fn get_derived_events(&self) -> HashSet<Self> {
        let mut events = HashSet::new();

        events.insert(*self);

        match self {
            Self::MouseMove | Self::TouchMove => {
                events.insert(Self::PointerEnter);
                events.insert(Self::PointerOver);
            }
            Self::MouseDown | Self::TouchStart => {
                events.insert(Self::PointerDown);
            }
            Self::MouseUp | Self::TouchEnd => {
                events.insert(Self::PointerPress);
            }
            // PointerOut is synthesized as the leave event; it also derives PointerLeave
            // so that both events are emitted when a node stops being hovered.
            Self::PointerOut => {
                events.insert(Self::PointerLeave);
            }
            _ => {}
        }

        events
    }

    fn get_cancellable_events(&self) -> HashSet<Self> {
        let mut events = HashSet::new();

        events.insert(*self);

        match self {
            Self::KeyDown => {
                events.insert(Self::GlobalKeyDown);
            }
            Self::KeyUp => {
                events.insert(Self::GlobalKeyUp);
            }
            Self::MouseUp | Self::TouchEnd => {
                events.extend([Self::PointerPress, Self::GlobalPointerPress])
            }
            Self::PointerPress => events.extend([Self::MouseUp, Self::GlobalPointerPress]),
            Self::MouseDown | Self::TouchStart => {
                events.extend([Self::PointerDown, Self::GlobalPointerDown])
            }
            Self::PointerDown => events.extend([Self::MouseDown, Self::GlobalPointerDown]),
            Self::CaptureGlobalPointerMove => {
                events.extend([
                    Self::MouseMove,
                    Self::TouchMove,
                    Self::PointerEnter,
                    Self::PointerOver,
                    Self::GlobalPointerMove,
                ]);
            }
            Self::CaptureGlobalPointerPress => {
                events.extend([
                    Self::MouseUp,
                    Self::TouchEnd,
                    Self::PointerPress,
                    Self::GlobalPointerPress,
                ]);
            }

            _ => {}
        }

        events
    }

    fn is_global(&self) -> bool {
        matches!(
            self,
            Self::GlobalKeyDown
                | Self::GlobalKeyUp
                | Self::GlobalPointerPress
                | Self::GlobalPointerDown
                | Self::GlobalPointerMove
                | Self::GlobalFileHover
                | Self::GlobalFileHoverCancelled
        )
    }

    fn is_moved(&self) -> bool {
        matches!(
            &self,
            Self::MouseMove
                | Self::TouchMove
                | Self::CaptureGlobalPointerMove
                | Self::GlobalPointerMove
        )
    }

    fn does_bubble(&self) -> bool {
        !self.is_moved()
            && !self.is_enter()
            && !self.is_left()
            && !self.is_global()
            && !self.is_capture()
    }

    fn is_emitted_once(&self) -> bool {
        self.does_bubble() || self.is_exclusive_enter() || self.is_exclusive_leave()
    }

    fn does_go_through_solid(&self) -> bool {
        // TODO
        false
    }

    fn is_enter(&self) -> bool {
        matches!(&self, Self::PointerEnter | Self::PointerOver)
    }

    fn is_pressed(&self) -> bool {
        matches!(self, Self::MouseDown | Self::PointerDown)
    }

    fn is_released(&self) -> bool {
        matches!(&self, Self::PointerPress)
    }

    fn is_exclusive_enter(&self) -> bool {
        matches!(&self, Self::PointerEnter)
    }

    fn is_exclusive_leave(&self) -> bool {
        matches!(&self, Self::PointerLeave)
    }

    fn new_leave() -> Self {
        Self::PointerOut
    }

    fn new_exclusive_leave() -> Self {
        Self::PointerLeave
    }

    fn new_exclusive_enter() -> Self {
        Self::PointerEnter
    }
}
