#![allow(clippy::type_complexity)]

use freya_engine::prelude::Color;
use freya_native_core::{
    events::EventName,
    prelude::NodeImmutable,
    NodeId,
};
use rustc_hash::FxHashSet;

use crate::{
    dom::FreyaDOM,
    events::{
        is_node_parent_of,
        DomEvent,
        PlatformEvent,
        PotentialEvent,
    },
    states::StyleState,
    types::PotentialEvents,
    values::Fill,
};

/// [`NodesState`] stores the nodes states given incoming events.
#[derive(Default)]
pub struct NodesState {
    pressed_nodes: FxHashSet<NodeId>,
    hovered_nodes: FxHashSet<NodeId>,
}

impl NodesState {
    /// Retain or not the states of the nodes given the [DomEvent]s and based on the sideffects of these removals
    /// a set of [PotentialEvent]s are returned.
    pub fn retain_states(
        &mut self,
        fdom: &FreyaDOM,
        dom_events: &[DomEvent],
        platform_events: &[PlatformEvent],
        scale_factor: f64,
    ) -> Vec<DomEvent> {
        let layout = fdom.layout();
        let rdom = fdom.rdom();
        let mut collateral_dom_events = Vec::default();

        // Any press event at all
        let platform_press_event = platform_events.iter().any(|e| e.is_pressed());

        // Pressed Nodes
        #[allow(unused_variables)]
        self.pressed_nodes.retain(|node_id| {
            // Check if a DOM event that presses this Node will get emitted
            let dom_press_event = dom_events
                .iter()
                .any(|event| event.name.is_pressed() && &event.node_id == node_id);

            // If there has been a mouse press but a DOM event was not emitted to this node, then we safely assume
            // the user does no longer want to press this Node
            if !dom_press_event && platform_press_event {
                #[cfg(debug_assertions)]
                tracing::info!("Unmarked as pressed {:?}", node_id);

                // Remove the node from the list of pressed nodes
                return false;
            }

            true
        });

        // Any movement event at all
        let platform_movement_event = platform_events.iter().find(|e| e.is_moved());

        // Hovered Nodes
        self.hovered_nodes.retain(|node_id| {
            // Check if a DOM event that moves the cursor in this Node will get emitted
            let dom_movement_event = dom_events
                .iter()
                .any(|event| event.name.is_moved() && &event.node_id == node_id);

            if !dom_movement_event {
                // If there has been a mouse movement but a DOM event was not emitted to this node, then we safely assume
                // the user does no longer want to hover this Node
                if let Some(platform_event) = platform_movement_event {
                    if let Some(layout_node) = layout.get(*node_id) {
                        // Emit a MouseLeave event as the cursor was moved outside the Node bounds
                        let event = EventName::MouseLeave;
                        for derived_event in event.get_derived_events() {
                            let is_node_listening = rdom.is_node_listening(node_id, &derived_event);
                            if is_node_listening {
                                collateral_dom_events.push(DomEvent::new(
                                    *node_id,
                                    derived_event,
                                    platform_event.clone(),
                                    Some(layout_node.area),
                                    scale_factor,
                                ));
                            }
                        }

                        #[cfg(debug_assertions)]
                        tracing::info!("Unmarked as hovered {:?}", node_id);
                    }

                    // Remove the node from the list of hovered nodes
                    return false;
                }
            }
            true
        });

        collateral_dom_events
    }

    pub fn filter_dom_events(&self, dom_events: &mut Vec<DomEvent>) {
        dom_events.retain(|ev| {
            match ev.name {
                // Only let through enter events when the node was not hovered
                _ if ev.name.is_enter() => !self.hovered_nodes.contains(&ev.node_id),

                // Only let through release events when the node was already pressed
                _ if ev.name.is_released() => self.pressed_nodes.contains(&ev.node_id),

                _ => true,
            }
        });
    }

    /// Create the nodes states given the [PotentialEvent]s.
    pub fn create_update(
        &self,
        fdom: &FreyaDOM,
        potential_events: &PotentialEvents,
    ) -> NodesStatesUpdate {
        let rdom = fdom.rdom();

        let mut hovered_nodes = FxHashSet::default();
        let mut pressed_nodes = FxHashSet::default();

        // Update the state of the nodes given the new events.
        for events in potential_events.values() {
            let mut child_node: Option<NodeId> = None;

            for PotentialEvent { node_id, name, .. } in events.iter().rev() {
                if let Some(child_node) = child_node {
                    if !is_node_parent_of(rdom, child_node, *node_id) {
                        continue;
                    }
                }

                let node = rdom.get(*node_id).unwrap();
                let StyleState { background, .. } = &*node.get::<StyleState>().unwrap();

                if background != &Fill::Color(Color::TRANSPARENT) && !name.does_go_through_solid() {
                    // If the background isn't transparent,
                    // we must make sure that next nodes are parent of it
                    // This only matters for events that bubble up (e.g. cursor click events)
                    child_node = Some(*node_id);
                }

                match name {
                    // Update hovered nodes state
                    name if name.is_moved() => {
                        // Mark the Node as hovered if it wasn't already
                        hovered_nodes.insert(*node_id);

                        #[cfg(debug_assertions)]
                        tracing::info!("Marked as hovered {:?}", node_id);
                    }

                    // Update pressed nodes state
                    name if name.is_pressed() => {
                        // Mark the Node as pressed if it wasn't already
                        pressed_nodes.insert(*node_id);

                        #[cfg(debug_assertions)]
                        tracing::info!("Marked as pressed {:?}", node_id);
                    }
                    _ => {}
                }
            }
        }
        NodesStatesUpdate {
            pressed_nodes,
            hovered_nodes,
        }
    }

    /// Apply the given [NodesStatesUpdate] in a way so that only newly hovered/pressed nodes are cached.
    /// Any discard of nodes in the [NodesStatesUpdate] wont matter here.
    pub fn apply_update(&mut self, update: NodesStatesUpdate) {
        self.hovered_nodes.extend(update.hovered_nodes);
        self.pressed_nodes.extend(update.pressed_nodes);
    }
}

pub struct NodesStatesUpdate {
    pressed_nodes: FxHashSet<NodeId>,
    hovered_nodes: FxHashSet<NodeId>,
}

impl NodesStatesUpdate {
    /// Discard the state of a given [NodeId] and a [EventName] in this [NodesStatesUpdate].
    pub fn discard(&mut self, name: &EventName, node_id: &NodeId) {
        match name {
            // Just like a movement makes the node hover, a discard movement also unhovers it
            _ if name.is_moved() => {
                self.hovered_nodes.remove(node_id);
            }
            _ if name.is_pressed() => {
                self.pressed_nodes.remove(node_id);
            }
            _ => {}
        }
    }
}
