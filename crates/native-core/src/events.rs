use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Debug, Hash)]
pub enum EventName {
    Click,
    MiddleClick,
    RightClick,

    MouseUp,
    MouseDown,
    MouseMove,
    MouseEnter,
    MouseLeave,

    Wheel,

    PointerOver,
    PointerDown,
    PointerEnter,
    PointerLeave,
    PointerUp,
    PointerPress,

    KeyDown,
    KeyUp,
    GlobalKeyDown,
    GlobalKeyUp,

    TouchCancel,
    TouchStart,
    TouchMove,
    TouchEnd,

    GlobalClick,
    GlobalPointerUp,
    GlobalMouseDown,
    GlobalMouseMove,
    GlobalFileHover,
    GlobalFileHoverCancelled,

    CaptureGlobalMouseMove,
    CaptureGlobalPointerUp,

    FileDrop,
}

impl FromStr for EventName {
    type Err = ();

    fn from_str(txt: &str) -> Result<Self, Self::Err> {
        match txt {
            "click" => Ok(EventName::Click),
            "rightclick" => Ok(EventName::RightClick),
            "middleclick" => Ok(EventName::MiddleClick),
            "mouseup" => Ok(EventName::MouseUp),
            "mousedown" => Ok(EventName::MouseDown),
            "mousemove" => Ok(EventName::MouseMove),
            "mouseenter" => Ok(EventName::MouseEnter),
            "mouseleave" => Ok(EventName::MouseLeave),
            "wheel" => Ok(EventName::Wheel),
            "pointermove" => Ok(EventName::PointerOver),
            "pointerdown" => Ok(EventName::PointerDown),
            "pointerenter" => Ok(EventName::PointerEnter),
            "pointerleave" => Ok(EventName::PointerLeave),
            "pointerup" => Ok(EventName::PointerUp),
            "pointerpress" => Ok(EventName::PointerPress),
            "keydown" => Ok(EventName::KeyDown),
            "keyup" => Ok(EventName::KeyUp),
            "globalkeydown" => Ok(EventName::GlobalKeyDown),
            "globalkeyup" => Ok(EventName::GlobalKeyUp),
            "touchcancel" => Ok(EventName::TouchCancel),
            "touchstart" => Ok(EventName::TouchStart),
            "touchmove" => Ok(EventName::TouchMove),
            "touchend" => Ok(EventName::TouchEnd),
            "globalclick" => Ok(EventName::GlobalClick),
            "globalpointerup" => Ok(EventName::GlobalPointerUp),
            "globalmousedown" => Ok(EventName::GlobalMouseDown),
            "globalmousemove" => Ok(EventName::GlobalMouseMove),
            "filedrop" => Ok(EventName::FileDrop),
            "globalfilehover" => Ok(EventName::GlobalFileHover),
            "globalfilehovercancelled" => Ok(EventName::GlobalFileHoverCancelled),
            "captureglobalmousemove" => Ok(EventName::CaptureGlobalMouseMove),
            "captureglobalpointerup" => Ok(EventName::CaptureGlobalPointerUp),
            _ => Err(()),
        }
    }
}

impl From<EventName> for &str {
    fn from(event: EventName) -> Self {
        match event {
            EventName::Click => "click",
            EventName::MiddleClick => "middleclick",
            EventName::RightClick => "rightclick",
            EventName::MouseUp => "mouseup",
            EventName::MouseDown => "mousedown",
            EventName::MouseMove => "mousemove",
            EventName::MouseEnter => "mouseenter",
            EventName::MouseLeave => "mouseleave",
            EventName::Wheel => "wheel",
            EventName::PointerOver => "pointermove",
            EventName::PointerDown => "pointerdown",
            EventName::PointerEnter => "pointerenter",
            EventName::PointerLeave => "pointerleave",
            EventName::PointerUp => "pointerup",
            EventName::PointerPress => "pointerpress",
            EventName::KeyUp => "keyup",
            EventName::KeyDown => "keydown",
            EventName::GlobalKeyDown => "globalkeydown",
            EventName::GlobalKeyUp => "globalkeyup",
            EventName::TouchCancel => "touchcancel",
            EventName::TouchStart => "touchstart",
            EventName::TouchMove => "touchmove",
            EventName::TouchEnd => "touchend",
            EventName::GlobalClick => "globalclick",
            EventName::GlobalPointerUp => "globalpointerup",
            EventName::GlobalMouseDown => "globalmousedown",
            EventName::GlobalMouseMove => "globalmousemove",
            EventName::FileDrop => "filedrop",
            EventName::GlobalFileHover => "globalfilehover",
            EventName::GlobalFileHoverCancelled => "globalfilehovercancelled",
            EventName::CaptureGlobalMouseMove => "captureglobalmousemove",
            EventName::CaptureGlobalPointerUp => "captureglobalpointerup",
        }
    }
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
            // Pointer events more priority over non-pointer
            e if e.is_pointer() && !other.is_pointer() => std::cmp::Ordering::Less,
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
    /// Check if it's one of the Pointer variants
    pub fn is_capture(&self) -> bool {
        matches!(
            &self,
            Self::CaptureGlobalMouseMove | Self::CaptureGlobalPointerUp
        )
    }

