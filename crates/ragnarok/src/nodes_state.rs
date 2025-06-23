use std::collections::HashSet;

use rustc_hash::{
    FxHashMap,
    FxHashSet,
};

use crate::{
    EmmitableEvent,
    EventsMeasurer,
    NameOfEvent,
    NodeKey,
    PotentialEvent,
    SourceEvent,
};

/// [`NodesState`] stores the nodes states given incoming events.
#[derive(Default)]
pub struct NodesState<Key: NodeKey> {
    pressed_nodes: FxHashSet<Key>,
    hovered_nodes: FxHashSet<Key>,
}

pub type PotentialEvents<Key, Name, Source> =
    FxHashMap<Name, Vec<PotentialEvent<Key, Name, Source>>>;

impl<Key: NodeKey> NodesState<Key> {
    /// Retain or not the states of the nodes given the [EmmitableEvent]s and based on the sideffects of these removals
    /// a set of [PotentialEvent]s are returned.
    pub(crate) fn retain_states<
        Emmitable: EmmitableEvent<Key = Key, Name = Name>,
        Name: NameOfEvent,
        Source: SourceEvent,
    >(
        &mut self,
        events_measurer: &impl EventsMeasurer<
            Key = Key,
            Name = Name,
            Emmitable = Emmitable,
            Source = Source,
        >,
        emmitable_events: &[Emmitable],
        source_events: &[Source],
    ) -> Vec<Emmitable> {
        let mut collateral_emmitable_events = Vec::default();

        // Any press event at all
        let source_press_event = source_events.iter().any(|e| e.is_pressed());

        // Pressed Nodes
        #[allow(unused_variables)]
        self.pressed_nodes.retain(|node_key| {
            // Check if a DOM event that presses this Node will get emitted
            let emmitable_press_event = emmitable_events
                .iter()
                .any(|event| event.name().is_pressed() && &event.key() == node_key);

            // If there has been a mouse press but a DOM event was not emitted to this node, then we safely assume
            // the user does no longer want to press this Node
            if !emmitable_press_event && source_press_event {
                #[cfg(debug_assertions)]
                tracing::info!("Unmarked as pressed {:?}", node_key);

                // Remove the node from the list of pressed nodes
                return false;
            }

            true
        });

        // Any movement event at all
        let source_movement_event = source_events.iter().find(|e| e.is_moved());

        // Hovered Nodes
        self.hovered_nodes.retain(|node_key| {
            // Check if a DOM event that moves the cursor in this Node will get emitted
            let emmitable_movement_event = emmitable_events.iter().any(|event| {
                (event.name().is_moved() || event.name().is_enter()) && &event.key() == node_key
            });

            if !emmitable_movement_event {
                // If there has been a mouse movement but a DOM event was not emitted to this node, then we safely assume
                // the user does no longer want to hover this Node
                if let Some(source_event) = source_movement_event {
                    if let Some(area) = events_measurer.try_area_of(*node_key) {
                        // Emit a MouseLeave event as the cursor was moved outside the Node bounds
                        let event = Name::new_leave();
                        for derived_event in event.get_derived_events() {
                            let is_node_listening =
                                events_measurer.is_listening_to(*node_key, &derived_event);
                            if is_node_listening {
                                collateral_emmitable_events.push(
                                    events_measurer.new_emmitable_event(
                                        *node_key,
                                        derived_event,
                                        source_event.clone(),
                                        Some(area),
                                    ),
                                );
                            }
                        }

                        #[cfg(debug_assertions)]
                        tracing::info!("Unmarked as hovered {:?}", node_key);
                    }

                    // Remove the node from the list of hovered nodes
                    return false;
                }
            }
            true
        });

        collateral_emmitable_events
    }

    pub(crate) fn filter_emmitable_events<
        Emmitable: EmmitableEvent<Key = Key, Name = Name>,
        Name: NameOfEvent,
    >(
        &self,
        emmitable_events: &mut Vec<Emmitable>,
    ) {
        emmitable_events.retain(|ev| {
            match ev.name() {
                // Only let through enter events when the node was not hovered
                _ if ev.name().is_enter() => !self.hovered_nodes.contains(&ev.key()),

                // Only let through release events when the node was already pressed
                _ if ev.name().is_released() => self.pressed_nodes.contains(&ev.key()),

                _ => true,
            }
        });
    }

    /// Create the nodes states given the [PotentialEvent]s.
    pub fn create_update<
        Emmitable: EmmitableEvent<Key = Key, Name = Name>,
        Name: NameOfEvent,
        Source: SourceEvent,
    >(
        &self,
        events_measurer: &impl EventsMeasurer<Key = Key, Name = Name>,
        potential_events: &PotentialEvents<Key, Name, Source>,
    ) -> NodesStatesUpdate<Key> {
        let mut hovered_nodes = FxHashSet::default();
        let mut pressed_nodes = FxHashSet::default();

        // Update the state of the nodes given the new events.
        for events in potential_events.values() {
            let mut child_node: Option<Key> = None;

            for PotentialEvent { node_key, name, .. } in events.iter().rev() {
                if let Some(child_node) = child_node {
                    if !events_measurer.is_node_parent_of(child_node, *node_key) {
                        continue;
                    }
                }

                if !events_measurer.is_node_transparent(*node_key) && !name.does_go_through_solid()
                {
                    // If the background isn't transparent,
                    // we must make sure that next nodes are parent of it
                    // This only matters for events that bubble up (e.g. cursor click events)
                    child_node = Some(*node_key);
                }

                match name {
                    // Update hovered nodes state
                    name if name.is_moved() => {
                        // Mark the Node as hovered if it wasn't already
                        hovered_nodes.insert(*node_key);

                        #[cfg(debug_assertions)]
                        tracing::info!("Marked as hovered {:?}", node_key);
                    }

                    // Update pressed nodes state
                    name if name.is_pressed() => {
                        // Mark the Node as pressed if it wasn't already
                        pressed_nodes.insert(*node_key);

                        #[cfg(debug_assertions)]
                        tracing::info!("Marked as pressed {:?}", node_key);
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
    pub fn apply_update(&mut self, update: NodesStatesUpdate<Key>) {
        self.hovered_nodes.extend(update.hovered_nodes);
        self.pressed_nodes.extend(update.pressed_nodes);
    }

    pub fn is_hovered(&self, key: Key) -> bool {
        self.hovered_nodes.contains(&key)
    }

    pub fn is_pressed(&self, key: Key) -> bool {
        self.pressed_nodes.contains(&key)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NodesStatesUpdate<Key: NodeKey> {
    pressed_nodes: FxHashSet<Key>,
    hovered_nodes: FxHashSet<Key>,
}

impl<Key: NodeKey> Default for NodesStatesUpdate<Key> {
    fn default() -> Self {
        Self {
            pressed_nodes: HashSet::default(),
            hovered_nodes: HashSet::default(),
        }
    }
}

impl<Key: NodeKey> NodesStatesUpdate<Key> {
    /// Discard the state of a given [Self::Key] and a [Self::EventName] in this [NodesStatesUpdate].
    pub fn discard<Name: NameOfEvent>(&mut self, name: &Name, node_key: &Key) {
        match name {
            // Just like a movement makes the node hover, a discard movement also unhovers it
            _ if name.is_moved() => {
                self.hovered_nodes.remove(node_key);
            }
            _ if name.is_pressed() => {
                self.pressed_nodes.remove(node_key);
            }
            _ => {}
        }
    }
}
