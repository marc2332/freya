use freya_native_core::real_dom::NodeImmutable;
use freya_native_core::NodeId;
use freya_native_core::{prelude::NodeImmutableDioxusExt, tree::TreeRef};

use freya_engine::prelude::*;
use freya_node_state::{Fill, Style, ViewportState};
use itertools::sorted;

pub use crate::events::{DomEvent, NodesState, PlatformEvent};

use crate::prelude::*;

/// Process the events and emit them to the VirtualDOM
pub fn process_events(
    dom: &FreyaDOM,
    events: &mut EventsQueue,
    event_emitter: &EventEmitter,
    nodes_state: &mut NodesState,
    scale_factor: f64,
) {
    // 1. Get global events created from the incoming events
    let global_events = measure_global_events(events);

    // 2. Get potential events that could be emitted based on the elements layout and viewports
    let potential_events = measure_potential_event_listeners(events, dom);

    // 3. Get what events can be actually emitted based on what elements are listening
    let dom_events = measure_dom_events(potential_events, dom, scale_factor);

    // 4. Filter the dom events and get potential collateral events, e.g. mouseover -> mouseenter
    let (potential_collateral_events, mut to_emit_dom_events) =
        nodes_state.process_events(&dom_events, events);

    // 5. Get what collateral events can actually be emitted
    let to_emit_dom_collateral_events =
        measure_dom_events(potential_collateral_events, dom, scale_factor);

    let colateral_global_events = measure_colateral_global_events(&to_emit_dom_collateral_events);

    // 6. Join both the dom and colateral dom events and sort them
    to_emit_dom_events.extend(to_emit_dom_collateral_events);
    to_emit_dom_events.sort_unstable();

    // 7. Emit the DOM events
    for event in to_emit_dom_events {
        event_emitter.send(event).unwrap();
    }

    // 8. Emit the global events
    emit_global_events_listeners(
        global_events,
        colateral_global_events,
        dom,
        event_emitter,
        scale_factor,
    );

    // 9. Clear the events queue
    events.clear();
}

/// Measure colateral global events
pub fn measure_colateral_global_events(events: &[DomEvent]) -> Vec<DomEvent> {
    let mut global_events = Vec::default();
    for event in events {
        let Some(event_name) = event.name.get_global_event() else {
            continue;
        };
        let mut global_event = event.clone();
        global_event.name = event_name;
        global_event.layer = None;
        global_events.push(global_event);
    }
    global_events
}

/// Measure global events
pub fn measure_global_events(events: &EventsQueue) -> Vec<PlatformEvent> {
    let mut global_events = Vec::default();
    for event in events {
        let Some(event_name) = event.get_name().get_global_event() else {
            continue;
        };
        let mut global_event = event.clone();
        global_event.set_name(event_name);
        global_events.push(global_event);
    }
    global_events
}

/// Measure what potential event listeners could be triggered
pub fn measure_potential_event_listeners(events: &EventsQueue, fdom: &FreyaDOM) -> PotentialEvents {
    let mut potential_events = PotentialEvents::default();

    let layout = fdom.layout();
    let rdom = fdom.rdom();
    let layers = fdom.layers();

    // Propagate events from the top to the bottom
    for (layer, layer_nodes) in sorted(layers.layers().iter()) {
        for node_id in layer_nodes {
            let layout_node = layout.get(*node_id);
            if let Some(layout_node) = layout_node {
                'events: for event in events.iter() {
                    if let PlatformEvent::Keyboard { name, .. } = event {
                        let event_data = PotentialEvent {
                            node_id: *node_id,
                            layer: Some(*layer),
                            event: event.clone(),
                        };
                        potential_events.entry(*name).or_default().push(event_data);
                    } else {
                        let data = match event {
                            PlatformEvent::Mouse { name, cursor, .. } => Some((name, cursor)),
                            PlatformEvent::Wheel { name, cursor, .. } => Some((name, cursor)),
                            PlatformEvent::Touch { name, location, .. } => Some((name, location)),
                            PlatformEvent::File { name, cursor, .. } => Some((name, cursor)),
                            _ => None,
                        };
                        if let Some((name, cursor)) = data {
                            let cursor_is_inside = layout_node.area.contains(cursor.to_f32());

                            // Make sure the cursor is inside the node area
                            if cursor_is_inside {
                                let node = rdom.get(*node_id).unwrap();
                                let node_viewports = node.get::<ViewportState>().unwrap();

                                // Make sure the cursor is inside all the applicable viewports from the element
                                for viewport_id in &node_viewports.viewports {
                                    let viewport = layout.get(*viewport_id).unwrap().visible_area();
                                    if !viewport.contains(cursor.to_f32()) {
                                        continue 'events;
                                    }
                                }

                                let event_data = PotentialEvent {
                                    node_id: *node_id,
                                    layer: Some(*layer),
                                    event: event.clone(),
                                };

                                potential_events
                                    .entry(*name)
                                    .or_insert_with(Vec::new)
                                    .push(event_data);
                            }
                        }
                    }
                }
            }
        }
    }

    potential_events
}

