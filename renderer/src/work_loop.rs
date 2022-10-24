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
use freya_node_state::node::NodeState;
use skia_safe::{textlayout::FontCollection, Canvas, Color};
use std::{collections::HashMap, ops::Index, sync::Arc};

use crate::{
    events_processor::EventsProcessor, renderer::render_skia, FreyaEvents, SafeDOM,
    SafeEventEmitter, SafeFreyaEvents, SafeLayoutManager,
};

/// The Work Loop has a few jobs:
/// - Measure the nodes layouts
/// - Organize the nodes layouts in layers
/// - Paint the nodes
/// - Calculate what events must be triggered
pub fn work_loop(
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
        area.clone(),
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
    layers_nums.sort_by(|a, b| a.cmp(b));

    // Calculate all the applicable viewports for the given elements
    let mut calculated_viewports: HashMap<ElementId, Vec<NodeArea>> = HashMap::new();

    for layer_num in &layers_nums {
        let layer = layers.layers.get(layer_num).unwrap();
        for element in layer.values() {
            match &element.node_type {
                NodeType::Element { tag, children, .. } => {
                    for child in children {
                        if !calculated_viewports.contains_key(&child) {
                            calculated_viewports.insert(*child, Vec::new());
                        }
                        if calculated_viewports.contains_key(&element.node_id) {
                            calculated_viewports.insert(
                                *child,
                                calculated_viewports.get(&element.node_id).unwrap().clone(),
                            );
                        }
                        if tag == "container" {
                            calculated_viewports
                                .get_mut(&child)
                                .unwrap()
                                .push(element.node_area.clone());
                        }
                    }
                }
                NodeType::Text { .. } => {}
                _ => {}
            }
        }
    }

    // Render all the layers from the bottom to the top
    for layer_num in &layers_nums {
        let layer = layers.layers.get(layer_num).unwrap();
        'elements: for element in layer.values() {
            let viewports = calculated_viewports.get(&element.node_id);

            if let Some(viewports) = viewports {
                for viewport in viewports {
                    if element.node_area.x + element.node_area.width < viewport.x
                        || element.node_area.y + element.node_area.height < viewport.y
                        || element.node_area.x > viewport.x + viewport.width
                        || element.node_area.y > viewport.y + viewport.height
                    {
                        continue 'elements;
                    }
                }
            }

            canvas.save();
            render_skia(
                &mut dom,
                &mut canvas,
                &element,
                font_collection,
                &calculated_viewports
                    .get(&element.node_id)
                    .unwrap_or(&Vec::new()),
            );
            canvas.restore();
        }
    }

    // Calculated events are those that match considering their viewports
    let mut calculated_events: HashMap<&'static str, Vec<(RenderData, FreyaEvents)>> =
        HashMap::new();

    // Propagate events from the top to the bottom
    for layer_num in &layers_nums {
        let layer = layers.layers.get(layer_num).unwrap();

        for element in layer.values() {
            let events = freya_events.lock().unwrap();

            for event in events.iter() {
                let area = &element.node_area;
                if let FreyaEvents::KeyboardEvent { name, .. } = event {
                    if !calculated_events.contains_key(name) {
                        calculated_events.insert(name, vec![(element.clone(), event.clone())]);
                    } else {
                        calculated_events
                            .get_mut(name)
                            .unwrap()
                            .push((element.clone(), event.clone()));
                    }
                } else {
                    let data = match event {
                        FreyaEvents::MouseEvent { name, cursor, .. } => Some((name, cursor)),
                        FreyaEvents::WheelEvent { name, cursor, .. } => Some((name, cursor)),
                        _ => None,
                    };
                    if let Some((name, cursor)) = data {
                        let x = area.x as f64;
                        let y = area.y as f64;
                        let width = (area.x + area.width) as f64;
                        let height = (area.y + area.height) as f64;

                        let mut visible = true;

                        // Make sure the cursor is inside all the applicable viewports from the element
                        for viewport in calculated_viewports
                            .get(&element.node_id)
                            .unwrap_or(&Vec::new())
                        {
                            if cursor.0 < viewport.x as f64
                                || cursor.0 > (viewport.x + viewport.width) as f64
                                || cursor.1 < viewport.y as f64
                                || cursor.1 > (viewport.y + viewport.height) as f64
                            {
                                visible = false;
                            }
                        }

                        // Make sure the cursor is inside the node area
                        if visible
                            && cursor.0 > x
                            && cursor.0 < width
                            && cursor.1 > y
                            && cursor.1 < height
                        {
                            if !calculated_events.contains_key(name) {
                                calculated_events
                                    .insert(name, vec![(element.clone(), event.clone())]);
                            } else {
                                calculated_events
                                    .get_mut(name)
                                    .unwrap()
                                    .push((element.clone(), event.clone()));
                            }
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

        let mut found_nodes: Vec<(&RenderData, &FreyaEvents)> = Vec::new();

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
            let event = match &request {
                &FreyaEvents::MouseEvent { cursor, .. } => Some(UserEvent {
                    scope_id: None,
                    priority: EventPriority::Medium,
                    element: Some(node.node_id.clone()),
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
                &FreyaEvents::WheelEvent { scroll, .. } => Some(UserEvent {
                    scope_id: None,
                    priority: EventPriority::Medium,
                    element: Some(node.node_id.clone()),
                    name: event_name,
                    bubbles: false,
                    data: Arc::new(WheelData::new(WheelDelta::Pixels(PixelsVector::new(
                        scroll.0, scroll.1, 0.0,
                    )))),
                }),
                &FreyaEvents::KeyboardEvent { name, code } => Some(UserEvent {
                    scope_id: None,
                    priority: EventPriority::Medium,
                    element: Some(node.node_id.clone()),
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
