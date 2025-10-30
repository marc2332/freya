#[derive(Clone, Copy, PartialEq, Debug, Hash)]
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

    // Platform Keyboard
    KeyDown,
    KeyUp,

    // Platform Touch
    TouchCancel,
    TouchStart,
    TouchMove,
    TouchEnd,

    GlobalMouseMove,
    GlobalMouseUp,
    GlobalMouseDown,

    GlobalKeyDown,
    GlobalKeyUp,

    CaptureGlobalMouseMove,
    CaptureGlobalMouseUp,

    Wheel,

    Sized,
}

impl Eq for EventName {}

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
            Self::CaptureGlobalMouseMove | Self::CaptureGlobalMouseUp
        )
    }

    pub fn is_left(&self) -> bool {
        matches!(&self, Self::PointerLeave)
    }

    pub fn is_down(&self) -> bool {
        matches!(self, Self::PointerDown)
    }

    pub fn is_press(&self) -> bool {
        matches!(self, Self::PointerPress)
    }
}

impl ragnarok::NameOfEvent for EventName {
    fn get_global_events(&self) -> Vec<Self> {
        let mut events = Vec::new();
        match self {
            Self::MouseUp => events.extend([Self::GlobalMouseUp, Self::CaptureGlobalMouseUp]),
            Self::MouseDown => events.push(Self::GlobalMouseDown),
            Self::MouseMove => events.extend([Self::GlobalMouseMove, Self::CaptureGlobalMouseMove]),

            Self::KeyDown => events.push(Self::GlobalKeyDown),
            Self::KeyUp => events.push(Self::GlobalKeyUp),
            _ => {}
        }
        events
    }

    fn get_derived_events(&self) -> Vec<Self> {
        let mut events = Vec::new();

        events.push(*self);

        match self {
            Self::MouseMove | Self::TouchMove => events.extend([Self::PointerEnter]),
            Self::MouseDown | Self::TouchStart => events.extend([Self::PointerDown]),
            Self::MouseUp | Self::TouchEnd => events.extend([Self::PointerPress]),
            _ => {}
        }

        events
    }

    fn get_cancellable_events(&self) -> Vec<Self> {
        let mut events = Vec::new();

        events.push(*self);

        match self {
            Self::KeyDown => events.extend([Self::GlobalKeyDown]),
            Self::KeyUp => events.extend([Self::GlobalKeyUp]),

            Self::MouseUp | Self::PointerPress => events.extend([Self::GlobalMouseUp]),
            Self::MouseDown | Self::PointerDown => events.extend([Self::GlobalMouseDown]),

            Self::CaptureGlobalMouseMove => {
                events.extend([Self::MouseMove, Self::PointerEnter, Self::GlobalMouseMove])
            }
            Self::CaptureGlobalMouseUp => {
                events.extend([Self::MouseUp, Self::PointerPress, Self::GlobalMouseUp])
            }

            _ => {}
        }

        events
    }

    fn is_global(&self) -> bool {
        matches!(
            self,
            Self::GlobalKeyDown | Self::GlobalKeyUp | Self::GlobalMouseUp | Self::GlobalMouseMove
        )
    }

    fn is_moved(&self) -> bool {
        matches!(
            &self,
            Self::MouseMove
                | Self::TouchMove
                | Self::CaptureGlobalMouseMove
                | Self::GlobalMouseMove
        )
    }

    fn does_bubble(&self) -> bool {
        !self.is_moved()
            && !self.is_enter()
            && !self.is_left()
            && !self.is_global()
            && !self.is_capture()
    }

    fn does_go_through_solid(&self) -> bool {
        // TODO
        false
    }

    fn is_enter(&self) -> bool {
        matches!(&self, Self::PointerEnter)
    }

    fn is_pressed(&self) -> bool {
        matches!(self, Self::MouseDown | Self::PointerDown)
    }

    fn is_released(&self) -> bool {
        matches!(&self, Self::PointerPress)
    }

    fn new_leave() -> Self {
        Self::PointerLeave
    }
}
