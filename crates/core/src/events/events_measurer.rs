use freya_engine::prelude::*;
use freya_native_core::{
    real_dom::NodeImmutable,
    tree::TreeRef,
    NodeId,
};
use itertools::{
    sorted,
    Itertools,
};

use super::{
    PlatformEventData,
    PotentialEvent,
};
pub use crate::events::{
    DomEvent,
    NodesState,
    PlatformEvent,
};
use crate::{
    dom::{
        DioxusDOM,
        FreyaDOM,
    },
    elements::{
        ElementUtils,
        ElementUtilsResolver,
    },
    states::{
        StyleState,
        ViewportState,
    },
    types::{
        EventEmitter,
        EventsQueue,
        PotentialEvents,
    },
    values::Fill,
};

/// Process the events and emit them to the VirtualDOM
pub fn process_events(
    fdom: &FreyaDOM,
    events: &mut EventsQueue,
    event_emitter: &EventEmitter,
    nodes_state: &mut NodesState,
    scale_factor: f64,

    focus_id: Option<NodeId>,
) {
    // Get potential events that could be emitted based on the elements layout and viewports
    let potential_events = measure_potential_events(events, fdom, scale_factor, focus_id);

    // Get what events can be actually emitted based on what elements are listening
    let mut dom_events = measure_dom_events(&potential_events, fdom, scale_factor);

    // Get dom ollateral events, e.g. mousemove -> mouseenter
    let collateral_dom_events = nodes_state.retain_states(fdom, &dom_events, events, scale_factor);
    nodes_state.filter_dom_events(&mut dom_events);
    nodes_state.create_states(fdom, &potential_events);

    // Get the global events
    measure_platform_global_events(fdom, events, &mut dom_events, scale_factor);

    // Join all the dom events and sort them
    dom_events.extend(collateral_dom_events);
    dom_events.sort_unstable();

    let mut flattened_potential_events = potential_events.into_values().flatten().collect_vec();
    flattened_potential_events.sort_unstable();

    // Send all the events
    event_emitter
        .send((dom_events, flattened_potential_events))
        .unwrap();

    // Clear the events queue
    events.clear();
}

/// For every event in the queue, a global event is created
pub fn measure_platform_global_events(
    fdom: &FreyaDOM,
    events: &EventsQueue,
    dom_events: &mut Vec<DomEvent>,
    scale_factor: f64,
) {
    let rdom = fdom.rdom();
    for PlatformEvent { name, data } in events {
        let derived_events_names = name.get_derived_events();

        for derived_event_name in derived_events_names {
            let Some(global_name) = derived_event_name.get_global_event() else {
                continue;
            };

            let listeners = rdom.get_listeners(&global_name);

            for listener in listeners {
                let event = DomEvent::new(
                    listener.id(),
                    global_name,
                    *name,
                    data.clone(),
                    None,
                    scale_factor,
                );
                dom_events.push(event)
            }
        }
    }
}

