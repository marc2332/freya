use dioxus_native_core::{node::NodeType, NodeId};
use freya_common::Area;
use freya_dom::FreyaDOM;
use freya_layout::NodeLayoutMeasurer;
use freya_layout::{Layers, RenderData};

use rustc_hash::FxHashMap;
use skia_safe::{textlayout::FontCollection, Color};
use std::ops::Index;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub mod events;

use events::{DomEvent, EventsProcessor, FreyaEvent};

pub type EventEmitter = UnboundedSender<DomEvent>;
pub type EventReceiver = UnboundedReceiver<DomEvent>;
pub type EventsQueue = Vec<FreyaEvent>;
pub type ViewportsCollection = FxHashMap<NodeId, (Option<Area>, Vec<NodeId>)>;
pub type NodesEvents<'a> = FxHashMap<&'a str, Vec<(RenderData, FreyaEvent)>>;

// Calculate all the applicable viewports for the given nodes
pub fn calculate_viewports(
    layers_nums: &[&i16],
    layers: &Layers,
    rdom: &FreyaDOM,
) -> ViewportsCollection {
    let mut viewports_collection = FxHashMap::default();

    for layer_num in layers_nums {
        let layer = layers.layers.get(layer_num).unwrap();
        for dom_element in layer.values() {
            let dioxus_node = dom_element.get_node(rdom);
            if let Some(dioxus_node) = dioxus_node {
                if let NodeType::Element { tag, .. } = &dioxus_node.node_data.node_type {
                    if tag == "container" {
                        viewports_collection
                            .entry(*dom_element.get_id())
                            .or_insert_with(|| (None, Vec::new()))
                            .0 = Some(dom_element.node_area);
                    }
                    if let Some(children) = &dom_element.get_children() {
                        for child in children {
                            if viewports_collection.contains_key(dom_element.get_id()) {
                                let mut inherited_viewports = viewports_collection
                                    .get(dom_element.get_id())
                                    .unwrap()
                                    .1
                                    .clone();

                                inherited_viewports.push(*dom_element.get_id());

                                viewports_collection.insert(*child, (None, inherited_viewports));
                            }
                        }
                    }
                }
            }
        }
    }
    viewports_collection
}

// Calculate possible events in nodes considering their viewports
pub fn calculate_node_events<'a>(
    layers_nums: &[&i16],
    layers: &Layers,
    events: &EventsQueue,
    viewports_collection: &ViewportsCollection,
) -> (NodesEvents<'a>, Vec<FreyaEvent>) {
    let mut calculated_events = FxHashMap::default();
    let mut global_events = Vec::default();

    for event in events {
        let event_name = match event.get_name() {
            "click" => Some("globalclick"),
            "mouseover" => Some("globalmouseover"),
            _ => None,
        };
        if let Some(event_name) = event_name {
            let mut global_event = event.clone();
            global_event.set_name(event_name);
            global_events.push(global_event);
        }
    }

    // Propagate events from the top to the bottom
    for layer_num in layers_nums {
        let layer = layers.layers.get(layer_num).unwrap();

        for element in layer.values() {
            'events: for event in events.iter() {
                let area = &element.node_area;
                if let FreyaEvent::Keyboard { name, .. } = event {
                    let event_data = (element.clone(), event.clone());
                    calculated_events
                        .entry(*name)
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
                        let cursor_is_inside = area.contains(cursor.to_f32());

                        // Make sure the cursor is inside the node area
                        if cursor_is_inside {
                            let viewports = viewports_collection.get(element.get_id());

                            // Make sure the cursor is inside all the applicable viewports from the element
                            if let Some((_, viewports)) = viewports {
                                for viewport_id in viewports {
                                    let viewport = viewports_collection.get(viewport_id).unwrap().0;
                                    if let Some(viewport) = viewport {
                                        if !viewport.contains(cursor.to_f32()) {
                                            continue 'events;
                                        }
                                    }
                                }
                            }

                            let event_data = (element.clone(), event.clone());

                            calculated_events
                                .entry(*name)
                                .or_insert_with(Vec::new)
                                .push(event_data);
                        }
                    }
                }
            }
        }
    }

    (calculated_events, global_events)
}

