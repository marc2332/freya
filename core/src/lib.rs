use dioxus_native_core::prelude::{ElementNode, NodeImmutableDioxusExt};
use dioxus_native_core::real_dom::NodeImmutable;
use dioxus_native_core::{node::NodeType, NodeId};
use euclid::{Length, Point2D};
use freya_common::NodeArea;
use freya_elements::events_data::{KeyboardData, MouseData, WheelData};
use freya_layout::{DioxusDOM, NodeLayoutMeasurer};
use freya_layout::{Layers, RenderData};

use freya_node_state::Style;
use rustc_hash::FxHashMap;
use skia_safe::{textlayout::FontCollection, Color};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub mod dom;
pub mod events;

use events::{DomEvent, DomEventData, EventsProcessor, FreyaEvent};

pub type EventEmitter = UnboundedSender<DomEvent>;
pub type EventReceiver = UnboundedReceiver<DomEvent>;
pub type EventsQueue = Vec<FreyaEvent>;
pub type ViewportsCollection = FxHashMap<NodeId, (Option<NodeArea>, Vec<NodeId>)>;
pub type NodesEvents<'a> = FxHashMap<&'a str, Vec<(RenderData, FreyaEvent)>>;

// Calculate all the applicable viewports for the given nodes
pub fn calculate_viewports(
    layers_nums: &[&i16],
    layers: &Layers,
    rdom: &DioxusDOM,
) -> ViewportsCollection {
    let mut viewports_collection = FxHashMap::default();

    for layer_num in layers_nums {
        let layer = layers.layers.get(layer_num).unwrap();
        for dom_element in layer.values() {
            let node = dom_element.get_node(rdom);
            let node_type = &*node.node_type();
            if let NodeType::Element(ElementNode { tag, .. }) = node_type {
                if tag == "container" {
                    viewports_collection
                        .entry(*dom_element.get_id())
                        .or_insert_with(|| (None, Vec::new()))
                        .0 = Some(dom_element.node_area);
                }
                for child in node.children() {
                    if viewports_collection.contains_key(dom_element.get_id()) {
                        let mut inherited_viewports = viewports_collection
                            .get(dom_element.get_id())
                            .unwrap()
                            .1
                            .clone();

                        inherited_viewports.push(*dom_element.get_id());

                        viewports_collection.insert(child.id(), (None, inherited_viewports));
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
                        _ => None,
                    };
                    if let Some((name, cursor)) = data {
                        let ((x, y), (x2, y2)) = area.get_rect();

                        let cursor_is_inside =
                            cursor.0 > x && cursor.0 < x2 && cursor.1 > y && cursor.1 < y2;

                        if name == &"click" {
                            let mut global_event = event.clone();
                            global_event.set_name("globalclick");
                            global_events.push(global_event);
                        }

                        // Make sure the cursor is inside the node area
                        if cursor_is_inside {
                            let viewports = viewports_collection.get(element.get_id());

                            // Make sure the cursor is inside all the applicable viewports from the element
                            if let Some((_, viewports)) = viewports {
                                for viewport_id in viewports {
                                    let viewport = viewports_collection.get(viewport_id).unwrap().0;
                                    if let Some(viewport) = viewport {
                                        if viewport.is_point_outside(*cursor) {
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
    rdom: &DioxusDOM,
    event_emitter: &EventEmitter,
) -> Vec<DomEvent> {
    let mut new_events = Vec::new();

    for (event_name, event_nodes) in calculated_events.iter_mut() {
        let listeners = rdom.get_listening_sorted(event_name);

        let mut found_nodes: Vec<(&RenderData, &FreyaEvent)> = Vec::new();

        'event_nodes: for (node, request) in event_nodes.iter() {
            for listener in &listeners {
                if listener.id() == *node.get_id() {
                    let node_ref = node.get_node(rdom);

                    let Style { background, .. } = &*node_ref.get::<Style>().unwrap();
                    if background != &Color::TRANSPARENT && event_name == &"wheel" {
                        break 'event_nodes;
                    }

                    if background != &Color::TRANSPARENT && event_name == &"click" {
                        found_nodes.clear();
                    }

                    if event_name == &"mouseover"
                        || event_name == &"click"
                        || event_name == &"keydown"
                        || event_name == &"keyup"
                    {
                        // Mouseover and click events can be stackked
                        found_nodes.push((node, request))
                    } else {
                        found_nodes = vec![(node, request)]
                    }
                }
            }
        }

        for (node, request) in found_nodes {
            let node_ref = rdom.get(node.node_id).unwrap();
            let element_id = node_ref.mounted_id().unwrap();
            let event = match request {
                FreyaEvent::Mouse { cursor, button, .. } => DomEvent {
                    element_id,
                    name: event_name.to_string(),
                    data: DomEventData::Mouse(MouseData::new(
                        Point2D::from_lengths(Length::new(cursor.0), Length::new(cursor.1)),
                        Point2D::from_lengths(
                            Length::new(cursor.0 - node.node_area.x as f64),
                            Length::new(cursor.1 - node.node_area.y as f64),
                        ),
                        *button,
                    )),
                },
                FreyaEvent::Wheel { scroll, .. } => DomEvent {
                    element_id,
                    name: event_name.to_string(),
                    data: DomEventData::Wheel(WheelData::new(scroll.0, scroll.1)),
                },
                FreyaEvent::Keyboard {
                    key,
                    code,
                    modifiers,
                    ..
                } => DomEvent {
                    element_id,
                    name: event_name.to_string(),
                    data: DomEventData::Keyboard(KeyboardData::new(key.clone(), *code, *modifiers)),
                },
            };

            new_events.push(event.clone());
            event_emitter.send(event).unwrap();
        }
    }

    new_events
}

/// Calculate global events to be triggered
fn calculate_global_events_listeners(
    global_events: Vec<FreyaEvent>,
    dom: &DioxusDOM,
    event_emitter: &EventEmitter,
) {
    for global_event in global_events {
        let event_name = global_event.get_name();
        let listeners = dom.get_listening_sorted(event_name);

        for listener in listeners {
            let element_id = listener.mounted_id().unwrap();
            let event = match global_event {
                FreyaEvent::Mouse { cursor, button, .. } => DomEvent {
                    element_id,
                    name: event_name.to_string(),
                    data: DomEventData::Mouse(MouseData::new(
                        Point2D::from_lengths(Length::new(cursor.0), Length::new(cursor.1)),
                        Point2D::from_lengths(Length::new(cursor.0), Length::new(cursor.1)),
                        button,
                    )),
                },
                FreyaEvent::Wheel { scroll, .. } => DomEvent {
                    element_id,
                    name: event_name.to_string(),
                    data: DomEventData::Wheel(WheelData::new(scroll.0, scroll.1)),
                },
                FreyaEvent::Keyboard {
                    ref key,
                    code,
                    modifiers,
                    ..
                } => DomEvent {
                    element_id,
                    name: event_name.to_string(),
                    data: DomEventData::Keyboard(KeyboardData::new(key.clone(), code, modifiers)),
                },
            };
            event_emitter.send(event).unwrap();
        }
    }
}

/// Process the layout of the DOM
pub fn process_layout(
    dom: &DioxusDOM,
    area: NodeArea,
    font_collection: &mut FontCollection,
) -> (Layers, ViewportsCollection) {
    let mut layers = Layers::default();

    {
        let root = dom.get(NodeId::new_from_index_and_gen(0, 0)).unwrap();
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
        root_node_measurer.measure_area(true);
    }

    let mut layers_nums: Vec<&i16> = layers.layers.keys().collect();

    // Order the layers from top to bottom
    layers_nums.sort();

    let viewports_collection = calculate_viewports(&layers_nums, &layers, dom);

    (layers, viewports_collection)
}

/// Process the events and emit them to the DOM
pub fn process_events(
    dom: &DioxusDOM,
    layers: &Layers,
    events: &mut EventsQueue,
    event_emitter: &EventEmitter,
    events_processor: &mut EventsProcessor,
    viewports_collection: &ViewportsCollection,
) {
    let mut layers_nums: Vec<&i16> = layers.layers.keys().collect();

    // Order the layers from top to bottom
    layers_nums.sort();

    let (mut node_events, global_events) =
        calculate_node_events(&layers_nums, layers, events, viewports_collection);

    let emitted_events = calculate_events_listeners(&mut node_events, dom, event_emitter);

    calculate_global_events_listeners(global_events, dom, event_emitter);

    let new_processed_events = events_processor.process_events_batch(emitted_events, node_events);

    for event in new_processed_events {
        event_emitter.send(event).unwrap();
    }

    events.clear();
}

/// Render the layout
pub fn process_render<HookOptions>(
    viewports_collection: &ViewportsCollection,
    dom: &DioxusDOM,
    font_collection: &mut FontCollection,
    layers: &Layers,
    hook_options: &mut HookOptions,
    render_hook: impl Fn(
        &DioxusDOM,
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
                        if viewport.is_area_outside(dom_element.node_area) {
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
