use dioxus_core::{
    Event,
    VirtualDom,
};
use freya_native_core::dioxus::NodeImmutableDioxusExt;

use super::{
    DomEvent,
    NodesState,
    PotentialEvent,
};
use crate::dom::SafeDOM;

pub fn handle_processed_events(
    sdom: &SafeDOM,
    vdom: &mut VirtualDom,
    nodes_state: &mut NodesState,
    mut dom_events: Vec<DomEvent>,
    flattened_potential_events: Vec<PotentialEvent>,
) {
    let fdom = sdom.get();
    let rdom = fdom.rdom();
    let mut processed_events = Vec::<DomEvent>::new();

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

        // Call the actual event handler
        vdom.runtime()
            .handle_event(event_name.into(), event, element_id);

        if !event_clone.default_action_enabled() {
            // Get the events that this event can cancel
            let cancellable_events = dom_event.name.get_cancellable_events();

            // Remove the rest of dom events that are cancellable
            dom_events.retain(|event| !cancellable_events.contains(&event.name));

            // Discard all the potential events that don't match the processed events
            // And that can be cancelled
            // This should include this event itself
            for potential_event in &flattened_potential_events {
                let is_cancellable = cancellable_events.contains(&potential_event.name);
                if is_cancellable {
                    let processed_event = processed_events.iter().find(|event| {
                        potential_event.name == event.source_event
                            && potential_event.node_id == event.node_id
                    });
                    if processed_event.is_none() {
                        nodes_state.clear_state(&potential_event.name, &potential_event.node_id);
                    }
                }
            }
        }

        processed_events.push(dom_event);

        vdom.process_events();
    }
}
