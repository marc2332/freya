use dioxus_native_core::prelude::{ElementNode, NodeImmutableDioxusExt};
use dioxus_native_core::real_dom::NodeImmutable;
use dioxus_native_core::{node::NodeType, NodeId};
use freya_common::Area;
use freya_dom::{DioxusNodeResolver, FreyaDOM, SkiaMeasurer};
use freya_layout::Layers;

use freya_node_state::Style;
use rustc_hash::FxHashMap;
use skia_safe::{textlayout::FontCollection, Color};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub mod events;
pub mod node;

use events::{DomEvent, EventsProcessor, FreyaEvent};

pub type EventEmitter = UnboundedSender<DomEvent>;
pub type EventReceiver = UnboundedReceiver<DomEvent>;
pub type EventsQueue = Vec<FreyaEvent>;
pub type ViewportsCollection = FxHashMap<NodeId, (Option<torin::Area>, Vec<NodeId>)>;
pub type NodesEvents<'a> = FxHashMap<&'a str, Vec<(NodeId, FreyaEvent)>>;

// Calculate all the applicable viewports for the given nodes
pub fn calculate_viewports(
    layers_nums: &[&i16],
    layers: &Layers,
    rdom: &FreyaDOM,
) -> ViewportsCollection {
    let mut viewports_collection = FxHashMap::default();
    let layout = rdom.layout();

    for layer_num in layers_nums {
        let layer = layers.layers.get(layer_num).unwrap();
        for node_id in layer {
            let node = rdom.dom().get(*node_id);
            let node_areas = layout.get_size(*node_id);

            if let Some((node, node_areas)) = node.zip(node_areas) {
                let node_type = &*node.node_type();

                if let NodeType::Element(ElementNode { tag, .. }) = node_type {
                    if tag == "container" {
                        viewports_collection
                            .entry(*node_id)
                            .or_insert_with(|| (None, Vec::new()))
                            .0 = Some(node_areas.area);
                    }
                    for child in node.children() {
                        if viewports_collection.contains_key(node_id) {
                            let mut inherited_viewports =
                                viewports_collection.get(node_id).unwrap().1.clone();

                            inherited_viewports.push(*node_id);

                            viewports_collection.insert(child.id(), (None, inherited_viewports));
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
    rdom: &FreyaDOM,
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

    let layout = rdom.layout();

    // Propagate events from the top to the bottom
    for layer_num in layers_nums {
        let layer = layers.layers.get(layer_num).unwrap();

        for node_id in layer {
            let areas = layout.get_size(*node_id);
            if let Some(areas) = areas {
                'events: for event in events.iter() {
                    if let FreyaEvent::Keyboard { name, .. } = event {
                        let event_data = (*node_id, event.clone());
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

        let mut found_nodes: Vec<(&NodeId, &FreyaEvent)> = Vec::new();

        'event_nodes: for (node_id, request) in event_nodes.iter() {
            for listener in &listeners {
                if listener.id() == *node_id {
                    let node_ref = dom.dom().get(*node_id);

                    let node_ref = if let Some(node_ref) = node_ref {
                        node_ref
                    } else {
                        continue 'event_nodes;
                    };

                    let Style { background, .. } = &*node_ref.get::<Style>().unwrap();
                    if background != &Color::TRANSPARENT && event_name == &"wheel" {
                        break 'event_nodes;
                    }

                    if background != &Color::TRANSPARENT && event_name == &"wheel" {
                        break 'event_nodes;
                    }

                    if background != &Color::TRANSPARENT
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
                        found_nodes.push((node_id, request))
                    } else {
                        found_nodes = vec![(node_id, request)]
                    }
                }
            }
        }

        for (node_id, request_event) in found_nodes {
            let areas = dom.layout().get_size(*node_id).unwrap().clone();
            let node_ref = dom.dom().get(*node_id).unwrap();
            let element_id = node_ref.mounted_id().unwrap();
            let event = DomEvent::from_freya_event(
                event_name,
                element_id,
                request_event,
                Some(areas.area),
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
            let element_id = listener.mounted_id().unwrap();
            let event = DomEvent::from_freya_event(
                event_name,
                element_id,
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
    _scale_factor: f32,
) -> (Layers, ViewportsCollection) {
    let rdom = dom.dom();
    let node_resolver = DioxusNodeResolver::new(rdom);
    let skia_measurer = SkiaMeasurer::new(rdom, font_collection);

    let root_id = dom.dom().root_id();

    dom.layout()
        .measure(root_id, area, &mut Some(skia_measurer), &node_resolver);

    let layers = Layers::new(rdom, &dom.layout());

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
        calculate_node_events(&layers_nums, layers, events, viewports_collection, dom);

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
        &NodeId,
        &Area,
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
        'elements: for node_id in layer {
            let viewports = viewports_collection.get(node_id);
            let layout = dom.layout();
            let areas = layout.get_size(*node_id);

            if let Some(areas) = areas {
                // Skip elements that are completely out of any their parent's viewport
                if let Some((_, viewports)) = viewports {
                    for viewport_id in viewports {
                        let viewport = viewports_collection.get(viewport_id).unwrap().0;
                        if let Some(viewport) = viewport {
                            if !viewport.intersects(&areas.area) {
                                continue 'elements;
                            }
                        }
                    }
                }

                // Render the element
                render_hook(
                    dom,
                    node_id,
                    &areas.area,
                    font_collection,
                    viewports_collection,
                    hook_options,
                )
            }
        }
    }
}