// Calculate events that can actually be triggered
fn calculate_events_listeners(
    calculated_events: &mut NodesEvents,
    dom: &FreyaDOM,
    event_emitter: &EventEmitter,
    scale_factor: f64,
) -> Vec<DomEvent> {
    let mut new_events = Vec::new();

    for (event_name, event_nodes) in calculated_events.iter_mut() {
        let listeners = dom.dom().get_listening_sorted(event_name);

        let mut found_nodes: Vec<(&RenderData, &FreyaEvent)> = Vec::new();

        'event_nodes: for (node, request) in event_nodes.iter() {
            for listener in &listeners {
                if listener.node_data.node_id == *node.get_id() {
                    let dioxus_node = if let Some(node) = node.get_node(dom) {
                        node
                    } else {
                        continue 'event_nodes;
                    };

                    if dioxus_node.state.style.background != Color::TRANSPARENT
                        && event_name == &"wheel"
                    {
                        break 'event_nodes;
                    }

                    if dioxus_node.state.style.background != Color::TRANSPARENT
                        && (event_name == &"click"
                            || event_name == &"touchstart"
                            || event_name == &"touchend")
                    {
                        found_nodes.clear();
                    }

                    if event_name == &"mouseover"
                        || event_name == &"click"
                        || event_name == &"keydown"
                        || event_name == &"keyup"
                        || event_name == &"touchcancel"
                        || event_name == &"touchend"
                        || event_name == &"touchmove"
                        || event_name == &"touchstart"
                    {
                        // Mouseover and click events can be stackked
                        found_nodes.push((node, request))
                    } else {
                        found_nodes = vec![(node, request)]
                    }
                }
            }
        }

        for (node, request_event) in found_nodes {
            let event = DomEvent::from_freya_event(
                event_name,
                node.element_id.unwrap(),
                request_event,
                Some(node.node_area),
                scale_factor,
            );
            new_events.push(event.clone());
            event_emitter.send(event).unwrap();
        }
    }

    new_events
}

/// Calculate global events to be triggered
fn calculate_global_events_listeners(
    global_events: Vec<FreyaEvent>,
    dom: &FreyaDOM,
    event_emitter: &EventEmitter,
    scale_factor: f64,
) {
    for global_event in global_events {
        let event_name = global_event.get_name();
        let listeners = dom.dom().get_listening_sorted(event_name);

        for listener in listeners {
            let event = DomEvent::from_freya_event(
                event_name,
                listener.node_data.element_id.unwrap(),
                &global_event,
                None,
                scale_factor,
            );
            event_emitter.send(event).unwrap();
        }
    }
}

/// Process the layout of the DOM
pub fn process_layout(
    dom: &FreyaDOM,
    area: Area,
    font_collection: &mut FontCollection,
    scale_factor: f32,
) -> (Layers, ViewportsCollection) {
    let mut layers = Layers::default();

    {
        let root = dom.dom().index(NodeId(0));
        let mut remaining_area = area;
        let mut root_node_measurer = NodeLayoutMeasurer::new(
            root,
            &mut remaining_area,
            area,
            dom,
            &mut layers,
            0,
            font_collection,
        );
        root_node_measurer.measure_area(true, scale_factor);
    }

    let mut layers_nums: Vec<&i16> = layers.layers.keys().collect();

    // Order the layers from top to bottom
    layers_nums.sort();

    let viewports_collection = calculate_viewports(&layers_nums, &layers, dom);

    (layers, viewports_collection)
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

    let (mut node_events, global_events) =
        calculate_node_events(&layers_nums, layers, events, viewports_collection);

    let emitted_events =
        calculate_events_listeners(&mut node_events, dom, event_emitter, scale_factor);

    calculate_global_events_listeners(global_events, dom, event_emitter, scale_factor);

    let new_processed_events = events_processor.process_events_batch(emitted_events, node_events);

    for event in new_processed_events {
        event_emitter.send(event).unwrap();
    }

    events.clear();
}

/// Render the layout
pub fn process_render<HookOptions>(
    viewports_collection: &ViewportsCollection,
    dom: &FreyaDOM,
    font_collection: &mut FontCollection,
    layers: &Layers,
    hook_options: &mut HookOptions,
    render_hook: impl Fn(
        &FreyaDOM,
        &RenderData,
        &mut FontCollection,
        &ViewportsCollection,
        &mut HookOptions,
    ),
) {
    let mut layers_nums: Vec<&i16> = layers.layers.keys().collect();

    // Order the layers from top to bottom
    layers_nums.sort();

    // Render all the layers from the bottom to the top
    for layer_num in &layers_nums {
        let layer = layers.layers.get(layer_num).unwrap();
        'elements: for dom_element in layer.values() {
            let viewports = viewports_collection.get(dom_element.get_id());

            // Skip elements that are completely out of any their parent's viewport
            if let Some((_, viewports)) = viewports {
                for viewport_id in viewports {
                    let viewport = viewports_collection.get(viewport_id).unwrap().0;
                    if let Some(viewport) = viewport {
                        if !viewport.intersects(&dom_element.node_area) {
                            continue 'elements;
                        }
                    }
                }
            }

            // Render the element
            render_hook(
                dom,
                dom_element,
                font_collection,
                viewports_collection,
                hook_options,
            )
        }
    }
}
