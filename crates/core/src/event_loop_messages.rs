use cursor_icon::CursorIcon;
use torin::prelude::{
    Area,
    CursorPoint,
};
#[cfg(feature = "winit")]
use winit::window::Window;
use winit::window::WindowId;

#[cfg(feature = "winit")]
use crate::window_config;
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

pub enum EventLoopAppMessageAction {
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
    /// Close the window
    CloseWindow,
    /// Raw platform event, this are low level events.
    PlatformEvent(PlatformEvent),
    /// Accessibility Window Event
    #[cfg(feature = "winit")]
    Accessibility(accesskit_winit::WindowEvent),
    /// Callback to access the Window.
    #[cfg(feature = "winit")]
    WithWindow(Box<dyn FnOnce(&Window) + Send + Sync>),
}

pub struct EventLoopAppMessage {
    pub window_id: Option<WindowId>,
    pub action: EventLoopAppMessageAction,
}

/// Message for Freya's event loop
pub enum EventLoopMessage {
    App(EventLoopAppMessage),
    #[cfg(feature = "winit")]
    NewWindow(window_config::WindowConfig),
}
