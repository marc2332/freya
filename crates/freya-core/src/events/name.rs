#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum EventName {
    // Platform Mouse
    MouseUp,
    MouseDown,
    MouseMove,

    // Platform Mouse or Touch
    PointerPress,
    PointerDown,
    PointerMove,
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
    GlobalPointerUp,
    GlobalPointerDown,

    GlobalKeyDown,
    GlobalKeyUp,

    CaptureGlobalPointerMove,
    CaptureGlobalPointerUp,

    Wheel,

    Sized,

    Visible,

    Hidden,

    Styled,

    DragEnter,
    DragOver,
    DragLeave,
    Drop,

    ImePreedit,

    AccessibilityAction,
}

use std::{
    cmp::Ordering,
    collections::HashSet,
};

use ragnarok::NameOfEvent as _;

impl PartialOrd for EventName {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EventName {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.priority(), *self as usize).cmp(&(other.priority(), *other as usize))
    }
}

impl EventName {
    fn priority(&self) -> u8 {
        match self {
            // Capture events have max priority.
            event if event.is_capture() => 0,
            // Exclusive left events precede non-exclusive left events.
            event if event.is_exclusive_left() => 1,
            // Left events have priority over non-left events.
            event if event.is_left() => 2,
            // Over events have priority over enter events.
            event if event.is_non_exclusive_enter() => 3,
            // Non-capture globals fire last.
            event if event.is_global() => 5,
            _ => 4,
        }
    }

    /// Check if this even captures others or not
    pub fn is_capture(&self) -> bool {
        matches!(
            &self,
            Self::CaptureGlobalPointerMove | Self::CaptureGlobalPointerUp
        )
    }

    /// Check if this is a global pointer event
    pub fn is_global_pointer(&self) -> bool {
        matches!(
            self,
            Self::GlobalPointerMove
                | Self::GlobalPointerUp
                | Self::GlobalPointerDown
                | Self::CaptureGlobalPointerMove
                | Self::CaptureGlobalPointerUp
        )
    }

    pub fn is_left(&self) -> bool {
        matches!(&self, Self::PointerLeave | Self::PointerOut) || self.is_drag_leave()
    }

    pub fn is_exclusive_left(&self) -> bool {
        matches!(&self, Self::PointerLeave)
    }

    pub fn is_non_exclusive_enter(&self) -> bool {
        matches!(&self, Self::PointerOver) || self.is_drag_enter()
    }

    pub fn is_down(&self) -> bool {
        matches!(self, Self::PointerDown)
    }

    pub fn is_pointer_move(&self) -> bool {
        matches!(self, Self::PointerMove)
    }

    pub fn is_press(&self) -> bool {
        matches!(self, Self::PointerPress)
    }
}

impl ragnarok::NameOfEvent for EventName {
    fn get_global_events(&self) -> HashSet<Self> {
        match self {
            Self::MouseUp | Self::TouchEnd => {
                HashSet::from([Self::GlobalPointerUp, Self::CaptureGlobalPointerUp])
            }
            Self::MouseDown | Self::TouchStart => HashSet::from([Self::GlobalPointerDown]),
            Self::MouseMove | Self::TouchMove => {
                HashSet::from([Self::GlobalPointerMove, Self::CaptureGlobalPointerMove])
            }

            Self::KeyDown => HashSet::from([Self::GlobalKeyDown]),
            Self::KeyUp => HashSet::from([Self::GlobalKeyUp]),

            _ => HashSet::new(),
        }
    }

    fn get_derived_events(&self) -> HashSet<Self> {
        let mut events = HashSet::new();

        events.insert(*self);

        match self {
            Self::MouseMove | Self::TouchMove => {
                events.insert(Self::PointerMove);
                events.insert(Self::PointerEnter);
                events.insert(Self::PointerOver);
            }
            Self::TouchStart => {
                events.insert(Self::PointerDown);
                events.insert(Self::PointerEnter);
                events.insert(Self::PointerOver);
            }
            Self::MouseDown => {
                events.insert(Self::PointerDown);
            }
            Self::MouseUp | Self::TouchEnd => {
                events.insert(Self::PointerPress);
            }
            Self::PointerOut => {
                events.insert(Self::PointerLeave);
            }
            Self::DragOver => events.extend(Self::new_drag_enter()),
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
                events.extend([Self::PointerPress, Self::GlobalPointerUp])
            }
            Self::PointerPress => events.extend([Self::MouseUp, Self::GlobalPointerUp]),
            Self::MouseDown | Self::TouchStart => {
                events.extend([Self::PointerDown, Self::GlobalPointerDown])
            }
            Self::PointerDown => events.extend([Self::MouseDown, Self::GlobalPointerDown]),
            Self::CaptureGlobalPointerMove => {
                events.extend([
                    Self::MouseMove,
                    Self::TouchMove,
                    Self::PointerMove,
                    Self::PointerEnter,
                    Self::PointerOver,
                    Self::GlobalPointerMove,
                ]);
            }
            Self::CaptureGlobalPointerUp => {
                events.extend([
                    Self::MouseUp,
                    Self::TouchEnd,
                    Self::PointerPress,
                    Self::GlobalPointerUp,
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
                | Self::GlobalPointerUp
                | Self::GlobalPointerDown
                | Self::GlobalPointerMove
        )
    }

    fn is_moved(&self) -> bool {
        matches!(
            &self,
            Self::MouseMove
                | Self::TouchMove
                | Self::PointerMove
                | Self::CaptureGlobalPointerMove
                | Self::GlobalPointerMove
        )
    }

    fn does_bubble(&self) -> bool {
        !matches!(self, Self::DragEnter | Self::DragLeave)
            && !self.is_moved()
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
        matches!(self, Self::MouseDown | Self::PointerDown | Self::TouchStart)
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

    fn is_drag_enter(&self) -> bool {
        matches!(self, Self::DragEnter)
    }

    fn is_drag_over(&self) -> bool {
        matches!(self, Self::DragOver)
    }

    fn is_drag_leave(&self) -> bool {
        matches!(self, Self::DragLeave)
    }

    fn is_drop(&self) -> bool {
        matches!(self, Self::Drop)
    }

    fn new_drag_enter() -> Option<Self> {
        Some(Self::DragEnter)
    }

    fn new_drag_leave() -> Option<Self> {
        Some(Self::DragLeave)
    }
}
