use accesskit::NodeId;
use accesskit_winit::ActionRequestEvent;
use dioxus_core::Template;

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
