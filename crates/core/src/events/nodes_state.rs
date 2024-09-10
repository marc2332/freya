#![allow(clippy::type_complexity)]

use freya_native_core::NodeId;
use rustc_hash::FxHashMap;

use crate::{
    events::{
        DomEvent,
        PlatformEvent,
    },
    prelude::{
        EventName,
        PotentialEvent,
        PotentialEvents,
    },
};

#[derive(Clone)]
struct NodeMetadata {
    layer: Option<i16>,
}

/// [`NodesState`] stores the nodes states given incoming events.
#[derive(Default)]
pub struct NodesState {
    pressed_nodes: FxHashMap<NodeId, NodeMetadata>,
    hovered_nodes: FxHashMap<NodeId, NodeMetadata>,
}

impl NodesState {
    /// Given the current state, event, and NodeID check if it is allowed to be emitted
    /// For example, it will not make sense to emit a Click event on an element that was not pressed before.
    pub fn is_event_allowed(&self, event: &PlatformEvent, node_id: &NodeId) -> bool {
        if event.get_name().is_click() {
            self.pressed_nodes.contains_key(node_id)
        } else {
            true
        }
    }

    /// Update the node states given the new events and suggest potential collateral new events
    pub fn process_collateral(
        &mut self,
        pontential_events: &PotentialEvents,
        events_to_emit: &[DomEvent],
        events: &[PlatformEvent],
    ) -> PotentialEvents {
        let mut potential_events = PotentialEvents::default();

        // Any mouse press event at all
        let recent_mouse_press_event = any_event_of(events, |e| e.was_cursor_pressed_or_released());

        // Pressed Nodes
        self.pressed_nodes.retain(|node_id, _| {
            // Always unmark as pressed when there has been a new mouse down or click event
            if recent_mouse_press_event.is_some() {
                #[cfg(debug_assertions)]
                tracing::info!("Unmarked as pressed {:?}", node_id);

                // Remove the node from the list of pressed nodes
                return false;
            }

            true
        });

        // Any mouse movement event at all
        let recent_mouse_movement_event = any_event_of(events, |e| e.was_cursor_moved());

        // Hovered Nodes
        self.hovered_nodes.retain(|node_id, metadata| {
            // Check if a DOM event that moves the cursor in this Node will get emitted
            let no_recently_hovered =
                filter_dom_events_by(events_to_emit, node_id, |e| e.was_cursor_moved());

            if no_recently_hovered {
                // If there has been a mouse movement but a DOM event was not emitted to this node, then we safely assume
                // the user does no longer want to hover this Node
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

                    #[cfg(debug_assertions)]
                    tracing::info!("Unmarked as hovered {:?}", node_id);

                    // Remove the node from the list of hovered nodes
                    return false;
                }
            }
            true
        });

        // Update the state of the noves given the new events.

        // We clone this here so events emitted in the same batch that mark an node
        // as hovered or pressed will not affect the other events
        let hovered_nodes = self.hovered_nodes.clone();
        let pressed_nodes = self.pressed_nodes.clone();

        for events in pontential_events.values() {
            for PotentialEvent {
                node_id,
                event,
                layer,
            } in events
            {
                match event {
                    // Update hovered nodes state
                    PlatformEvent::Mouse { name, .. } if name.can_change_hover_state() => {
                        let is_hovered = hovered_nodes.contains_key(node_id);

                        // Mark the Node as hovered if it wasn't already
                        if !is_hovered {
                            self.hovered_nodes
                                .insert(*node_id, NodeMetadata { layer: *layer });

                            #[cfg(debug_assertions)]
                            tracing::info!("Marked as hovered {:?}", node_id);
                        }

                        if name.is_enter() {
                            // If the Node was already hovered, we don't need to emit an `enter` event again.
                            if is_hovered {
                                continue;
                            }
                        }
                    }

                    // Update pressed nodes state
                    PlatformEvent::Mouse { name, .. } if name.can_change_press_state() => {
                        let is_pressed = pressed_nodes.contains_key(node_id);

                        // Mark the Node as pressed if it wasn't already
                        if !is_pressed {
                            self.pressed_nodes
                                .insert(*node_id, NodeMetadata { layer: *layer });

                            #[cfg(debug_assertions)]
                            tracing::info!("Marked as pressed {:?}", node_id);
                        }
                    }
                    _ => {}
                }
            }
        }

        // Order the events by their Nodes layer
        for events in potential_events.values_mut() {
            events.sort_by(|left, right| left.layer.cmp(&right.layer))
        }

        potential_events
    }
}

fn any_event_of(
    events: &[PlatformEvent],
    filter: impl Fn(EventName) -> bool,
) -> Option<PlatformEvent> {
    events
        .iter()
        .find(|event| filter(event.get_name()))
        .cloned()
}

fn filter_dom_events_by(
    events_to_emit: &[DomEvent],
    node_id: &NodeId,
    filter: impl Fn(EventName) -> bool,
) -> bool {
    events_to_emit
        .iter()
        .find_map(|event| {
            if filter(event.name) && &event.node_id == node_id {
                Some(false)
            } else {
                None
            }
        })
        .unwrap_or(true)
}
