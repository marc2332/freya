use freya_engine::prelude::*;
use freya_native_core::{
    real_dom::NodeImmutable,
    tree::TreeRef,
    NodeId,
};
use freya_node_state::{
    Fill,
    StyleState,
    ViewportState,
};
use itertools::sorted;

pub use crate::events::{
    DomEvent,
    NodesState,
    PlatformEvent,
};
use crate::{
    elements::ElementUtilsResolver,
    prelude::*,
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
    let potential_events = measure_potential_event_listeners(events, fdom, scale_factor, focus_id);

    // Get what events can be actually emitted based on what elements are listening
    let mut dom_events = measure_dom_events(&potential_events, fdom, scale_factor);

    // Get potential collateral events, e.g. mousemove -> mouseenter
    let potential_collateral_events =
        nodes_state.process_collateral(fdom, &potential_events, &mut dom_events, events);

    // Get what collateral events can actually be emitted
    let collateral_events_to_emit =
        measure_dom_events(&potential_collateral_events, fdom, scale_factor);

    // Get the collateral events created by the global events
    let collateral_global_events = measure_collateral_global_events(&collateral_events_to_emit);

    // Join both the `dom_events` and their collateral events
    dom_events.extend(collateral_events_to_emit);
    dom_events.sort_unstable();

    // Get the global events
    measure_global_events(fdom, events, &mut dom_events, scale_factor);
    dom_events.extend(collateral_global_events);

    // Send all the events
    event_emitter.send(dom_events).unwrap();

    // Clear the events queue
    events.clear();
}

/// Create a global event for every collateral event
pub fn measure_collateral_global_events(events: &[DomEvent]) -> Vec<DomEvent> {
    let mut global_events = Vec::default();
    for event in events {
        let Some(event_name) = event.name.get_global_event() else {
            continue;
        };
        global_events.push(DomEvent {
            name: event_name,
            node_id: event.node_id,
            data: event.data.clone(),
            bubbles: event.bubbles,
        });
    }
    global_events
}

/// For every event in the queue, a global event is created
pub fn measure_global_events(
    fdom: &FreyaDOM,
    events: &EventsQueue,
    dom_events: &mut Vec<DomEvent>,
    scale_factor: f64,
) {
    for PlatformEvent { name, data } in events {
        let Some(global_name) = name.get_global_event() else {
            continue;
        };

        let listeners = fdom.rdom().get_listeners(&global_name);

        for listener in listeners {
            let event = DomEvent::new(
                PotentialEvent {
                    node_id: listener.id(),
                    layer: None,
                    name: global_name,
                    data: data.clone(),
                },
                None,
                scale_factor,
            );
            dom_events.push(event)
        }
    }
}

/// Measure what event listeners could potentially be triggered
pub fn measure_potential_event_listeners(
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
        // Iterate over the nodes in reversed to their declaration because
        // the next nodes could always render in top of their previous
        for node_id in layer_nodes.iter().rev() {
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
                            layer: Some(*layer),
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
                    layer: Some(*layer),
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
        let collateral_events = event_name.get_collateral_events();

        // Iterate over the collateral events (including the source)
        'event: for collateral_event in collateral_events {
            let mut child_node: Option<NodeId> = None;

            // Iterate over the potential events in reverse so the ones in higher layers appeat first
            for PotentialEvent {
                node_id,
                data,
                name,
                layer,
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

                if rdom.is_node_listening(node_id, &collateral_event) {
                    let potential_event = PotentialEvent {
                        node_id: *node_id,
                        name: collateral_event,
                        data: data.clone(),
                        layer: *layer,
                    };

                    let layout_node = layout.get(*node_id).unwrap();
                    let dom_event = DomEvent::new(
                        potential_event,
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
