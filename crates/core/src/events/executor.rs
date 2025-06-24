use dioxus_core::{
    Event,
    VirtualDom,
};
use freya_native_core::{
    dioxus::NodeImmutableDioxusExt,
    events::EventName,
    NodeId,
};

use crate::{
    dom::DioxusDOM,
    events::{
        DomEvent,
        PlatformEvent,
    },
};

pub struct EventsExecutorAdapter<'a> {
    pub rdom: &'a DioxusDOM,
    pub vdom: &'a mut VirtualDom,
}

impl ragnarok::EventsExecutor for EventsExecutorAdapter<'_> {
    type Key = NodeId;
    type Name = EventName;
    type Source = PlatformEvent;
    type Emmitable = DomEvent;

    fn emit_event(&mut self, event: Self::Emmitable) -> bool {
        let Some(element_id) = self
            .rdom
            .get(event.node_id)
            .and_then(|node| node.mounted_id())
        else {
            return false;
        };
        let event_name = event.name;
        let event = Event::new(event.data.clone().any(), event.bubbles);
        let event_clone = event.clone();

        #[cfg(debug_assertions)]
        tracing::info!("Running event {event_name:?} in Element {element_id:?}");

        // Call the actual event handler
        self.vdom
            .runtime()
            .handle_event(event_name.into(), event, element_id);

        event_clone.default_action_enabled()
    }

    fn emitted_events(&mut self) {
        self.vdom.process_events();
    }
}
