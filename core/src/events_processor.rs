use dioxus_native_core::NodeId;
use rustc_hash::FxHashMap;

use crate::{
    dom_events::{does_event_move_cursor, DomEvent},
    freya_events::FreyaEvent,
    EventEmitter,
};

/// State of an element.
#[derive(Default)]
struct ElementState {
    hovered: bool,
}

/// [`EventsProcessor`] stores the elements events states.
///
/// TODO(marc2332): Remove deleted Elements
#[derive(Default)]
pub struct EventsProcessor {
    states: FxHashMap<NodeId, ElementState>,
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

        for (element, element_state) in self.states.iter_mut() {
            {
                let no_recent_mouse_movement_on_me = events_to_emit
                    .iter()
                    .find_map(|event| {
                        if event.does_move_cursor() && &event.node_id == element {
                            Some(false)
                        } else {
                            None
                        }
                    })
                    .unwrap_or(true);

                let recent_mouse_movement_event = events
                    .iter()
                    .find(|event| {
                        if let FreyaEvent::Mouse { name, .. } = event {
                            does_event_move_cursor(name)
                        } else {
                            false
                        }
                    })
                    .cloned();

                if element_state.hovered && no_recent_mouse_movement_on_me {
                    if let Some(FreyaEvent::Mouse { cursor, button, .. }) =
                        recent_mouse_movement_event
                    {
                        let events = new_events.entry("mouseleave".to_string()).or_default();
                        events.push((
                            *element,
                            FreyaEvent::Mouse {
                                name: "mouseleave".to_string(),
                                cursor,
                                button,
                            },
                        ));

                        // Mark the element as no longer being hovered
                        element_state.hovered = false;
                    }
                }
            }
        }

        // All these events will mark the node as being hovered
        // "mouseover" "mouseenter" "pointerover"  "pointerenter"

        // Emit valid events
        for event in &events_to_emit {
            let id = &event.node_id;

            let should_trigger = match event.name.as_str() {
                name @ "mouseover"
                | name @ "mouseenter"
                | name @ "pointerover"
                | name @ "pointerenter" => {
                    if !self.states.contains_key(id) {
                        self.states.insert(*id, ElementState::default());
                    }

                    let node_state = self.states.get_mut(id).unwrap();

                    if name == "mouseenter" || name == "pointerenter" {
                        // If the event is already being hovered then it's pointless to trigger the movement event
                        !node_state.hovered
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
            if does_event_move_cursor(event.name.as_str()) {
                if !self.states.contains_key(id) {
                    self.states.insert(*id, ElementState::default());
                }

                let node_state = self.states.get_mut(id).unwrap();

                node_state.hovered = true;
            }
        }

        new_events
    }
}
