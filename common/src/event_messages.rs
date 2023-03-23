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
}
