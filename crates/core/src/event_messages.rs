use freya_common::AccessibilityFocusStrategy;
use torin::prelude::{
    Area,
    CursorPoint,
};
use uuid::Uuid;
use winit::window::{
    CursorIcon,
    Window,
};

use crate::prelude::PlatformEvent;

pub struct TextGroupMeasurement {
    pub text_id: Uuid,
    pub cursor_id: usize,
    pub cursor_position: Option<CursorPoint>,
    pub cursor_selection: Option<(CursorPoint, CursorPoint)>,
}

/// Custom EventLoop messages
pub enum EventMessage {
    /// Poll the VirtualDOM
    PollVDOM,
    /// Request a rerender
    RequestRerender,
    /// Request a full rerender
    RequestFullRerender,
    /// Invalidate a certain drawing area
    InvalidateArea(Area),
    /// Remeasure a text elements group
    RemeasureTextGroup(TextGroupMeasurement),
    /// Change the cursor icon
    SetCursorIcon(CursorIcon),
    /// Accessibility Window Event
    Accessibility(accesskit_winit::WindowEvent),
    /// Focus with the given strategy
    FocusAccessibilityNode(AccessibilityFocusStrategy),
    /// Close the whole app
    ExitApp,
    /// Callback to access the Window.
    WithWindow(Box<dyn FnOnce(&Window) + Send + Sync>),
    /// Raw platform event, this are low level events.
    PlatformEvent(PlatformEvent),
}

impl From<accesskit_winit::Event> for EventMessage {
    fn from(value: accesskit_winit::Event) -> Self {
        Self::Accessibility(value.window_event)
    }
}
