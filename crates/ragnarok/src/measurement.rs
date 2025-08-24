use itertools::Itertools;

use crate::{
    EmmitableEvent,
    EventsMeasurer,
    NameOfEvent,
    NodeKey,
    PotentialEvent,
    PotentialEvents,
    SourceEvent,
};

/// For every source event and its derivated events, a global equivalent event is emitted.
pub fn measure_source_global_events<
    Key: NodeKey,
    Name: NameOfEvent,
    Source: SourceEvent<Name = Name>,
    Emmitable: EmmitableEvent<Key = Key, Name = Name>,
>(
    events_measurer: &impl EventsMeasurer<
        Key = Key,
        Name = Name,
        Emmitable = Emmitable,
        Source = Source,
    >,
    source_events: &Vec<Source>,
    emmitable_events: &mut Vec<Emmitable>,
) {
    for source_event in source_events {
        let event_name = source_event.as_event_name();
        let derived_events_names = event_name.get_derived_events();

        for derived_event_name in derived_events_names {
            for global_event_name in derived_event_name.get_global_events() {
                let listeners = events_measurer.get_listeners_of(&global_event_name);

                for listener in listeners {
                    let event = events_measurer.new_emmitable_event(
                        listener,
                        global_event_name,
                        source_event.clone(),
                        None,
                    );
                    emmitable_events.push(event)
                }
            }
        }
    }
}

/// Measure what event listeners could potentially be triggered
pub fn measure_potential_events<
    Key: NodeKey,
    Name: NameOfEvent,
    Source: SourceEvent<Name = Name>,
    Emmitable: EmmitableEvent<Key = Key, Name = Name>,
>(
    source_events: &Vec<Source>,
    events_measurer: &impl EventsMeasurer<
        Key = Key,
        Name = Name,
        Emmitable = Emmitable,
        Source = Source,
    >,
    focus_id: Option<Key>,
) -> PotentialEvents<Key, Name, Source> {
    let mut potential_events = PotentialEvents::default();

    // Walk layer by layer from the bottom to the top
    for (layer, layer_nodes) in events_measurer
        .get_layers()
        .sorted_by(|(layer, _), (layer_b, _)| layer.cmp(layer_b))
    {
        for node_id in layer_nodes {
            for source_event in source_events {
                let Some(cursor) = source_event.try_cursor() else {
                    if focus_id == Some(*node_id) {
                        let potential_event = PotentialEvent {
                            node_key: *node_id,
                            layer: *layer,
                            name: source_event.as_event_name(),
                            source_event: source_event.clone(),
                        };
                        potential_events
                            .entry(source_event.as_event_name())
                            .or_default()
                            .push(potential_event);
                    }
                    continue;
                };

                if !events_measurer.is_point_inside(*node_id, cursor) {
                    continue;
                }

                let potential_event = PotentialEvent {
                    node_key: *node_id,
                    layer: *layer,
                    name: source_event.as_event_name(),
                    source_event: source_event.clone(),
                };

                potential_events
                    .entry(source_event.as_event_name())
                    .or_insert_with(Vec::new)
                    .push(potential_event);
            }
        }
    }

    potential_events
}

/// Measure what events could be emitted
pub fn measure_emmitable_events<
    Key: NodeKey,
    Name: NameOfEvent,
    Source: SourceEvent<Name = Name>,
    Emmitable: EmmitableEvent,
>(
    potential_events: &PotentialEvents<Key, Name, Source>,
    events_measurer: &impl EventsMeasurer<
        Key = Key,
        Name = Name,
        Emmitable = Emmitable,
        Source = Source,
    >,
) -> Vec<Emmitable> {
    let mut emmitable_events = Vec::new();

    for (event, potential_events) in potential_events {
        // Get the derived events, but exclude globals like some file events
        let derived_events_names = event
            .get_derived_events()
            .into_iter()
            .filter(|event| !event.is_global());

        // Iterate over the derived events (including the source)
        'event: for derived_event_name in derived_events_names {
            let mut child_node: Option<Key> = None;

            // Iterate over the potential events in reverse so the ones in higher layers appeat first
            for PotentialEvent {
                node_key: node_id,
                name,
                source_event,
                ..
            } in potential_events.iter().rev()
            {
                if let Some(child_node) = child_node {
                    if !events_measurer.is_node_parent_of(child_node, *node_id) {
                        continue;
                    }
                }

                if events_measurer.is_listening_to(*node_id, &derived_event_name) {
                    let area = events_measurer.try_area_of(*node_id);
                    if let Some(area) = area {
                        let emmitable_event = events_measurer.new_emmitable_event(
                            *node_id,
                            derived_event_name,
                            source_event.clone(),
                            Some(area),
                        );
                        emmitable_events.push(emmitable_event);

                        // Events that bubble will only be emitted once
                        // Those that don't will be stacked
                        if name.does_bubble() {
                            continue 'event;
                        }
                    }
                }

                if !events_measurer.is_node_transparent(*node_id) && !name.does_go_through_solid() {
                    // If the background isn't transparent,
                    // we must make sure that next nodes are parent of it
                    // This only matters for events that bubble up (e.g. cursor click events)
                    child_node = Some(*node_id);
                }
            }
        }
    }

    emmitable_events
}
