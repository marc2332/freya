use dioxus_core::Template;
use uuid::Uuid;
use winit::window::CursorIcon;

/// Custom EventLoop messages
#[derive(Debug)]
pub enum EventMessage {
    /// Update the given template
    UpdateTemplate(Template),
    /// Pull the VirtualDOM
    PollVDOM,
    /// Request a rerender
    RequestRerender,
    /// Remeasure a text elements group
    RemeasureTextGroup(Uuid),
    /// Change the cursor icon
    SetCursorIcon(CursorIcon),
    /// Accessibility Window Event
    Accessibility(accesskit_winit::WindowEvent),
    /// Focus the given accessibility NodeID
    FocusAccessibilityNode(accesskit::NodeId),
    /// Focus the next accessibility Node
    FocusNextAccessibilityNode,
    /// Focus the previous accessibility Node
    FocusPrevAccessibilityNode,
    /// Trigger window dragging
    DragWindow
}

impl From<accesskit_winit::Event> for EventMessage {
    fn from(value: accesskit_winit::Event) -> Self {
        Self::Accessibility(value.window_event)
    }
}