/// Measure what event listeners could potentially be triggered
pub fn measure_potential_events(
    events: &EventsQueue,
    fdom: &FreyaDOM,
    scale_factor: f64,
    focus_id: Option<NodeId>,
) -> PotentialEvents {
    let mut potential_events = PotentialEvents::default();

    let layout = fdom.layout();
    let rdom = fdom.rdom();
    let layers = fdom.layers();

    // Walk layer by layer from the bottom to the top
    for (layer, layer_nodes) in sorted(layers.iter()) {
        for node_id in layer_nodes.iter() {
            let Some(layout_node) = layout.get(*node_id) else {
                continue;
            };
            'events: for PlatformEvent { name, data } in events {
                let cursor = match data {
                    PlatformEventData::Mouse { cursor, .. } => cursor,
                    PlatformEventData::Wheel { cursor, .. } => cursor,
                    PlatformEventData::Touch { location, .. } => location,
                    PlatformEventData::File { cursor, .. } => cursor,
                    PlatformEventData::Keyboard { .. } if focus_id == Some(*node_id) => {
                        let potential_event = PotentialEvent {
                            node_id: *node_id,
                            layer: *layer,
                            name: *name,
                            data: data.clone(),
                        };
                        potential_events
                            .entry(*name)
                            .or_default()
                            .push(potential_event);
                        continue;
                    }
                    _ => continue,
                };

                let node = rdom.get(*node_id).unwrap();
                let node_type = node.node_type();

                let Some(element_utils) = node_type.tag().and_then(|tag| tag.utils()) else {
                    continue;
                };

                // Make sure the cursor is inside the node area
                if !element_utils.is_point_inside_area(
                    cursor,
                    &node,
                    layout_node,
                    scale_factor as f32,
                ) {
                    continue;
                }

                let node = rdom.get(*node_id).unwrap();
                let node_viewports = node.get::<ViewportState>().unwrap();

                // Make sure the cursor is inside all the inherited viewports of the node
                for node_id in &node_viewports.viewports {
                    let node_ref = rdom.get(*node_id).unwrap();
                    let node_type = node_ref.node_type();
                    let Some(element_utils) = node_type.tag().and_then(|tag| tag.utils()) else {
                        continue;
                    };
                    let layout_node = layout.get(*node_id).unwrap();
                    if !element_utils.is_point_inside_area(
                        cursor,
                        &node_ref,
                        layout_node,
                        scale_factor as f32,
                    ) {
                        continue 'events;
                    }
                }

                let potential_event = PotentialEvent {
                    node_id: *node_id,
                    layer: *layer,
                    name: *name,
                    data: data.clone(),
                };

                potential_events
                    .entry(*name)
                    .or_insert_with(Vec::new)
                    .push(potential_event);
            }
        }
    }

    potential_events
}

pub fn is_node_parent_of(rdom: &DioxusDOM, node: NodeId, parent_node: NodeId) -> bool {
    let mut head = Some(node);
    while let Some(id) = head.take() {
        let tree = rdom.tree_ref();
        if let Some(parent_id) = tree.parent_id(id) {
            if parent_id == parent_node {
                return true;
            }

            head = Some(parent_id)
        }
    }
    false
}

/// Measure what DOM events could be emitted
fn measure_dom_events(
    potential_events: &PotentialEvents,
    fdom: &FreyaDOM,
    scale_factor: f64,
) -> Vec<DomEvent> {
    let mut dom_events = Vec::new();
    let rdom = fdom.rdom();
    let layout = fdom.layout();

    for (event_name, event_nodes) in potential_events {
        // Get the derived events, but exclude globals like some file events
        let derived_events_names = event_name
            .get_derived_events()
            .into_iter()
            .filter(|event| !event.is_global());

        // Iterate over the derived events (including the source)
        'event: for derived_event_name in derived_events_names {
            let mut child_node: Option<NodeId> = None;

            // Iterate over the potential events in reverse so the ones in higher layers appeat first
            for PotentialEvent {
                node_id,
                data,
                name,
                ..
            } in event_nodes.iter().rev()
            {
                let Some(node) = rdom.get(*node_id) else {
                    continue;
                };

                if let Some(child_node) = child_node {
                    if !is_node_parent_of(rdom, child_node, *node_id) {
                        continue;
                    }
                }

                if rdom.is_node_listening(node_id, &derived_event_name) {
                    let layout_node = layout.get(*node_id).unwrap();
                    let dom_event = DomEvent::new(
                        *node_id,
                        derived_event_name,
                        *event_name,
                        data.clone(),
                        Some(layout_node.visible_area()),
                        scale_factor,
                    );
                    dom_events.push(dom_event);

                    // Events that bubble will only be emitted once
                    // Those that don't will be stacked
                    if name.does_bubble() {
                        continue 'event;
                    }
                }

                let StyleState { background, .. } = &*node.get::<StyleState>().unwrap();

                if background != &Fill::Color(Color::TRANSPARENT) && !name.does_go_through_solid() {
                    // If the background isn't transparent,
                    // we must make sure that next nodes are parent of it
                    // This only matters for events that bubble up (e.g. cursor click events)
                    child_node = Some(*node_id);
                }
            }
        }
    }

    dom_events
}
