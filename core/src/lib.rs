use dioxus_native_core::prelude::{ElementNode, NodeImmutableDioxusExt};
use dioxus_native_core::real_dom::NodeImmutable;
use dioxus_native_core::{node::NodeType, NodeId};
use freya_common::NodeReferenceLayout;
use freya_dom::prelude::{DioxusDOM, DioxusDOMAdapter, FreyaDOM};
use freya_layout::{Layers, SkiaMeasurer};
use torin::geometry::Area;

use freya_node_state::{CursorMode, CursorSettings, References, SizeState, Style};
use rustc_hash::FxHashMap;
use skia_safe::{textlayout::FontCollection, Color};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub mod events;
pub mod node;

use events::{DomEvent, EventsProcessor, FreyaEvent};
use torin::torin::Torin;

pub type EventEmitter = UnboundedSender<DomEvent>;
pub type EventReceiver = UnboundedReceiver<DomEvent>;
pub type EventsQueue = Vec<FreyaEvent>;
pub type ViewportsCollection = FxHashMap<NodeId, (Option<Area>, Vec<NodeId>)>;
pub type NodesEvents = FxHashMap<String, Vec<(NodeId, FreyaEvent)>>;

// Calculate all the applicable viewports for the given nodes
pub fn calculate_viewports(
    layers_nums: &[&i16],
    layers: &Layers,
    fdom: &FreyaDOM,
) -> ViewportsCollection {
    let mut viewports_collection = FxHashMap::default();
    let layout = fdom.layout();
    let rdom = fdom.rdom();

    for layer_num in layers_nums {
        let layer = layers.layers.get(layer_num).unwrap();
        for node_id in layer {
            let node = rdom.get(*node_id);
            let node_areas = layout.get(*node_id);

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

/// Measure globale events
pub fn measure_global_events(events: &EventsQueue) -> Vec<FreyaEvent> {
    let mut global_events = Vec::default();
    for event in events {
        let event_name = match event.get_name() {
            "click" => Some("globalclick"),
            "mouseover" => Some("globalmouseover"),
            _ => None,
        };
        if let Some(event_name) = event_name {
            let mut global_event = event.clone();
            global_event.set_name(event_name.to_string());
            global_events.push(global_event);
        }
    }
    global_events
}

/// Measure what potential event listeners could be triggered
pub fn measure_potential_event_listeners(
    layers_nums: &[&i16],
    layers: &Layers,
    events: &EventsQueue,
    viewports_collection: &ViewportsCollection,
    fdom: &FreyaDOM,
) -> NodesEvents {
    let mut potential_events = FxHashMap::default();

    let layout = fdom.layout();

    // Propagate events from the top to the bottom
    for layer_num in layers_nums {
        let layer = layers.layers.get(layer_num).unwrap();

        for node_id in layer {
            let areas = layout.get(*node_id);
            if let Some(areas) = areas {
                'events: for event in events.iter() {
                    if let FreyaEvent::Keyboard { name, .. } = event {
                        let event_data = (*node_id, event.clone());
                        potential_events
                            .entry(name.clone())
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

                                potential_events
                                    .entry(name.clone())
                                    .or_insert_with(Vec::new)
                                    .push(event_data);
                            }
                        }
                    }
                }
            }
        }
    }

    potential_events
}

/// Some events might cause other events, like for example:
/// A `mouseover` might also trigger a `mouseenter`
/// A `mousedown` or a `touchdown` might also trigger a `pointerdown`
fn get_derivated_events(event_name: &str) -> Vec<&str> {
    match event_name {
        "mouseover" => {
            vec![event_name, "mouseenter", "pointerenter", "pointerover"]
        }
        "mousedown" | "touchdown" => {
            vec![event_name, "pointerdown"]
        }
        "click" | "ontouchend" => {
            vec![event_name, "pointerup"]
        }
        "mouseleave" => {
            vec![event_name, "pointerleave"]
        }
        _ => vec![event_name],
    }
}

/// Measure what DOM events could be emited
fn measure_dom_events(
    potential_events: &mut NodesEvents,
    fdom: &FreyaDOM,
    scale_factor: f64,
) -> Vec<DomEvent> {
    let mut new_events = Vec::new();
    let rdom = fdom.rdom();

    for (event_name, event_nodes) in potential_events.iter_mut() {
        let derivated_events = get_derivated_events(event_name.as_str());

        let mut found_nodes: Vec<(&NodeId, FreyaEvent)> = Vec::new();
        for derivated_event_name in derivated_events {
            let listeners = rdom.get_listening_sorted(derivated_event_name);
            'event_nodes: for (node_id, request) in event_nodes.iter() {
                for listener in &listeners {
                    if listener.id() == *node_id {
                        let Style { background, .. } = &*listener.get::<Style>().unwrap();

                        if background != &Color::TRANSPARENT && derivated_event_name == "wheel" {
                            break 'event_nodes;
                        }

                        if background != &Color::TRANSPARENT
                            && (derivated_event_name == "click"
                                || derivated_event_name == "touchstart"
                                || derivated_event_name == "touchend")
                        {
                            found_nodes.clear();
                        }

                        let mut request = request.clone();
                        request.set_name(derivated_event_name.to_string());

                        const STACKED_ELEMENTS: [&str; 13] = [
                            "mouseover",
                            "mouseenter",
                            "mouseleave",
                            "click",
                            "keydown",
                            "keyup",
                            "touchcancel",
                            "touchend",
                            "touchend",
                            "touchstart",
                            "pointerover",
                            "pointerenter",
                            "pointerleave",
                        ];

                        if STACKED_ELEMENTS.contains(&derivated_event_name) {
                            found_nodes.push((node_id, request))
                        } else {
                            found_nodes = vec![(node_id, request)]
                        }
                    }
                }
            }
        }

        for (node_id, request_event) in found_nodes {
            let areas = fdom.layout().get(*node_id).unwrap().clone();
            let node_ref = fdom.rdom().get(*node_id).unwrap();
            let element_id = node_ref.mounted_id().unwrap();
            let event = DomEvent::from_freya_event(
                *node_id,
                element_id,
                &request_event,
                Some(areas.area),
                scale_factor,
            );
            new_events.push(event);
        }
    }

    new_events
}

