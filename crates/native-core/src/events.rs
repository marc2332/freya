use std::str::FromStr;

use smallvec::SmallVec;

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
            // Always prioritize leave events before anything else
            Self::MouseLeave | Self::PointerLeave => {
                if self == other {
                    std::cmp::Ordering::Equal
                } else {
                    std::cmp::Ordering::Less
                }
            }

            _ => std::cmp::Ordering::Greater,
        }
    }
}

impl EventName {
    /// Get the equivalent to a global event
    pub fn get_global_event(&self) -> Option<Self> {
        match self {
            Self::MouseUp => Some(Self::GlobalClick),
            Self::PointerUp => Some(Self::GlobalPointerUp),
            Self::MouseDown => Some(Self::GlobalMouseDown),
            Self::MouseMove => Some(Self::GlobalMouseMove),
            Self::GlobalFileHover => Some(Self::GlobalFileHover),
            Self::GlobalFileHoverCancelled => Some(Self::GlobalFileHoverCancelled),
            Self::KeyDown => Some(EventName::GlobalKeyDown),
            Self::KeyUp => Some(EventName::GlobalKeyUp),
            _ => None,
        }
    }

    /// Some events might cause other events, like for example:
    /// A `mousemove` might also trigger a `mouseenter`
    /// A `mousedown` or a `touchdown` might also trigger a `pointerdown`
    pub fn get_derived_events(&self) -> SmallVec<[Self; 4]> {
        let mut events = SmallVec::new();

        events.push(*self);

        match self {
            Self::MouseMove => {
                events.extend([Self::MouseEnter, Self::PointerEnter, Self::PointerOver])
            }
            Self::TouchMove => events.extend([Self::PointerEnter, Self::PointerOver]),
            Self::MouseDown | Self::TouchStart => events.push(Self::PointerDown),
            Self::MouseUp | Self::MiddleClick | Self::RightClick | Self::TouchEnd => {
                events.extend([Self::Click, Self::PointerUp])
            }
            Self::MouseLeave => events.push(Self::PointerLeave),
            _ => {}
        }

        events
    }

    /// Check if the event means that the pointer (e.g. cursor) just entered a Node
    pub fn is_enter(&self) -> bool {
        matches!(&self, Self::MouseEnter | Self::PointerEnter)
    }

    pub fn is_global(&self) -> bool {
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

    /// Check if it's one of the Pointer variants
    pub fn is_pointer(&self) -> bool {
        matches!(
            &self,
            Self::PointerEnter
                | Self::PointerLeave
                | Self::PointerOver
                | Self::PointerDown
                | Self::PointerUp
                | Self::GlobalPointerUp
        )
    }

    /// Check if the event means the cursor was moved.
    pub fn is_moved(&self) -> bool {
        matches!(
            &self,
            Self::MouseMove | Self::MouseEnter | Self::PointerEnter | Self::PointerOver
        )
    }

    /// Check if the event means the cursor has left.
    pub fn is_left(&self) -> bool {
        matches!(&self, Self::MouseLeave | Self::PointerLeave)
    }

    // Bubble all events except:
    // - Global events
    // - Mouse movements events
    pub fn does_bubble(&self) -> bool {
        !self.is_moved() && !self.is_left() && !self.is_global()
    }

    /// Only let events that do not move the mouse, go through solid nodes
    pub fn does_go_through_solid(&self) -> bool {
        matches!(self, Self::GlobalKeyDown | Self::GlobalKeyUp)
    }

    /// Check if this event can change the hover state of a Node.
    pub fn is_hovered(&self) -> bool {
        matches!(
            self,
            Self::MouseMove | Self::MouseEnter | Self::PointerOver | Self::PointerEnter
        )
    }

    /// Check if this event can press state of a Node.
    pub fn is_pressed(&self) -> bool {
        matches!(self, Self::MouseDown | Self::TouchStart | Self::PointerDown)
    }

    /// Check if this event can release the press state of a Node.
    pub fn is_released(&self) -> bool {
        matches!(&self, Self::Click | Self::PointerUp)
    }
}
