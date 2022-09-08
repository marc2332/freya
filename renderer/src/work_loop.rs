use dioxus_core::{ElementId, EventPriority, SchedulerMsg, UserEvent};
use dioxus_native_core::real_dom::{Node, NodeType};
use layers_engine::{Layers, NodeArea, NodeData};
use layout_engine::calculate_node;
use skia_safe::{Canvas, Color, Font};
use state::node::{NodeState, Size};
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
    font: &Font,
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
                font,
                &calculated_viewports.get(id).unwrap_or(&Vec::new()),
            );
            canvas.restore();
        }
    }

    // Calculated events are those that match considering their viewports
    let mut calculated_events: HashMap<&'static str, Vec<(NodeData, RendererRequest)>> =
        HashMap::new();

    // Propagate events from the top to the bottom
    for layer_num in &layers_nums {
        let layer = layers.layers.get(layer_num).unwrap();

        for (id, element) in layer.iter() {
            let requests = renderer_requests.lock().unwrap();

            for request in requests.iter() {
                let node = &element.node_data;
                let area = &element.node_area;
                match request {
                    RendererRequest::MouseEvent { name, event } => {
                        let x = area.x as f64;
                        let y = area.y as f64;
                        let width = (area.x + area.width) as f64;
                        let height = (area.y + area.height) as f64;
                        let cursor = event.client_coordinates();

                        let mut visible = true;

                        // Make sure the cursor is inside all the applicable viewports from the element
                        for viewport in calculated_viewports.get(&id).unwrap_or(&Vec::new()) {
                            if cursor.x < viewport.x as f64
                                || cursor.x > (viewport.x + viewport.width) as f64
                                || cursor.y < viewport.y as f64
                                || cursor.y > (viewport.y + viewport.height) as f64
                            {
                                visible = false;
                            }
                        }

                        // Make sure the cursor is inside the node area
                        if visible
                            && cursor.x > x
                            && cursor.x < width
                            && cursor.y > y
                            && cursor.y < height
                        {
                            if !calculated_events.contains_key(name) {
                                calculated_events
                                    .insert(name, vec![(node.clone(), request.clone())]);
                            } else {
                                calculated_events
                                    .get_mut(name)
                                    .unwrap()
                                    .push((node.clone(), request.clone()));
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    // The new events are the events from `calculated_events` but actually are listening
    // and fit considering the current state of the app

    let mut new_events: Vec<UserEvent> = Vec::new();

    for (event_name, event_nodes) in calculated_events.iter_mut() {
        let dom = dom.lock().unwrap();
        let listeners = dom.get_listening_sorted(event_name);

        let mut found_node: Option<(&Node<NodeState>, &RendererRequest)> = None;

        'event_nodes: for (node_data, request) in event_nodes.iter() {
            let node = &node_data.node;
            for listener in &listeners {
                if listener.id == node.id {
                    if node.state.style.background != Color::TRANSPARENT && event_name == &"scroll"
                    {
                        break 'event_nodes;
                    }
                    found_node = Some((node, request));
                }
            }
        }

        if let Some((node, request)) = found_node {
            match &request {
                &RendererRequest::MouseEvent { event, .. } => {
                    let event = UserEvent {
                        scope_id: None,
                        priority: EventPriority::Medium,
                        element: Some(node.id.clone()),
                        name: event_name,
                        bubbles: false,
                        data: Arc::new(event.clone()),
                    };

                    new_events.push(event.clone());

                    event_emitter
                        .lock()
                        .unwrap()
                        .as_ref()
                        .unwrap()
                        .unbounded_send(SchedulerMsg::Event(event))
                        .unwrap();
                }
                _ => {}
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
