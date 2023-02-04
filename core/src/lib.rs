use dioxus_native_core::{node::NodeType, real_dom::RealDom, NodeId};
use euclid::{Length, Point2D};
use freya_common::{LayoutMemorizer, NodeArea};
use freya_elements::events_data::{KeyboardData, MouseData, WheelData};
use freya_layout::NodeLayoutMeasurer;
use freya_layout::{Layers, RenderData};
use freya_node_state::{CustomAttributeValues, NodeState};
use rustc_hash::FxHashMap;
use skia_safe::{textlayout::FontCollection, Color};
use std::{
    ops::Index,
    sync::{Arc, Mutex},
};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub mod events;

use events::{DomEvent, DomEventData, EventsProcessor, FreyaEvent};

pub type SharedRealDOM = Arc<Mutex<RealDom<NodeState, CustomAttributeValues>>>;
pub type EventEmitter = UnboundedSender<DomEvent>;
pub type EventReceiver = UnboundedReceiver<DomEvent>;
pub type SharedLayoutMemorizer = Arc<Mutex<LayoutMemorizer>>;
pub type SharedFreyaEvents = Arc<Mutex<Vec<FreyaEvent>>>;
pub type ViewportsCollection = FxHashMap<NodeId, (Option<NodeArea>, Vec<NodeId>)>;
pub type NodesEvents<'a> = FxHashMap<&'a str, Vec<(RenderData, FreyaEvent)>>;

// Calculate all the applicable viewports for the given nodes
pub fn calculate_viewports(layers_nums: &[&i16], layers: &Layers) -> ViewportsCollection {
    let mut viewports_collection = FxHashMap::default();

    for layer_num in layers_nums {
        let layer = layers.layers.get(layer_num).unwrap();
        for dom_element in layer.values() {
            if let NodeType::Element { tag, .. } = &dom_element.get_type() {
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
    viewports_collection
}

// Calculate possible events in nodes considering their viewports
pub fn calculate_node_events<'a>(
    layers_nums: &[&i16],
    layers: &Layers,
    freya_events: SharedFreyaEvents,
    viewports_collection: ViewportsCollection,
) -> (NodesEvents<'a>, Vec<FreyaEvent>) {
    let mut calculated_events = FxHashMap::default();
    let mut global_events = Vec::default();

    // Propagate events from the top to the bottom
    for layer_num in layers_nums {
        let layer = layers.layers.get(layer_num).unwrap();

        for element in layer.values() {
            let events = freya_events.lock().unwrap();

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
    dom: &SharedRealDOM,
    event_emitter: &EventEmitter,
) -> Vec<DomEvent> {
    let mut new_events = Vec::new();

    for (event_name, event_nodes) in calculated_events.iter_mut() {
        let dom = dom.lock().unwrap();
        let listeners = dom.get_listening_sorted(event_name);

        let mut found_nodes: Vec<(&RenderData, &FreyaEvent)> = Vec::new();

        'event_nodes: for (node, request) in event_nodes.iter() {
            for listener in &listeners {
                if listener.node_data.node_id == *node.get_id() {
                    if node.get_state().style.background != Color::TRANSPARENT
                        && event_name == &"wheel"
                    {
                        break 'event_nodes;
                    }

                    if node.get_state().style.background != Color::TRANSPARENT
                        && event_name == &"click"
                    {
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
            let event = match request {
                FreyaEvent::Mouse { cursor, button, .. } => DomEvent {
                    element_id: node.element_id.unwrap(),
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
                    element_id: node.element_id.unwrap(),
                    name: event_name.to_string(),
                    data: DomEventData::Wheel(WheelData::new(scroll.0, scroll.1)),
                },
                FreyaEvent::Keyboard { key, code, .. } => DomEvent {
                    element_id: node.element_id.unwrap(),
                    name: event_name.to_string(),
                    data: DomEventData::Keyboard(KeyboardData::new(key.clone(), *code)),
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
    dom: &SharedRealDOM,
    event_emitter: &EventEmitter,
) {
    for global_event in global_events {
        let event_name = global_event.get_name();
        let dom = dom.lock().unwrap();
        let listeners = dom.get_listening_sorted(event_name);

        for listener in listeners {
            let event = match global_event {
                FreyaEvent::Mouse { cursor, button, .. } => DomEvent {
                    element_id: listener.node_data.element_id.unwrap(),
                    name: event_name.to_string(),
                    data: DomEventData::Mouse(MouseData::new(
                        Point2D::from_lengths(Length::new(cursor.0), Length::new(cursor.1)),
                        Point2D::from_lengths(Length::new(cursor.0), Length::new(cursor.1)),
                        button,
                    )),
                },
                FreyaEvent::Wheel { scroll, .. } => DomEvent {
                    element_id: listener.node_data.element_id.unwrap(),
                    name: event_name.to_string(),
                    data: DomEventData::Wheel(WheelData::new(scroll.0, scroll.1)),
                },
                FreyaEvent::Keyboard { ref key, code, .. } => DomEvent {
                    element_id: listener.node_data.element_id.unwrap(),
                    name: event_name.to_string(),
                    data: DomEventData::Keyboard(KeyboardData::new(key.clone(), code)),
                },
            };
            event_emitter.send(event).unwrap();
        }
    }
}

/// 1. Measure the nodes layouts
/// 2. Organize the nodes layouts in layers
/// 3. Calculate all the nodes viewports
/// 4. Call the render to paint
/// 5. Calculate what events must be triggered
#[allow(clippy::too_many_arguments)]
pub fn process_work<HookOptions>(
    dom: &SharedRealDOM,
    area: NodeArea,
    freya_events: SharedFreyaEvents,
    event_emitter: &EventEmitter,
    font_collection: &mut FontCollection,
    events_processor: &mut EventsProcessor,
    manager: &SharedLayoutMemorizer,
    hook_options: &mut HookOptions,
    render_hook: impl Fn(
        &SharedRealDOM,
        &RenderData,
        &mut FontCollection,
        &ViewportsCollection,
        &mut HookOptions,
    ),
) {
    let layers = &mut Layers::default();

    {
        let root = dom.lock().unwrap().index(NodeId(0)).clone();
        let mut remaining_area = area;
        let mut root_node_measurer = NodeLayoutMeasurer::new(
            root,
            &mut remaining_area,
            area,
            dom,
            layers,
            0,
            font_collection,
            manager,
            true
        );
        root_node_measurer.measure_area(true);
    }

    #[cfg(debug_assertions)]
    {
        use tracing::info;
        let dirty_nodes_counter = manager.lock().unwrap().dirty_nodes_counter;
        if dirty_nodes_counter > 0 {
            let nodes = manager.lock().unwrap().nodes.len();
            info!("Measured layout of {}/{}", dirty_nodes_counter, nodes);
            manager.lock().unwrap().dirty_nodes_counter = 0;
        }
    }

    let mut layers_nums: Vec<&i16> = layers.layers.keys().collect();

    // Order the layers from top to bottom
    layers_nums.sort();

    let viewports_collection = calculate_viewports(&layers_nums, layers);

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
                &viewports_collection,
                hook_options,
            )
        }
    }

    let (mut node_events, global_events) = calculate_node_events(
        &layers_nums,
        layers,
        freya_events.clone(),
        viewports_collection,
    );

    let emitted_events = calculate_events_listeners(&mut node_events, dom, event_emitter);

    calculate_global_events_listeners(global_events, dom, event_emitter);

    let new_processed_events = events_processor.process_events_batch(emitted_events, node_events);

    for event in new_processed_events {
        event_emitter.send(event).unwrap();
    }

    freya_events.lock().unwrap().clear();
}
