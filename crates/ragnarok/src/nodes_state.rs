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
pub struct NodesState<Key: NodeKey> {
    pressed_nodes: FxHashSet<Key>,
    hovered_nodes: FxHashSet<Key>,
    entered_node: Option<Key>,
}

impl<Key: NodeKey> Default for NodesState<Key> {
    fn default() -> Self {
        Self {
            pressed_nodes: FxHashSet::default(),
            hovered_nodes: FxHashSet::default(),
            entered_node: None,
        }
    }
}

pub type PotentialEvents<Key, Name, Source> =
    FxHashMap<Name, Vec<PotentialEvent<Key, Name, Source>>>;

impl<Key: NodeKey> NodesState<Key> {
    /// Retain node states given the [EmmitableEvent]s, emitting leave events as side effects.
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

        let source_press_event = source_events.iter().any(|e| e.is_pressed());

        #[allow(unused_variables)]
        self.pressed_nodes.retain(|node_key| {
            let emmitable_press_event = emmitable_events
                .iter()
                .any(|event| event.name().is_pressed() && &event.key() == node_key);

            // A press occurred but not on this node, so it's no longer pressed
            if !emmitable_press_event && source_press_event {
                #[cfg(debug_assertions)]
                tracing::info!("Unmarked as pressed {:?}", node_key);

                return false;
            }

            true
        });

        let source_movement_event = source_events.iter().find(|e| e.is_moved());
        let mut removed_from_hovered = FxHashSet::default();

        self.hovered_nodes.retain(|node_key| {
            let Some(area) = events_measurer.try_area_of(node_key) else {
                removed_from_hovered.insert(*node_key);
                return false;
            };

            let cursor_still_inside = source_movement_event
                .and_then(|e| e.try_location())
                .is_none_or(|cursor| events_measurer.is_point_inside(node_key, cursor));

            if cursor_still_inside {
                return true;
            }

            // Safe: `cursor_still_inside` is only false when a movement event exists
            let source_event = source_movement_event.unwrap();
            for derived_event in Name::new_leave().get_derived_events() {
                if events_measurer.is_listening_to(node_key, &derived_event) {
                    collateral_emmitable_events.push(events_measurer.new_emmitable_event(
                        *node_key,
                        derived_event,
                        source_event.clone(),
                        Some(area),
                    ));
                }
            }

            #[cfg(debug_assertions)]
            tracing::info!("Unmarked as hovered {:?}", node_key);

            removed_from_hovered.insert(*node_key);

            false
        });

        // Emit exclusive leave when the deepest node changes
        // but the old node is still hovered (otherwise the regular leave covers it).
        if let Some(source_event) = source_movement_event {
            let new_deepest = emmitable_events
                .iter()
                .find(|e| e.name().is_exclusive_enter())
                .map(|e| e.key());

            if let Some(old_entered) = self.entered_node {
                let deepest_changed = new_deepest != Some(old_entered);
                let still_hovered = !removed_from_hovered.contains(&old_entered);

                if deepest_changed && still_hovered {
                    let exclusive_leave = Name::new_exclusive_leave();
                    if events_measurer.is_listening_to(&old_entered, &exclusive_leave)
                        && let Some(area) = events_measurer.try_area_of(&old_entered)
                    {
                        collateral_emmitable_events.push(events_measurer.new_emmitable_event(
                            old_entered,
                            exclusive_leave,
                            source_event.clone(),
                            Some(area),
                        ));
                    }
                }
            }
        }

        collateral_emmitable_events
    }

    pub(crate) fn filter_emmitable_events<
        Emmitable: EmmitableEvent<Key = Key, Name = Name>,
        Name: NameOfEvent,
    >(
        &mut self,
        emmitable_events: &mut Vec<Emmitable>,
    ) {
        let entered_node = emmitable_events
            .iter()
            .rev()
            .find(|e| e.name().is_moved() || e.name().is_exclusive_enter())
            .map(|e| e.key());

        emmitable_events.retain(|ev| match ev.name() {
            // Deduplicate exclusive enter against `entered_node`
            _ if ev.name().is_exclusive_enter()  => {
                entered_node.as_ref() == Some(&ev.key()) && entered_node != self.entered_node
            }
            // Deduplicate non-exclusive enter against `hovered_nodes`
            _ if ev.name().is_enter() => !self.hovered_nodes.contains(&ev.key()),
            // Only emit release events for already-pressed nodes
            _ if ev.name().is_released() => self.pressed_nodes.contains(&ev.key()),
            _ => true,
        });

        self.entered_node = entered_node;
    }

    /// Create the nodes states given the [PotentialEvent]s.
    pub fn create_update<Name: NameOfEvent, Source: SourceEvent>(
        &self,
        events_measurer: &impl EventsMeasurer<Key = Key, Name = Name>,
        potential_events: &PotentialEvents<Key, Name, Source>,
    ) -> NodesStatesUpdate<Key> {
        let mut hovered_nodes = FxHashSet::default();
        let mut pressed_nodes = FxHashSet::default();

        for events in potential_events.values() {
            let mut child_node: Option<Key> = None;

            for PotentialEvent { node_key, name, .. } in events.iter().rev() {
                if let Some(child_node) = child_node
                    && !events_measurer.is_node_parent_of(&child_node, *node_key)
                {
                    continue;
                }

                // If the background isn't transparent,
                // we must make sure that next nodes are parent of it.
                // This only matters for events that don't go through solids (e.g. cursor click events)
                if !events_measurer.is_node_transparent(node_key) && !name.does_go_through_solid() {
                    child_node = Some(*node_key);
                }

                match name {
                    name if name.is_moved() => {
                        hovered_nodes.insert(*node_key);

                        #[cfg(debug_assertions)]
                        tracing::info!("Marked as hovered {:?}", node_key);
                    }
                    name if name.is_pressed() => {
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

    /// Apply the given [NodesStatesUpdate], extending the cached hovered/pressed nodes.
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
            pressed_nodes: FxHashSet::default(),
            hovered_nodes: FxHashSet::default(),
        }
    }
}

impl<Key: NodeKey> NodesStatesUpdate<Key> {
    /// Discard the state of a given [NodeKey] and [NameOfEvent] in this [NodesStatesUpdate].
    pub fn discard<Name: NameOfEvent>(&mut self, name: &Name, node_key: &Key) {
        match name {
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
