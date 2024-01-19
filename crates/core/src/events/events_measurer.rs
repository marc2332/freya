use crate::layout::{Layers, Viewports};
use dioxus_native_core::prelude::NodeImmutableDioxusExt;
use dioxus_native_core::real_dom::NodeImmutable;
use dioxus_native_core::NodeId;
use freya_dom::prelude::FreyaDOM;

use freya_engine::prelude::*;
use freya_node_state::{Fill, Style};
use rustc_hash::FxHashMap;

pub use crate::events::{DomEvent, ElementsState, FreyaEvent};

use crate::types::{EventEmitter, EventsQueue, NodesEvents};

/// Process the events and emit them to the VirtualDOM
pub fn process_events(
    dom: &FreyaDOM,
    layers: &Layers,
    events: &mut EventsQueue,
    event_emitter: &EventEmitter,
    elements_state: &mut ElementsState,
    viewports: &Viewports,
    scale_factor: f64,
) {
    // 1. Get global events created from the incominge vents
    let global_events = measure_global_events(events);

    // 2. Get potential events that could be emitted based on the elements layout and viewports
    let mut potential_events = measure_potential_event_listeners(layers, events, viewports, dom);

    // 3. Get what events can be actually emitted based on what elements are listening
    let dom_events = measure_dom_events(&mut potential_events, dom, scale_factor);

    // 4. Filter the dom events and get potential derived events, e.g mouseover -> mouseenter
    let (mut potential_colateral_events, mut to_emit_dom_events) =
        elements_state.process_events(&dom_events, events);

    // 5. Get what derived events can actually be emitted
    let to_emit_dom_colateral_events =
        measure_dom_events(&mut potential_colateral_events, dom, scale_factor);

    // 6. Join both the dom and colateral dom events and sort them
    to_emit_dom_events.extend(to_emit_dom_colateral_events);
    to_emit_dom_events.sort_unstable();

    // 7. Emit the DOM events
    for event in to_emit_dom_events {
        event_emitter.send(event).unwrap();
    }

    // 8. Emit the global events
    emit_global_events_listeners(global_events, dom, event_emitter, scale_factor);

    // 9. Clear the events queue
    events.clear();
}

/// Measure globale events
pub fn measure_global_events(events: &EventsQueue) -> Vec<FreyaEvent> {
    let mut global_events = Vec::default();
    for event in events {
        let event_name = match event.get_name() {
            "click" => Some("globalclick"),
            "mousedown" => Some("globalmousedown"),
            "mouseover" => Some("globalmouseover"),
            _ => None,
        };
        if let Some(event_name) = event_name {
            let mut global_event = event.clone();
            global_event.set_name(event_name.to_string());
            global_events.push(global_event);
        }
    }
    global_events
}

