use dioxus_core::Template;

#[derive(Debug)]
pub enum EventMessage {
    UpdateTemplate(Template<'static>),
    PollVDOM,
    RequestRelayout,
}
