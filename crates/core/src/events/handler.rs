use dioxus_core::{
    Event,
    VirtualDom,
};
use freya_native_core::dioxus::NodeImmutableDioxusExt;
use tracing::info;

use super::{
    DomEvent,
    NodesState,
    PotentialEvent,
};
use crate::{
    dom::SafeDOM,
    events::NodesStatesUpdate,
};

pub struct ProcessedEvents {
    pub(crate) dom_events: Vec<DomEvent>,
    pub(crate) flattened_potential_events: Vec<PotentialEvent>,
    pub(crate) nodes_states_update: NodesStatesUpdate,
}

pub fn handle_processed_events(
    sdom: &SafeDOM,
    vdom: &mut VirtualDom,
    nodes_state: &mut NodesState,
    ProcessedEvents {
        mut dom_events,
        flattened_potential_events,
        mut nodes_states_update,
    }: ProcessedEvents,
) {
    let fdom = sdom.get();
    let rdom = fdom.rdom();
    let mut processed_events = Vec::<DomEvent>::new();

    #[cfg(debug_assertions)]
    info!("Processing {} DOM events", dom_events.len());

    while !dom_events.is_empty() {
        let dom_event = dom_events.remove(0);

        let Some(element_id) = rdom
            .get(dom_event.node_id)
            .and_then(|node| node.mounted_id())
        else {
            continue;
        };
        let event_name = dom_event.name;
        let event = Event::new(dom_event.data.clone().any(), dom_event.bubbles);
        let event_clone = event.clone();

        #[cfg(debug_assertions)]
        info!("Running event {event_name:?} in Element {element_id:?}");

        // Call the actual event handler
        vdom.runtime()
            .handle_event(event_name.into(), event, element_id);

        if !event_clone.default_action_enabled() {
            // Get the events that this event can cancel
            let cancellable_events = dom_event.name.get_cancellable_events();

            // Remove the rest of dom events that are cancellable
            dom_events.retain(|event| !cancellable_events.contains(&event.name));

            // Discarda the potential events that dont find a matching dom event
            // So for instance, a cancelled mousemove event wont be discarded if a mousenter was processed just before
            // At the same time, a cancelled mouse event that actually gets discarded will only discard this node state update
            // So if the affected node was already being hovered from the last events run, it will continue to be as so
            for potential_event in &flattened_potential_events {
                let is_cancellable = cancellable_events.contains(&potential_event.name);
                if is_cancellable {
                    let processed_event = processed_events.iter().find(|event| {
                        potential_event.name == event.source_event
                            && potential_event.node_id == event.node_id
                    });
                    if processed_event.is_none() {
                        nodes_states_update
                            .discard(&potential_event.name, &potential_event.node_id);
                    }
                }
            }
        }

        processed_events.push(dom_event);
    }

    vdom.process_events();

    nodes_state.apply_update(nodes_states_update);
}
