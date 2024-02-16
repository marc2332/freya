#![allow(clippy::type_complexity)]

use dioxus_native_core::{tree::TreeRef, NodeId};
use freya_dom::prelude::FreyaDOM;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::events::{does_event_move_cursor, DomEvent, FreyaEvent};

/// [`ElementsState`] stores the elements states given incoming events.
#[derive(Default)]
pub struct ElementsState {
    hovered_elements: FxHashSet<NodeId>,
}

impl ElementsState {
    /// Update the Element states given the new events
    pub fn process_events(
        &mut self,
        events_to_emit: &[DomEvent],
        events: &[FreyaEvent],
        dom: &FreyaDOM,
    ) -> (FxHashMap<String, Vec<(NodeId, FreyaEvent)>>, Vec<DomEvent>) {
        let mut new_events_to_emit = Vec::default();
        let mut new_events = FxHashMap::<String, Vec<(NodeId, FreyaEvent)>>::default();

        let recent_mouse_movement_event = any_recent_mouse_movement(events);

        // Suggest emitting `mouseleave` in elements not being hovered anymore
        self.hovered_elements.retain(|node_id| {
            let no_recent_mouse_movement_on_me =
                has_node_been_hovered_recently(events_to_emit, node_id);

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

        // We clone this here so events emitted in the same batch that mark an element
        // as hovered will not affect the other events
        let hovered_elements = self.hovered_elements.clone();

        // Emit new colateral events
        for event in events_to_emit {
            let should_trigger = if event.can_change_element_hover_state() {
                let is_hovered = hovered_elements.contains(&event.node_id);

                // Mark the Node as hovered if it wasn't already
                if !is_hovered {
                    self.hovered_elements.insert(event.node_id);
                }

                if event.name == "mouseenter" || event.name == "pointerenter" {
                    // If the Node was already hovered, we don't need to emit an `enter` event again.
                    !is_hovered
                } else {
                    true
                }
            } else {
                true
            };

            if should_trigger {
                new_events_to_emit.push(event.clone());
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

        // Order the events by their Nodes height in the DOM
        for events in new_events.values_mut() {
            let rdom = dom.rdom();
            events.sort_by(|(l, _), (r, _)| {
                let height_l = rdom.tree_ref().height(*l);
                let height_r = rdom.tree_ref().height(*r);
                height_l.cmp(&height_r)
            })
        }

        (new_events, new_events_to_emit)
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
