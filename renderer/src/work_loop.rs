use dioxus_core::{ElementId, EventPriority, SchedulerMsg, UserEvent};
use dioxus_native_core::real_dom::{Node, NodeType};
use layers_engine::{Layers, NodeArea, NodeData};
use layout_engine::calculate_node;
use skia_safe::{Canvas, Color, Font};
use state::node::{NodeState, SizeMode};
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
    let mut events_filtered: HashMap<&'static str, Vec<(NodeData, RendererRequest)>> =
        HashMap::new();
    calculate_node(
        &NodeData {
            width: SizeMode::Percentage(100),
            height: SizeMode::Percentage(100),
            padding: (0, 0, 0, 0),
            node: Some(root),
        },
        area.clone(),
        area,
        &mut (dom, &mut events_filtered, &renderer_requests),
        layers,
        |node_id, (dom, _, _)| {
            let child = {
                let dom = dom.lock().unwrap();
                dom.index(*node_id).clone()
            };

            Some(NodeData {
                width: child.state.size.width,
                height: child.state.size.height,
                padding: child.state.size.padding,
                node: Some(child),
            })
        },
        0,
    );

    let mut layers_nums: Vec<&i16> = layers.layers.keys().collect();

    // From top to bottom
    layers_nums.sort_by(|a, b| a.cmp(b));

    // Save all the viewports for each layer
    let mut viewports: HashMap<ElementId, Vec<NodeArea>> = HashMap::new();

    for layer_num in &layers_nums {
        let layer = layers.layers.get(layer_num).unwrap();
        for (id, element) in layer {
            match &element.node.node.as_ref().unwrap().node_type {
                NodeType::Element { tag, .. } => {
                    for child in &element.children {
                        if !viewports.contains_key(&child) {
                            viewports.insert(*child, Vec::new());
                        }
                        if viewports.contains_key(&id) {
                            viewports.insert(*child, viewports.get(&id).unwrap().clone());
                        }
                        if tag == "container" {
                            viewports
                                .get_mut(&child)
                                .unwrap()
                                .push(element.area.clone());
                        }
                    }
                }
                NodeType::Text { .. } => {}
                _ => {}
            }
        }
    }

    // From bottom to top
    layers_nums.sort_by(|a, b| a.cmp(b));

    // Render all the layers from the bottom to the top
    for layer_num in &layers_nums {
        let layer = layers.layers.get(layer_num).unwrap();

        for (id, element) in layer {
            canvas.save();
            render_skia(
                &mut dom,
                &mut canvas,
                &element.node,
                &element.area,
                font,
                &viewports.get(id).unwrap_or(&Vec::new()),
            );
            canvas.restore();
        }
    }

    layers_nums.sort_by(|a, b| a.cmp(b));

    // Propagate events from the top to the bottom
    for layer_num in &layers_nums {
        let layer = layers.layers.get(layer_num).unwrap();

        for (id, element) in layer.iter() {
            let requests = renderer_requests.lock().unwrap();

            for request in requests.iter() {
                let node = &element.node;
                let area = &element.area;
                match request {
                    RendererRequest::MouseEvent { name, event } => {
                        let x = area.x as f64;
                        let y = area.y as f64;
                        let width = (area.x + area.width) as f64;
                        let height = (area.y + area.height) as f64;
                        let cursor = event.client_coordinates();

                        let mut visible = true;

                        for viewport in viewports.get(&id).unwrap_or(&Vec::new()) {
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
                            if !events_filtered.contains_key(name) {
                                events_filtered.insert(name, vec![(node.clone(), request.clone())]);
                            } else {
                                events_filtered
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

    let mut events: Vec<UserEvent> = Vec::new();

    for (event_name, event_nodes) in events_filtered.iter_mut() {
        let dom = dom.lock().unwrap();
        let listeners = dom.get_listening_sorted(event_name);

        let mut found_node: Option<(&Node<NodeState>, &RendererRequest)> = None;

        'event_nodes: for (node_data, request) in event_nodes.iter() {
            let node = node_data.node.as_ref().unwrap();
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

                    events.push(event.clone());

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

    let new_events = events_processor.process_events_batch(events, events_filtered);

    for new_event in new_events {
        event_emitter
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .unbounded_send(SchedulerMsg::Event(new_event))
            .unwrap();
    }

    renderer_requests.lock().unwrap().clear();
}