fn is_node_parent_of(rdom: &DioxusDOM, node: NodeId, parent_node: NodeId) -> bool {
    let mut stack = vec![parent_node];
    while let Some(id) = stack.pop() {
        let tree = rdom.tree_ref();
        let mut children = tree.children_ids(id);
        drop(tree);
        if children.contains(&node) {
            return true;
        }

        if rdom.contains(id) {
            children.reverse();
            stack.extend(children.iter());
        }
    }

    false
}

/// Measure what DOM events could be emitted
fn measure_dom_events(
    potential_events: PotentialEvents,
    fdom: &FreyaDOM,
    scale_factor: f64,
) -> Vec<DomEvent> {
    let mut new_events = Vec::new();
    let rdom = fdom.rdom();

    // Iterate over all the events
    for (event_name, event_nodes) in potential_events {
        let collateral_events = event_name.get_collateral_events();

        let mut valid_events: Vec<PotentialEvent> = Vec::new();

        // Iterate over the collateral events (including the source)
        'event: for collateral_event in collateral_events {
            let mut child_node: Option<NodeId> = None;

            // Iterate over the event nodes
            for PotentialEvent {
                node_id,
                event,
                layer,
            } in event_nodes.iter().rev()
            {
                let Some(node) = rdom.get(*node_id) else {
                    continue;
                };

                if rdom.is_node_listening(node_id, &collateral_event) {
                    let valid_node = if let Some(child_node) = child_node {
                        is_node_parent_of(rdom, child_node, *node_id)
                    } else {
                        true
                    };

                    if valid_node {
                        let mut valid_event = event.clone();
                        valid_event.set_name(collateral_event);
                        valid_events.push(PotentialEvent {
                            node_id: *node_id,
                            event: valid_event,
                            layer: *layer,
                        });

                        // Stack events that do not bubble up
                        if event.get_name().does_bubble() {
                            continue 'event;
                        }
                    }
                }

                let Style { background, .. } = &*node.get::<Style>().unwrap();

                if background != &Fill::Color(Color::TRANSPARENT)
                    && !event.get_name().does_go_through_solid()
                {
                    // If the background isn't transparent,
                    // we must make sure that next nodes are parent of it
                    // This only matters for events that bubble up (e.g. cursor movement events)
                    child_node = Some(*node_id);
                }
            }
        }

        for potential_event in valid_events {
            let layout = fdom.layout();
            let layout_node = layout.get(potential_event.node_id);
            if let Some(layout_node) = layout_node {
                let node_ref = fdom.rdom().get(potential_event.node_id).unwrap();
                let element_id = node_ref.mounted_id().unwrap();
                let event = DomEvent::new(
                    potential_event,
                    element_id,
                    Some(layout_node.visible_area()),
                    scale_factor,
                );
                new_events.push(event);
            }
        }
    }

    new_events
}

/// Emit global events
fn emit_global_events_listeners(
    global_events: Vec<PlatformEvent>,
    global_colateral_events: Vec<DomEvent>,
    fdom: &FreyaDOM,
    event_emitter: &EventEmitter,
    scale_factor: f64,
) {
    for global_event in global_events {
        let event_name = global_event.get_name();
        let listeners = fdom.rdom().get_listeners(&event_name);

        for listener in listeners {
            let element_id = listener.mounted_id().unwrap();
            let event = DomEvent::new(
                PotentialEvent {
                    node_id: listener.id(),
                    layer: None,
                    event: global_event.clone(),
                },
                element_id,
                None,
                scale_factor,
            );
            event_emitter.send(event).unwrap();
        }
    }
    for colateral_global_event in global_colateral_events {
        event_emitter.send(colateral_global_event).unwrap();
    }
}
