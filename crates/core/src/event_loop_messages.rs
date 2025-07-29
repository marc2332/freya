#[cfg(all(debug_assertions, feature = "hot-reloading"))]
use dioxus_devtools::DevserverMsg;
use torin::prelude::{
    Area,
    CursorPoint,
};
#[cfg(feature = "winit")]
use winit::window::{
    CursorIcon,
    Window,
};

use crate::{
    accessibility::AccessibilityFocusStrategy,
    events::PlatformEvent,
};

pub struct TextGroupMeasurement {
    pub text_id: usize,
    pub cursor_id: usize,
    pub cursor_position: Option<CursorPoint>,
    pub cursor_selection: Option<(CursorPoint, CursorPoint)>,
}

/// Custom EventLoop messages
pub enum EventLoopMessage {
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
    /// Focus with the given strategy
    FocusAccessibilityNode(AccessibilityFocusStrategy),
    /// Close the whole app
    ExitApp,
    /// Raw platform event, this are low level events.
    PlatformEvent(PlatformEvent),
    /// Accessibility Window Event
    #[cfg(feature = "winit")]
    Accessibility(accesskit_winit::WindowEvent),
    /// Callback to access the Window.
    #[cfg(feature = "winit")]
    WithWindow(Box<dyn FnOnce(&Window) + Send + Sync>),
    /// dioxus hot patching events
    #[cfg(all(debug_assertions, feature = "hot-reloading"))]
    DioxusDevserverEvent(DevserverMsg),
}

#[cfg(feature = "winit")]
impl From<accesskit_winit::Event> for EventLoopMessage {
    fn from(value: accesskit_winit::Event) -> Self {
        Self::Accessibility(value.window_event)
    }
}
