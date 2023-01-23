use dioxus_core::ElementId;
use dioxus_native_core::{node::NodeType, real_dom::RealDom, NodeId};
use euclid::{Length, Point2D};
use freya_common::{LayoutMemorizer, NodeArea};
use freya_elements::events_data::{KeyboardData, MouseData, WheelData};
use freya_layers::{Layers, RenderData};
use freya_layout::NodeLayoutMeasurer;
use freya_node_state::{CustomAttributeValues, NodeState};
use rustc_hash::FxHashMap;
use skia_safe::{textlayout::FontCollection, Color};
use std::{
    any::Any,
    ops::Index,
    rc::Rc,
    sync::{Arc, Mutex},
};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tracing::info;

pub mod events;

use events::{EventsProcessor, FreyaEvent};

pub type SafeDOM = Arc<Mutex<RealDom<NodeState, CustomAttributeValues>>>;
pub type EventEmitter = UnboundedSender<DomEvent>;
pub type EventReceiver = UnboundedReceiver<DomEvent>;
pub type SafeLayoutMemorizer = Arc<Mutex<LayoutMemorizer>>;
pub type SafeFreyaEvents = Arc<Mutex<Vec<FreyaEvent>>>;
pub type ViewportsCollection = FxHashMap<NodeId, (Option<NodeArea>, Vec<NodeId>)>;

/// The Work Loop has a few jobs:
/// - Measure the nodes layouts
/// - Organize the nodes layouts in layers
/// - Calculate all the nodes viewports
/// - Call the render to paint
/// - Calculate what events must be triggered
#[allow(clippy::too_many_arguments)]
pub fn process_work<HookOptions>(
    dom: &SafeDOM,
    area: NodeArea,
    freya_events: SafeFreyaEvents,
    event_emitter: &EventEmitter,
    font_collection: &mut FontCollection,
    events_processor: &mut EventsProcessor,
    manager: &SafeLayoutMemorizer,
    hook_options: &mut HookOptions,
    render_hook: impl Fn(
        &SafeDOM,
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
        );
        root_node_measurer.measure_area(true);
    }

    #[cfg(debug_assertions)]
    {
        let dirty_nodes_counter = manager.lock().unwrap().dirty_nodes_counter;
        if dirty_nodes_counter > 0 {
            let nodes = manager.lock().unwrap().nodes.len();
            info!("Measured layout of {}/{}", dirty_nodes_counter, nodes);
            manager.lock().unwrap().dirty_nodes_counter = 0;
        }
    }

    let mut layers_nums: Vec<&i16> = layers.layers.keys().collect();

    // From top to bottom
    layers_nums.sort();

    // Calculate all the applicable viewports for the given nodes
    let mut viewports_collection: ViewportsCollection = FxHashMap::default();

    for layer_num in &layers_nums {
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

            // Let the render know what to actually render
            render_hook(
                dom,
                dom_element,
                font_collection,
                &viewports_collection,
                hook_options,
            )
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
                                .entry(name)
                                .or_insert_with(Vec::new)
                                .push(event_data);
                        }
                    }
                }
            }
        }
    }

    let mut new_events: Vec<DomEvent> = Vec::new();

    // Calculate what event listeners can actually be triggered
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

    // Calculate new events by processing the old and new
    let new_processed_events = events_processor.process_events_batch(new_events, calculated_events);

    for event in new_processed_events {
        event_emitter.send(event).unwrap();
    }

    freya_events.lock().unwrap().clear();
}

#[derive(Debug, Clone)]
pub struct DomEvent {
    pub name: String,
    pub element_id: ElementId,
    pub data: DomEventData,
}

#[derive(Debug, Clone)]
pub enum DomEventData {
    Mouse(MouseData),
    Keyboard(KeyboardData),
    Wheel(WheelData),
}

impl DomEventData {
    pub fn any(self) -> Rc<dyn Any> {
        match self {
            DomEventData::Mouse(m) => Rc::new(m),
            DomEventData::Keyboard(k) => Rc::new(k),
            DomEventData::Wheel(w) => Rc::new(w),
        }
    }
}
