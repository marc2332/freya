use accesskit::NodeId;
use accesskit_winit::ActionRequestEvent;
use dioxus_core::Template;
use uuid::Uuid;
use winit::window::CursorIcon;

/// Custom EventLoop messages
#[derive(Debug)]
pub enum EventMessage {
    /// Update the given template
    UpdateTemplate(Template<'static>),
    /// Pull the VirtualDOM
    PollVDOM,
    /// Request a layout recalculation
    RequestRelayout,
    /// Request a rerender
    RequestRerender,
    /// Remeasure a text elements group
    RemeasureTextGroup(Uuid),
    /// Change the cursor icon
    SetCursorIcon(CursorIcon),
    /// Accessibility action request event
    ActionRequestEvent(ActionRequestEvent),
    /// Focus the given accessibility NodeID
    FocusAccessibilityNode(NodeId),
}

impl From<ActionRequestEvent> for EventMessage {
    fn from(value: ActionRequestEvent) -> Self {
        Self::ActionRequestEvent(value)
    }
}
