use dioxus_native_core::prelude::NodeImmutableDioxusExt;
use dioxus_native_core::real_dom::NodeImmutable;
use dioxus_native_core::NodeId;
use freya_dom::prelude::FreyaDOM;
use freya_layout::Layers;

use freya_node_state::Style;
use rustc_hash::FxHashMap;
use skia_safe::Color;

pub use crate::dom_events::DomEvent;
pub use crate::events_processor::EventsProcessor;
pub use crate::freya_events::FreyaEvent;

use crate::{EventEmitter, EventsQueue, NodesEvents, ViewportsCollection};

/// Measure globale events
pub fn measure_global_events(events: &EventsQueue) -> Vec<FreyaEvent> {
    let mut global_events = Vec::default();
    for event in events {
        let event_name = match event.get_name() {
            "click" => Some("globalclick"),
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
    layers_nums: &[&i16],
    layers: &Layers,
    events: &EventsQueue,
    viewports_collection: &ViewportsCollection,
    fdom: &FreyaDOM,
) -> NodesEvents {
    let mut potential_events = FxHashMap::default();

    let layout = fdom.layout();

    // Propagate events from the top to the bottom
    for layer_num in layers_nums {
        let layer = layers.layers.get(layer_num).unwrap();

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
                                let viewports = viewports_collection.get(node_id);

                                // Make sure the cursor is inside all the applicable viewports from the element
                                if let Some((_, viewports)) = viewports {
                                    for viewport_id in viewports {
                                        let viewport =
                                            viewports_collection.get(viewport_id).unwrap().0;
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
                        if background != &Color::TRANSPARENT
                            && FIRST_CAPTURED_EVENTS.contains(&derivated_event_name)
                        {
                            break 'event_nodes;
                        }

                        // Only keep the last matched event
                        if background != &Color::TRANSPARENT
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
            let areas = fdom.layout().get(*node_id).unwrap().clone();
            let node_ref = fdom.rdom().get(*node_id).unwrap();
            let element_id = node_ref.mounted_id().unwrap();
            let event = DomEvent::from_freya_event(
                *node_id,
                element_id,
                &request_event,
                Some(areas.area),
                scale_factor,
            );
            new_events.push(event);
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
            let event = DomEvent::from_freya_event(
                listener.id(),
                element_id,
                &global_event,
                None,
                scale_factor,
            );
            event_emitter.send(event).unwrap();
        }
    }
}

/// Process the events and emit them to the DOM
pub fn process_events(
    dom: &FreyaDOM,
    layers: &Layers,
    events: &mut EventsQueue,
    event_emitter: &EventEmitter,
    events_processor: &mut EventsProcessor,
    viewports_collection: &ViewportsCollection,
    scale_factor: f64,
) {
    let mut layers_nums: Vec<&i16> = layers.layers.keys().collect();

    // Order the layers from top to bottom
    layers_nums.sort();

    let global_events = measure_global_events(events);

    let mut potential_events =
        measure_potential_event_listeners(&layers_nums, layers, events, viewports_collection, dom);

    let emitted_events = measure_dom_events(&mut potential_events, dom, scale_factor);

    let mut potential_colateral_events =
        events_processor.process_events(emitted_events, events, event_emitter);

    let emitted_colateral_events =
        measure_dom_events(&mut potential_colateral_events, dom, scale_factor);

    for event in emitted_colateral_events {
        event_emitter.send(event).unwrap();
    }

    emit_global_events_listeners(global_events, dom, event_emitter, scale_factor);

    events.clear();
}
