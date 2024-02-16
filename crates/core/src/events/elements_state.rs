#![allow(clippy::type_complexity)]

use dioxus_native_core::NodeId;
use rustc_hash::FxHashMap;

use crate::{
    events::{does_event_move_cursor, DomEvent, FreyaEvent},
    prelude::{PotentialEvent, PotentialEvents},
};

#[derive(Clone)]
struct ElementMetadata {
    layer: Option<i16>,
}

/// [`ElementsState`] stores the elements states given incoming events.
#[derive(Default)]
pub struct ElementsState {
    hovered_elements: FxHashMap<NodeId, ElementMetadata>,
}

impl ElementsState {
    /// Update the Element states given the new events
    pub fn process_events(
        &mut self,
        events_to_emit: &[DomEvent],
        events: &[FreyaEvent],
    ) -> (PotentialEvents, Vec<DomEvent>) {
        let mut new_events_to_emit = Vec::default();
        let mut potential_events = PotentialEvents::default();

        let recent_mouse_movement_event = any_recent_mouse_movement(events);

        // Suggest emitting `mouseleave` in elements not being hovered anymore
        self.hovered_elements.retain(|node_id, metadata| {
            let no_recent_mouse_movement_on_me =
                has_node_been_hovered_recently(events_to_emit, node_id);

            if no_recent_mouse_movement_on_me {
                if let Some(FreyaEvent::Mouse { cursor, button, .. }) = recent_mouse_movement_event
                {
                    let events = potential_events
                        .entry("mouseleave".to_string())
                        .or_default();
                    events.push(PotentialEvent {
                        node_id: *node_id,
                        layer: metadata.layer,
                        event: FreyaEvent::Mouse {
                            name: "mouseleave".to_string(),
                            cursor,
                            button,
                        },
                    });

                    // Remove the node from the list of hovered elements
                    return false;
                }
            }
            true
        });

        // We clone this here so events emitted in the same batch that mark an element
        // as hovered will not affect the other events
        let hovered_elements = self.hovered_elements.clone();

        // Emit new colateral events
        for event in events_to_emit {
            if event.can_change_element_hover_state() {
                let is_hovered = hovered_elements.contains_key(&event.node_id);

                // Mark the Node as hovered if it wasn't already
                if !is_hovered {
                    self.hovered_elements
                        .insert(event.node_id, ElementMetadata { layer: event.layer });
                }

                if event.name == "mouseenter" || event.name == "pointerenter" {
                    // If the Node was already hovered, we don't need to emit an `enter` event again.
                    if is_hovered {
                        continue;
                    }
                }
            }

            new_events_to_emit.push(event.clone());
        }

        // Update the internal states of elements given the events
        // e.g `mouseover` will mark the element as hovered.
        for event in events_to_emit {
            let id = &event.node_id;
            if does_event_move_cursor(event.name.as_str())
                && !self.hovered_elements.contains_key(id)
            {
                self.hovered_elements
                    .insert(*id, ElementMetadata { layer: event.layer });
            }
        }

        // Order the events by their Nodes layer
        for events in potential_events.values_mut() {
            events.sort_by(|left, right| left.layer.cmp(&right.layer))
        }

        (potential_events, new_events_to_emit)
    }
}

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
