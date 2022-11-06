use dioxus_core::{ElementId, EventPriority, SchedulerMsg, UserEvent};
use dioxus_html::{
    geometry::{
        euclid::{Length, Point2D},
        Coordinates, PixelsVector, WheelDelta,
    },
    input_data::{keyboard_types::Modifiers, MouseButton},
    on::{MouseData, WheelData},
};
use dioxus_native_core::real_dom::{Node, NodeType};
use enumset::enum_set;
use freya_elements::events::KeyboardData;
use freya_layers::{Layers, NodeData, RenderData};
use freya_layout::measure_node_layout;
use freya_layout_common::NodeArea;
use freya_node_state::NodeState;
use rustc_hash::FxHashMap;
use skia_safe::{textlayout::FontCollection, Canvas, Color};
use std::{ops::Index, sync::Arc};

use crate::{
    events_processor::EventsProcessor, renderer::render_skia, FreyaEvent, SafeDOM,
    SafeEventEmitter, SafeFreyaEvents, SafeLayoutManager,
};

pub type ViewportsCollection = FxHashMap<ElementId, (Option<NodeArea>, Vec<ElementId>)>;

/// The Work Loop has a few jobs:
/// - Measure the nodes layouts
/// - Organize the nodes layouts in layers
/// - Calculate all the nodes viewports
/// - Paint the nodes
/// - Calculate what events must be triggered
#[allow(clippy::too_many_arguments)]
pub(crate) fn work_loop(
    mut dom: &SafeDOM,
    mut canvas: &mut Canvas,
    area: NodeArea,
    freya_events: SafeFreyaEvents,
    event_emitter: &SafeEventEmitter,
    font_collection: &mut FontCollection,
    events_processor: &mut EventsProcessor,
    manager: &SafeLayoutManager,
) {
    let root: Node<NodeState> = {
        let dom = dom.lock().unwrap();
        dom.index(ElementId(0)).clone()
    };

    let layers = &mut Layers::default();

    measure_node_layout(
        &NodeData { node: root },
        area,
        area,
        &mut dom,
        layers,
        |node_id, dom| {
            let child = {
                let dom = dom.lock().unwrap();
                dom.index(*node_id).clone()
            };

            Some(NodeData { node: child })
        },
        0,
        font_collection,
        manager,
        true,
    );

    let mut layers_nums: Vec<&i16> = layers.layers.keys().collect();

    // From top to bottom
    layers_nums.sort();

    // Calculate all the applicable viewports for the given elements
    let mut viewports_collection: ViewportsCollection = FxHashMap::default();

    for layer_num in &layers_nums {
        let layer = layers.layers.get(layer_num).unwrap();
        for element in layer.values() {
            if let NodeType::Element { tag, children, .. } = &element.node_type {
                if tag == "container" {
                    viewports_collection
                        .entry(element.node_id)
                        .or_insert_with(|| (None, Vec::new()))
                        .0 = Some(element.node_area);
                }
                for child in children {
                    if viewports_collection.contains_key(&element.node_id) {
                        let mut inherited_viewports = viewports_collection
                            .get(&element.node_id)
                            .unwrap()
                            .1
                            .clone();

                        inherited_viewports.push(element.node_id);

                        viewports_collection.insert(*child, (None, inherited_viewports));
                    }
                }
            }
        }
    }

    // Render all the layers from the bottom to the top
    for layer_num in &layers_nums {
        let layer = layers.layers.get(layer_num).unwrap();
        'elements: for element in layer.values() {
            let viewports = viewports_collection.get(&element.node_id);

            // Skip elements that are totally out of some their parent's viewport
            if let Some((_, viewports)) = viewports {
                for viewport_id in viewports {
                    let viewport = viewports_collection.get(viewport_id).unwrap().0;
                    if let Some(viewport) = viewport {
                        if viewport.is_area_outside(element.node_area) {
                            continue 'elements;
                        }
                    }
                }
            }

            canvas.save();
            render_skia(
                &mut dom,
                &mut canvas,
                element,
                font_collection,
                &viewports_collection,
            );
            canvas.restore();
        }
    }

    // Calculated events are those that match considering their viewports
    let mut calculated_events: FxHashMap<&'static str, Vec<(RenderData, FreyaEvent)>> =
        FxHashMap::default();

    // Propagate events from the top to the bottom
    for layer_num in &layers_nums {
        let layer = layers.layers.get(layer_num).unwrap();

        for element in layer.values() {
            let events = freya_events.lock().unwrap();

            'events: for event in events.iter() {
                let area = &element.node_area;
                if let FreyaEvent::Keyboard { name, .. } = event {
                    let event_data = (element.clone(), event.clone());
                    calculated_events
                        .entry(name)
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

                        // Make sure the cursor is inside the node area
                        if cursor_is_inside {
                            let viewports = viewports_collection.get(&element.node_id);

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
                                .entry(name)
                                .or_insert_with(|| vec![event_data.clone()])
                                .push(event_data);
                        }
                    }
                }
            }
        }
    }

    let mut new_events: Vec<UserEvent> = Vec::new();

    // Calculate what event listeners can actually be triggered
    for (event_name, event_nodes) in calculated_events.iter_mut() {
        let dom = dom.lock().unwrap();
        let listeners = dom.get_listening_sorted(event_name);

        let mut found_nodes: Vec<(&RenderData, &FreyaEvent)> = Vec::new();

        'event_nodes: for (node, request) in event_nodes.iter() {
            for listener in &listeners {
                if listener.id == node.node_id {
                    if node.node_state.style.background != Color::TRANSPARENT
                        && event_name == &"wheel"
                    {
                        break 'event_nodes;
                    }

                    if node.node_state.style.background != Color::TRANSPARENT
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
                FreyaEvent::Mouse { cursor, .. } => Some(UserEvent {
                    scope_id: None,
                    priority: EventPriority::Medium,
                    element: Some(node.node_id),
                    name: event_name,
                    bubbles: false,
                    data: Arc::new(MouseData::new(
                        Coordinates::new(
                            Point2D::from_lengths(Length::new(cursor.0), Length::new(cursor.1)),
                            Point2D::default(),
                            Point2D::from_lengths(
                                Length::new(cursor.0 - node.node_area.x as f64),
                                Length::new(cursor.1 - node.node_area.y as f64),
                            ),
                            Point2D::default(),
                        ),
                        Some(MouseButton::Primary),
                        enum_set! {MouseButton::Primary},
                        Modifiers::empty(),
                    )),
                }),
                FreyaEvent::Wheel { scroll, .. } => Some(UserEvent {
                    scope_id: None,
                    priority: EventPriority::Medium,
                    element: Some(node.node_id),
                    name: event_name,
                    bubbles: false,
                    data: Arc::new(WheelData::new(WheelDelta::Pixels(PixelsVector::new(
                        scroll.0, scroll.1, 0.0,
                    )))),
                }),
                FreyaEvent::Keyboard { name, code } => Some(UserEvent {
                    scope_id: None,
                    priority: EventPriority::Medium,
                    element: Some(node.node_id),
                    name,
                    bubbles: false,
                    data: Arc::new(KeyboardData::new(code.clone())),
                }),
            };
            if let Some(event) = event {
                new_events.push(event.clone());

                event_emitter
                    .lock()
                    .unwrap()
                    .as_ref()
                    .unwrap()
                    .unbounded_send(SchedulerMsg::Event(event))
                    .unwrap();
            }
        }
    }

    // Calculate new events by processing the old and new
    let new_processed_events = events_processor.process_events_batch(new_events, calculated_events);

    for event in new_processed_events {
        event_emitter
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .unbounded_send(SchedulerMsg::Event(event))
            .unwrap();
    }

    freya_events.lock().unwrap().clear();
}
