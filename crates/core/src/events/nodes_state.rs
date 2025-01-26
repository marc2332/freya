#![allow(clippy::type_complexity)]

use freya_engine::prelude::Color;
use freya_native_core::{
    prelude::NodeImmutable,
    NodeId,
};
use freya_node_state::{
    Fill,
    StyleState,
};
use rustc_hash::FxHashMap;

use crate::{
    dom::FreyaDOM,
    events::{
        is_node_parent_of,
        DomEvent,
        PlatformEvent,
    },
    prelude::{
        EventName,
        PotentialEvent,
        PotentialEvents,
    },
};

#[derive(Clone, Debug)]
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
    /// Update the node states given the new events and suggest potential collateral new events
    pub fn process_collateral(
        &mut self,
        fdom: &FreyaDOM,
        pontential_events: &PotentialEvents,
        dom_events: &mut Vec<DomEvent>,
        events: &[PlatformEvent],
    ) -> PotentialEvents {
        let rdom = fdom.rdom();
        let mut potential_events = PotentialEvents::default();

        // Any mouse press event at all
        let recent_mouse_press_event = any_event_of(events, |e| e.was_cursor_pressed_or_released());

        // Pressed Nodes
        #[allow(unused_variables)]
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
                filter_dom_events_by(dom_events, node_id, |e| e.was_cursor_moved());

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

        dom_events.retain(|ev| {
            match ev.name {
                // Filter out enter events for nodes that were already hovered
                _ if ev.name.is_enter() => !self.hovered_nodes.contains_key(&ev.node_id),

                // Filter out press events for nodes that were already pressed
                _ if ev.name.is_pressed() => !self.pressed_nodes.contains_key(&ev.node_id),

                _ => true,
            }
        });

        // Update the state of the nodes given the new events.
        for events in pontential_events.values() {
            let mut child_node: Option<NodeId> = None;

            for PotentialEvent {
                node_id,
                event,
                layer,
            } in events.iter().rev()
            {
                if let Some(child_node) = child_node {
                    if !is_node_parent_of(rdom, child_node, *node_id) {
                        continue;
                    }
                }

                let node = rdom.get(*node_id).unwrap();
                let StyleState { background, .. } = &*node.get::<StyleState>().unwrap();

                if background != &Fill::Color(Color::TRANSPARENT)
                    && !event.get_name().does_go_through_solid()
                {
                    // If the background isn't transparent,
                    // we must make sure that next nodes are parent of it
                    // This only matters for events that bubble up (e.g. cursor movement events)
                    child_node = Some(*node_id);
                }

                match event.get_name() {
                    // Update hovered nodes state
                    name if name.can_change_hover_state() => {
                        // Mark the Node as hovered if it wasn't already
                        self.hovered_nodes.entry(*node_id).or_insert_with(|| {
                            #[cfg(debug_assertions)]
                            tracing::info!("Marked as hovered {:?}", node_id);

                            NodeMetadata { layer: *layer }
                        });
                    }

                    // Update pressed nodes state
                    name if name.can_change_press_state() => {
                        // Mark the Node as pressed if it wasn't already
                        self.pressed_nodes.entry(*node_id).or_insert_with(|| {
                            #[cfg(debug_assertions)]
                            tracing::info!("Marked as pressed {:?}", node_id);

                            NodeMetadata { layer: *layer }
                        });
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
