use dioxus_native_core::NodeId;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::{
    dom_events::{does_event_move_cursor, DomEvent},
    freya_events::FreyaEvent,
    EventEmitter,
};

fn any_recent_mouse_movement(events: &[FreyaEvent]) -> Option<FreyaEvent> {
    events
        .iter()
        .find(|event| {
            if let FreyaEvent::Mouse { name, .. } = event {
                does_event_move_cursor(name)
            } else {
                false
            }
        })
        .cloned()
}

fn has_node_been_hovered_recently(events_to_emit: &[DomEvent], element: &NodeId) -> bool {
    events_to_emit
        .iter()
        .find_map(|event| {
            if event.does_move_cursor() && &event.node_id == element {
                Some(false)
            } else {
                None
            }
        })
        .unwrap_or(true)
}

/// [`EventsProcessor`] stores the elements events states.
#[derive(Default)]
pub struct EventsProcessor {
    hovered_elements: FxHashSet<NodeId>,
}

impl EventsProcessor {
    /// Update the Element states given the new events
    pub fn process_events(
        &mut self,
        events_to_emit: Vec<DomEvent>,
        events: &[FreyaEvent],
        event_emitter: &EventEmitter,
    ) -> FxHashMap<String, Vec<(NodeId, FreyaEvent)>> {
        let mut new_events = FxHashMap::<String, Vec<(NodeId, FreyaEvent)>>::default();

        let recent_mouse_movement_event = any_recent_mouse_movement(events);

        // Suggest emitting `mouseleave` in elements not being hovered anymore
        self.hovered_elements.retain(|node_id| {
            let no_recent_mouse_movement_on_me =
                has_node_been_hovered_recently(&events_to_emit, node_id);

            if no_recent_mouse_movement_on_me {
                if let Some(FreyaEvent::Mouse { cursor, button, .. }) = recent_mouse_movement_event
                {
                    let events = new_events.entry("mouseleave".to_string()).or_default();
                    events.push((
                        *node_id,
                        FreyaEvent::Mouse {
                            name: "mouseleave".to_string(),
                            cursor,
                            button,
                        },
                    ));

                    // Remove the node from the list of hovered elements
                    return false;
                }
            }
            true
        });

        // All these events will mark the node as being hovered
        // "mouseover" "mouseenter" "pointerover"  "pointerenter"

         // We clone this here so events emitted in the same batch that mark an element as hovered will not affect the other events
        let hovered_elements = self.hovered_elements.clone();

        // Emit valid events
        for event in &events_to_emit {
            let id = &event.node_id;

            let should_trigger = match event.name.as_str() {
                name @ "mouseover"
                | name @ "mouseenter"
                | name @ "pointerover"
                | name @ "pointerenter" => {
                    let is_hovered = hovered_elements.contains(id);

                    if !is_hovered {
                        self.hovered_elements.insert(*id);
                    }

                    if name == "mouseenter" || name == "pointerenter" {
                        // If the event is already being hovered then it's pointless to trigger the movement event
                        !is_hovered
                    } else {
                        true
                    }
                }
                _ => true,
            };

            if should_trigger {
                event_emitter.send(event.clone()).unwrap();
            }
        }

        // Update the internal states of elements given the events
        // e.g `mouseover` will mark the element as hovered.
        for event in events_to_emit {
            let id = &event.node_id;
            if does_event_move_cursor(event.name.as_str()) && !self.hovered_elements.contains(id) {
                self.hovered_elements.insert(*id);
            }
        }

        new_events
    }
}
