use smallvec::SmallVec;

#[derive(Clone, Copy, PartialEq, Debug, Hash)]
pub enum EventName {
    Click,

    MouseDown,
    MouseOver,
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

    TouchCancel,
    TouchStart,
    TouchMove,
    TouchEnd,

    GlobalClick,
    GlobalMouseDown,
    GlobalMouseOver,
}

impl From<EventName> for &str {
    fn from(event: EventName) -> Self {
        match event {
            EventName::Click => "click",
            EventName::MouseDown => "mousedown",
            EventName::MouseOver => "mouseover",
            EventName::MouseEnter => "mouseenter",
            EventName::MouseLeave => "mouseleave",
            EventName::Wheel => "wheel",
            EventName::PointerOver => "pointerover",
            EventName::PointerDown => "pointerdown",
            EventName::PointerEnter => "pointerenter",
            EventName::PointerLeave => "pointerleave",
            EventName::PointerUp => "pointerup",
            EventName::KeyDown => "keydown",
            EventName::KeyUp => "keyup",
            EventName::TouchCancel => "touchcancel",
            EventName::TouchStart => "touchstart",
            EventName::TouchMove => "touchmove",
            EventName::TouchEnd => "touchend",
            EventName::GlobalClick => "globalclick",
            EventName::GlobalMouseDown => "globalmousedown",
            EventName::GlobalMouseOver => "globalmouseover",
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
            // Always priorize leave events before anything else
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
            Self::Click => Some(Self::GlobalClick),
            Self::MouseDown => Some(Self::GlobalMouseDown),
            Self::MouseOver => Some(Self::GlobalMouseOver),
            _ => None,
        }
    }

    /// Some events might cause other events, like for example:
    /// A `mouseover` might also trigger a `mouseenter`
    /// A `mousedown` or a `touchdown` might also trigger a `pointerdown`
    pub fn get_colateral_events(&self) -> SmallVec<[Self; 4]> {
        let mut events = SmallVec::new();

        events.push(*self);

        match self {
            Self::MouseOver | Self::TouchMove => {
                events.extend([Self::MouseEnter, Self::PointerEnter, Self::PointerOver])
            }
            Self::MouseDown | Self::TouchStart => events.push(Self::PointerDown),
            Self::Click | Self::TouchEnd => events.push(Self::PointerUp),
            Self::MouseLeave => events.push(Self::PointerLeave),
            _ => {}
        }

        events
    }

    /// Check if the event event means that the pointer (e.g cursor) just entered a Node
    pub fn is_enter(&self) -> bool {
        matches!(&self, Self::MouseEnter | Self::PointerEnter)
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
        )
    }

    /// Check if the event means the cursor was moved
    pub fn was_cursor_moved(&self) -> bool {
        matches!(
            &self,
            Self::MouseOver | Self::MouseEnter | Self::PointerEnter | Self::PointerOver
        )
    }

    // Bubble all events except:
    // - Keyboard events
    // - Mouse movements events
    pub fn does_bubble(&self) -> bool {
        !matches!(
            self,
            Self::KeyDown
                | Self::KeyUp
                | Self::MouseLeave
                | Self::PointerLeave
                | Self::MouseEnter
                | Self::PointerEnter
                | Self::MouseOver
                | Self::PointerOver
        )
    }

    // Only let events that do not move the mouse, go through solid nodes
    pub fn does_go_through_solid(&self) -> bool {
        !self.was_cursor_moved()
    }

    // Check if this event can change the hover state of a Node.
    pub fn can_change_hover_state(&self) -> bool {
        matches!(
            self,
            Self::MouseOver | Self::MouseEnter | Self::PointerOver | Self::PointerEnter
        )
    }
}
