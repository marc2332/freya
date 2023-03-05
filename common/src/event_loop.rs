use accesskit::NodeId;
use accesskit_winit::ActionRequestEvent;
use dioxus_core::Template;

#[derive(Debug)]
pub enum EventMessage {
    TemplateUpdate(Template<'static>),
    ActionRequestEvent(ActionRequestEvent),
    FocusAccessibilityButton(NodeId),
    Empty,
}

impl From<ActionRequestEvent> for EventMessage {
    fn from(value: ActionRequestEvent) -> Self {
        Self::ActionRequestEvent(value)
    }
}