/// Emit global events
fn emit_global_events_listeners(
    global_events: Vec<FreyaEvent>,
    fdom: &FreyaDOM,
    event_emitter: &EventEmitter,
    scale_factor: f64,
) {
    for global_event in global_events {
        let event_name = global_event.get_name();
        let listeners = fdom.rdom().get_listening_sorted(event_name);

        for listener in listeners {
            let element_id = listener.mounted_id().unwrap();
            let event = DomEvent::from_freya_event(
                listener.id(),
                element_id,
                &global_event,
                None,
                scale_factor,
            );
            event_emitter.send(event).unwrap();
        }
    }
}

pub fn process_layers(
    layers: &mut Layers,
    rdom: &DioxusDOM,
    layout: &Torin<NodeId>,
    font_collection: &FontCollection,
    scale_factor: f32,
) {
    let mut inherit_layers = FxHashMap::default();

    rdom.traverse_depth_first(|node| {
        let areas = layout.get(node.id());

        if let Some(areas) = areas {
            // Add the Node to a Layer
            let node_style = node.get::<Style>().unwrap();

            let inherited_relative_layer = node
                .parent_id()
                .map(|p| *inherit_layers.get(&p).unwrap())
                .unwrap_or(0);

            let (node_layer, node_relative_layer) = Layers::calculate_layer(
                node_style.relative_layer,
                node.height() as i16,
                inherited_relative_layer,
            );

            inherit_layers.insert(node.id(), node_relative_layer);
            layers.add_element(node.id(), node_layer);

            // Register paragraph elements

            if let NodeType::Element(ElementNode { tag, .. }) = &*node.node_type() {
                if tag == "paragraph" {
                    let cursor_settings = node.get::<CursorSettings>().unwrap();
                    let is_editable = CursorMode::Editable == cursor_settings.mode;

                    let references = node.get::<References>().unwrap();
                    if is_editable {
                        if let Some(cursor_ref) = &references.cursor_ref {
                            let text_group = layers
                                .paragraph_elements
                                .entry(cursor_ref.text_id)
                                .or_insert_with(Vec::default);

                            text_group.push(node.id());
                        }
                    }
                }
            }

            // Notify layout references

            let size_state = &*node.get::<SizeState>().unwrap();

            if let Some(reference) = &size_state.node_ref {
                let mut node_layout = NodeReferenceLayout {
                    area: areas.area,
                    inner: areas.inner_sizes,
                };
                node_layout.div(scale_factor);
                reference.send(node_layout).ok();
            }
        }
    });

    layers.measure_all_paragraph_elements(rdom, layout, font_collection);
}

/// Process the layout of the DOM
pub fn process_layout(
    fdom: &FreyaDOM,
    area: Area,
    font_collection: &mut FontCollection,
    scale_factor: f32,
) -> (Layers, ViewportsCollection) {
    let rdom = fdom.rdom();
    let dom_adapter = DioxusDOMAdapter::new(rdom);
    let skia_measurer = SkiaMeasurer::new(rdom, font_collection);

    // Finds the best Node from where to start measuring
    fdom.layout().find_best_root(&dom_adapter);

    let root_id = fdom.rdom().root_id();

    // Measure the layout
    fdom.layout()
        .measure(root_id, area, &mut Some(skia_measurer), &dom_adapter);

    // Create the layers
    let mut layers = Layers::default();
    process_layers(
        &mut layers,
        rdom,
        &fdom.layout(),
        font_collection,
        scale_factor,
    );

    let mut layers_nums: Vec<&i16> = layers.layers.keys().collect();

    // Order the layers from top to bottom
    layers_nums.sort();

    // Calculate the viewports
    let viewports_collection = calculate_viewports(&layers_nums, &layers, fdom);

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

    let global_events = measure_global_events(events);

    let mut potential_events =
        measure_potential_event_listeners(&layers_nums, layers, events, viewports_collection, dom);

    let emitted_events = measure_dom_events(&mut potential_events, dom, scale_factor);

    let mut potential_colateral_events =
        events_processor.process_events(emitted_events, events, event_emitter);

    let emitted_colateral_events =
        measure_dom_events(&mut potential_colateral_events, dom, scale_factor);

    for event in emitted_colateral_events {
        event_emitter.send(event).unwrap();
    }

    emit_global_events_listeners(global_events, dom, event_emitter, scale_factor);

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
            let areas = layout.get(*node_id);

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