    /// Check if it's one of the Pointer variants
    pub fn is_pointer(&self) -> bool {
        matches!(
            &self,
            Self::PointerEnter
                | Self::PointerLeave
                | Self::PointerOver
                | Self::PointerDown
                | Self::PointerUp
                | Self::PointerPress
                | Self::GlobalPointerUp
                | Self::CaptureGlobalPointerUp
        )
    }

    /// Check if the event means the cursor has left.
    pub fn is_left(&self) -> bool {
        matches!(&self, Self::MouseLeave | Self::PointerLeave)
    }
}

impl ragnarok::NameOfEvent for EventName {
    fn get_global_events(&self) -> Vec<Self> {
        let mut events = Vec::new();
        match self {
            Self::MouseUp => events.push(Self::GlobalClick),
            Self::PointerUp => events.extend([Self::GlobalPointerUp, Self::CaptureGlobalPointerUp]),
            Self::MouseDown => events.push(Self::GlobalMouseDown),
            Self::MouseMove => events.extend([Self::GlobalMouseMove, Self::CaptureGlobalMouseMove]),
            Self::GlobalFileHover => events.push(Self::GlobalFileHover),
            Self::GlobalFileHoverCancelled => events.push(Self::GlobalFileHoverCancelled),
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
            Self::MouseMove => {
                events.extend([Self::MouseEnter, Self::PointerEnter, Self::PointerOver])
            }
            Self::TouchMove => events.extend([Self::PointerEnter, Self::PointerOver]),
            Self::MouseDown | Self::TouchStart => events.push(Self::PointerDown),
            Self::MouseUp | Self::MiddleClick | Self::RightClick | Self::TouchEnd => {
                events.extend([Self::Click, Self::PointerUp, Self::PointerPress])
            }
            Self::MouseLeave => events.push(Self::PointerLeave),
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

            Self::Click => {
                events.extend([Self::MiddleClick, Self::GlobalClick, Self::GlobalPointerUp])
            }

            Self::PointerUp => events.extend([
                Self::Click,
                Self::MiddleClick,
                Self::GlobalClick,
                Self::GlobalPointerUp,
            ]),
            Self::PointerDown => events.extend([Self::MouseDown, Self::GlobalMouseDown]),
            Self::PointerOver => events.extend([Self::MouseMove, Self::GlobalMouseMove]),

            Self::PointerEnter => events.extend([Self::MouseEnter]),

            Self::CaptureGlobalMouseMove => events.extend([
                Self::MouseMove,
                Self::MouseEnter,
                Self::GlobalMouseMove,
                Self::PointerEnter,
            ]),

            Self::CaptureGlobalPointerUp => events.extend([
                Self::Click,
                Self::GlobalClick,
                Self::PointerUp,
                Self::PointerPress,
                Self::GlobalPointerUp,
            ]),
            _ => {}
        }

        events
    }

    fn is_enter(&self) -> bool {
        matches!(&self, Self::MouseEnter | Self::PointerEnter)
    }

    fn is_global(&self) -> bool {
        matches!(
            &self,
            Self::GlobalKeyDown
                | Self::GlobalKeyUp
                | Self::GlobalClick
                | Self::GlobalPointerUp
                | Self::GlobalMouseDown
                | Self::GlobalMouseMove
                | Self::GlobalFileHover
                | Self::GlobalFileHoverCancelled
        )
    }

    fn is_moved(&self) -> bool {
        matches!(&self, Self::MouseMove | Self::PointerEnter)
    }

    fn does_bubble(&self) -> bool {
        !self.is_moved()
            && !self.is_enter()
            && !self.is_left()
            && !self.is_global()
            && !self.is_capture()
    }

    fn does_go_through_solid(&self) -> bool {
        matches!(
            self,
            Self::KeyDown | Self::KeyUp | Self::GlobalKeyDown | Self::GlobalKeyUp
        )
    }

    fn is_pressed(&self) -> bool {
        matches!(self, Self::MouseDown | Self::TouchStart | Self::PointerDown)
    }

    fn is_released(&self) -> bool {
        matches!(&self, Self::Click | Self::PointerPress)
    }

    fn new_leave() -> Self {
        Self::MouseLeave
    }
}
