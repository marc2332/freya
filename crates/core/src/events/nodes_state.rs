#![allow(clippy::type_complexity)]

use freya_native_core::NodeId;
use rustc_hash::FxHashMap;

use crate::{
    events::{DomEvent, PlatformEvent},
    prelude::{EventName, PotentialEvent, PotentialEvents},
};

#[derive(Clone)]
struct NodeMetadata {
    layer: Option<i16>,
}

/// [`NodesState`] stores the nodes states given incoming events.
#[derive(Default)]
pub struct NodesState {
    hovered_nodes: FxHashMap<NodeId, NodeMetadata>,
}

impl NodesState {
    /// Update the node states given the new events
    pub fn process_events(
        &mut self,
        events_to_emit: &[DomEvent],
        events: &[PlatformEvent],
    ) -> (PotentialEvents, Vec<DomEvent>) {
        let mut new_events_to_emit = Vec::default();
        let mut potential_events = PotentialEvents::default();

        let recent_mouse_movement_event = any_recent_mouse_movement(events);

        self.hovered_nodes.retain(|node_id, metadata| {
            let no_recent_mouse_movement_on_me =
                has_node_been_hovered_recently(events_to_emit, node_id);

            if no_recent_mouse_movement_on_me {
                if let Some(PlatformEvent::Mouse { cursor, button, .. }) =
                    recent_mouse_movement_event
                {
                    let events = potential_events.entry(EventName::MouseLeave).or_default();
                    // Emit a MouseLeave event as the cursor was moved outside the Node bounds
                    events.push(PotentialEvent {
                        node_id: *node_id,
                        layer: metadata.layer,
                        event: PlatformEvent::Mouse {
                            name: EventName::MouseLeave,
                            cursor,
                            button,
                        },
                    });

                    // Remove the node from the list of hovered nodes as now, the cursor has left
                    return false;
                }
            }
            true
        });

        // We clone this here so events emitted in the same batch that mark an node
        // as hovered will not affect the other events
        let hovered_nodes = self.hovered_nodes.clone();

        // Emit new colateral events
        for event in events_to_emit {
            if event.name.can_change_hover_state() {
                let is_hovered = hovered_nodes.contains_key(&event.node_id);

                // Mark the Node as hovered if it wasn't already
                if !is_hovered {
                    self.hovered_nodes
                        .insert(event.node_id, NodeMetadata { layer: event.layer });
                }

                if event.name.is_enter() {
                    // If the Node was already hovered, we don't need to emit an `enter` event again.
                    if is_hovered {
                        continue;
                    }
                }
            }

            new_events_to_emit.push(event.clone());
        }

        // Update the internal states of nodes given the events
        // e.g `mouseover` will mark the node as hovered.
        for event in events_to_emit {
            if event.name.was_cursor_moved() && !self.hovered_nodes.contains_key(&event.node_id) {
                self.hovered_nodes
                    .insert(event.node_id, NodeMetadata { layer: event.layer });
            }
        }

        // Order the events by their Nodes layer
        for events in potential_events.values_mut() {
            events.sort_by(|left, right| left.layer.cmp(&right.layer))
        }

        (potential_events, new_events_to_emit)
    }
}

fn any_recent_mouse_movement(events: &[PlatformEvent]) -> Option<PlatformEvent> {
    events
        .iter()
        .find(|event| event.get_name().was_cursor_moved())
        .cloned()
}

fn has_node_been_hovered_recently(events_to_emit: &[DomEvent], node_id: &NodeId) -> bool {
    events_to_emit
        .iter()
        .find_map(|event| {
            if event.name.was_cursor_moved() && &event.node_id == node_id {
                Some(false)
            } else {
                None
            }
        })
        .unwrap_or(true)
}
