pub enum EventsChunk {
    Batch(Vec<EmmitableEvent>),
    Processed(ProcessedEvents<NodeId, EventName, EmmitableEvent, PlatformEvent>),
}
use ragnarok::ProcessedEvents;

use crate::{
    events::{
        emittable::EmmitableEvent,
        name::EventName,
    },
    integration::PlatformEvent,
    node_id::NodeId,
    runner::Runner,
};

pub struct EventsExecutorAdapter<'a> {
    pub runner: &'a mut Runner,
}

impl ragnarok::EventsExecutor for EventsExecutorAdapter<'_> {
    type Key = NodeId;
    type Name = EventName;
    type Source = PlatformEvent;
    type Emmitable = EmmitableEvent;

    fn emit_event(&mut self, event: Self::Emmitable) -> bool {
        // Call the actual event handler
        self.runner
            .handle_event(event.node_id, event.name, event.data, event.bubbles)
    }
}