/// Measure what potential event listeners could be triggered
pub fn measure_potential_event_listeners(
    layers: &Layers,
    events: &EventsQueue,
    viewports: &Viewports,
    fdom: &FreyaDOM,
) -> NodesEvents {
    let mut potential_events = FxHashMap::default();

    let layout = fdom.layout();

    // Propagate events from the top to the bottom
    for (_, layer) in layers.layers() {
        for node_id in layer {
            let areas = layout.get(*node_id);
            if let Some(areas) = areas {
                'events: for event in events.iter() {
                    if let FreyaEvent::Keyboard { name, .. } = event {
                        let event_data = (*node_id, event.clone());
                        potential_events
                            .entry(name.clone())
                            .or_insert_with(|| vec![event_data.clone()])
                            .push(event_data);
                    } else {
                        let data = match event {
                            FreyaEvent::Mouse { name, cursor, .. } => Some((name, cursor)),
                            FreyaEvent::Wheel { name, cursor, .. } => Some((name, cursor)),
                            FreyaEvent::Touch { name, location, .. } => Some((name, location)),
                            _ => None,
                        };
                        if let Some((name, cursor)) = data {
                            let cursor_is_inside = areas.area.contains(cursor.to_f32());

                            // Make sure the cursor is inside the node area
                            if cursor_is_inside {
                                let node_viewports = viewports.get(node_id);

                                // Make sure the cursor is inside all the applicable viewports from the element
                                if let Some((_, node_viewports)) = node_viewports {
                                    for viewport_id in node_viewports {
                                        let viewport = viewports.get(viewport_id).unwrap().0;
                                        if let Some(viewport) = viewport {
                                            if !viewport.contains(cursor.to_f32()) {
                                                continue 'events;
                                            }
                                        }
                                    }
                                }

                                let event_data = (*node_id, event.clone());

                                potential_events
                                    .entry(name.clone())
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

/// Some events might cause other events, like for example:
/// A `mouseover` might also trigger a `mouseenter`
/// A `mousedown` or a `touchdown` might also trigger a `pointerdown`
fn get_derivated_events(event_name: &str) -> Vec<&str> {
    match event_name {
        "mouseover" => {
            vec![event_name, "mouseenter", "pointerenter", "pointerover"]
        }
        "mousedown" | "touchdown" => {
            vec![event_name, "pointerdown"]
        }
        "click" | "ontouchend" => {
            vec![event_name, "pointerup"]
        }
        "mouseleave" => {
            vec![event_name, "pointerleave"]
        }
        _ => vec![event_name],
    }
}

const STACKED_EVENTS: [&str; 13] = [
    "mouseover",
    "mouseenter",
    "mouseleave",
    "click",
    "keydown",
    "keyup",
    "touchcancel",
    "touchend",
    "touchmove",
    "touchstart",
    "pointerover",
    "pointerenter",
    "pointerleave",
];

const FIRST_CAPTURED_EVENTS: [&str; 1] = ["wheel"];

const LAST_CAPTURED_EVENTS: [&str; 3] = ["click", "touchstart", "touchend"];

/// Measure what DOM events could be emited
fn measure_dom_events(
    potential_events: &mut NodesEvents,
    fdom: &FreyaDOM,
    scale_factor: f64,
) -> Vec<DomEvent> {
    let mut new_events = Vec::new();
    let rdom = fdom.rdom();

    for (event_name, event_nodes) in potential_events.iter_mut() {
        let derivated_events = get_derivated_events(event_name.as_str());

        let mut found_nodes: Vec<(&NodeId, FreyaEvent)> = Vec::new();
        for derivated_event_name in derivated_events {
            let listeners = rdom.get_listening_sorted(derivated_event_name);
            'event_nodes: for (node_id, request) in event_nodes.iter() {
                for listener in &listeners {
                    if listener.id() == *node_id {
                        let Style { background, .. } = &*listener.get::<Style>().unwrap();

                        let mut request = request.clone();
                        request.set_name(derivated_event_name.to_string());

                        // Stop searching on first match
                        if background != &Fill::Color(Color::TRANSPARENT)
                            && FIRST_CAPTURED_EVENTS.contains(&derivated_event_name)
                        {
                            break 'event_nodes;
                        }

                        // Only keep the last matched event
                        if background != &Fill::Color(Color::TRANSPARENT)
                            && LAST_CAPTURED_EVENTS.contains(&derivated_event_name)
                        {
                            found_nodes.clear();
                        }

                        // Stack the matched events
                        if STACKED_EVENTS.contains(&derivated_event_name) {
                            found_nodes.push((node_id, request))
                        } else {
                            found_nodes = vec![(node_id, request)]
                        }
                    }
                }
            }
        }

        for (node_id, request_event) in found_nodes {
            let areas = fdom.layout().get(*node_id).cloned();
            if let Some(areas) = areas {
                let node_ref = fdom.rdom().get(*node_id).unwrap();
                let element_id = node_ref.mounted_id().unwrap();
                let event = DomEvent::new(
                    *node_id,
                    element_id,
                    &request_event,
                    Some(areas.visible_area()),
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
    global_events: Vec<FreyaEvent>,
    fdom: &FreyaDOM,
    event_emitter: &EventEmitter,
    scale_factor: f64,
) {
    for global_event in global_events {
        let event_name = global_event.get_name();
        let listeners = fdom.rdom().get_listening_sorted(event_name);

        for listener in listeners {
            let element_id = listener.mounted_id().unwrap();
            let event = DomEvent::new(listener.id(), element_id, &global_event, None, scale_factor);
            event_emitter.send(event).unwrap();
        }
    }
}
