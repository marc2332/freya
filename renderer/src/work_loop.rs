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
use freya_layers::{Layers, NodeArea, NodeData, RenderData};
use freya_layout::calculate_node;
use freya_node_state::node::{NodeState, Size};
use skia_safe::{textlayout::FontCollection, Canvas, Color};
use std::{collections::HashMap, ops::Index, sync::Arc};

use crate::{
    events_processor::EventsProcessor, renderer::render_skia, EventEmitter, RendererRequest,
    RendererRequests, SkiaDom,
};

pub fn work_loop(
    mut dom: &SkiaDom,
    mut canvas: &mut Canvas,
    area: NodeArea,
    renderer_requests: RendererRequests,
    event_emitter: &EventEmitter,
    font_collection: &mut FontCollection,
    events_processor: &mut EventsProcessor,
) {
    let root: Node<NodeState> = {
        let dom = dom.lock().unwrap();
        dom.index(ElementId(0)).clone()
    };

    let layers = &mut Layers::default();

    calculate_node(
        &NodeData {
            size: Size::default(),
            node: root,
        },
        area.clone(),
        area,
        &mut dom,
        layers,
        |node_id, dom| {
            let child = {
                let dom = dom.lock().unwrap();
                dom.index(*node_id).clone()
            };

            Some(NodeData {
                size: child.state.size,
                node: child,
            })
        },
        0,
    );

    let mut layers_nums: Vec<&i16> = layers.layers.keys().collect();

    // From top to bottom
    layers_nums.sort_by(|a, b| a.cmp(b));

    // Calculate all the applicable viewports for the given elements
    let mut calculated_viewports: HashMap<ElementId, Vec<NodeArea>> = HashMap::new();

    for layer_num in &layers_nums {
        let layer = layers.layers.get(layer_num).unwrap();
        for (id, element) in layer {
            match &element.node_data.node.node_type {
                NodeType::Element { tag, .. } => {
                    for child in &element.node_children {
                        if !calculated_viewports.contains_key(&child) {
                            calculated_viewports.insert(*child, Vec::new());
                        }
                        if calculated_viewports.contains_key(&id) {
                            calculated_viewports
                                .insert(*child, calculated_viewports.get(&id).unwrap().clone());
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
        for (id, element) in layer {
            canvas.save();
            render_skia(
                &mut dom,
                &mut canvas,
                &element.node_data,
                &element.node_area,
                font_collection,
                &calculated_viewports.get(id).unwrap_or(&Vec::new()),
            );
            canvas.restore();
        }
    }

    // Calculated events are those that match considering their viewports
    let mut calculated_events: HashMap<&'static str, Vec<(RenderData, RendererRequest)>> =
        HashMap::new();

    // Propagate events from the top to the bottom
    for layer_num in &layers_nums {
        let layer = layers.layers.get(layer_num).unwrap();

        for (id, element) in layer.iter() {
            let requests = renderer_requests.lock().unwrap();

            for request in requests.iter() {
                let area = &element.node_area;
                let data = match request {
                    RendererRequest::MouseEvent { name, cursor, .. } => Some((name, cursor)),
                    RendererRequest::WheelEvent { name, cursor, .. } => Some((name, cursor)),
                    _ => None,
                };
                if let Some((name, cursor)) = data {
                    let x = area.x as f64;
                    let y = area.y as f64;
                    let width = (area.x + area.width) as f64;
                    let height = (area.y + area.height) as f64;

                    let mut visible = true;

                    // Make sure the cursor is inside all the applicable viewports from the element
                    for viewport in calculated_viewports.get(&id).unwrap_or(&Vec::new()) {
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
                                .insert(name, vec![(element.clone(), request.clone())]);
                        } else {
                            calculated_events
                                .get_mut(name)
                                .unwrap()
                                .push((element.clone(), request.clone()));
                        }
                    }
                }
            }
        }
    }

    let mut new_events: Vec<UserEvent> = Vec::new();

    // Calculate what process can actually be triggered
    for (event_name, event_nodes) in calculated_events.iter_mut() {
        let dom = dom.lock().unwrap();
        let listeners = dom.get_listening_sorted(event_name);

        let mut found_node: Option<(&RenderData, &RendererRequest)> = None;

        'event_nodes: for (node, request) in event_nodes.iter() {
            let node_state = &node.node_data.node;
            for listener in &listeners {
                if listener.id == node_state.id {
                    if node_state.state.style.background != Color::TRANSPARENT
                        && event_name == &"wheel"
                    {
                        break 'event_nodes;
                    }
                    found_node = Some((node, request));
                }
            }
        }

        if let Some((node, request)) = found_node {
            let event = match &request {
                &RendererRequest::MouseEvent { cursor, .. } => Some(UserEvent {
                    scope_id: None,
                    priority: EventPriority::Medium,
                    element: Some(node.node_data.node.id.clone()),
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
                &RendererRequest::WheelEvent { scroll, .. } => Some(UserEvent {
                    scope_id: None,
                    priority: EventPriority::Medium,
                    element: Some(node.node_data.node.id.clone()),
                    name: event_name,
                    bubbles: false,
                    data: Arc::new(WheelData::new(WheelDelta::Pixels(PixelsVector::new(
                        scroll.0, scroll.1, 0.0,
                    )))),
                }),
                _ => None,
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

    renderer_requests.lock().unwrap().clear();
}
