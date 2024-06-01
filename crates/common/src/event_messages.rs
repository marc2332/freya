use accesskit::NodeId;
use accesskit_winit::ActionRequestEvent;
use dioxus_core::Template;
use torin::prelude::CursorPoint;
use uuid::Uuid;
use winit::window::{CursorIcon, Window};

pub struct TextGroupMeasurement {
    pub text_id: Uuid,
    pub cursor_id: usize,
    pub cursor_position: Option<CursorPoint>,
    pub cursor_selection: Option<(CursorPoint, CursorPoint)>,
}

/// Custom EventLoop messages
pub enum EventMessage {
    /// Update the given template
    UpdateTemplate(Template),
    /// Pull the VirtualDOM
    PollVDOM,
    /// Request a rerender
    RequestRerender,
    /// Remeasure a text elements group
    RemeasureTextGroup(TextGroupMeasurement),
    /// Change the cursor icon
    SetCursorIcon(CursorIcon),
    /// Accessibility action request event
    ActionRequestEvent(ActionRequestEvent),
    /// Focus the given accessibility NodeID
    FocusAccessibilityNode(NodeId),
    /// Focus the next accessibility Node
    FocusNextAccessibilityNode,
    /// Focus the previous accessibility Node
    FocusPrevAccessibilityNode,
    /// Close the whole app
    ExitApp,
    /// Callback to access the Window.
    WithWindow(Box<dyn FnOnce(&Window) + Send + Sync>),
}

impl From<ActionRequestEvent> for EventMessage {
    fn from(value: ActionRequestEvent) -> Self {
        Self::ActionRequestEvent(value)
    }
}
