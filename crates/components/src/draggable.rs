use dioxus::prelude::*;
use freya_core::custom_attributes::NodeReferenceLayout;
use freya_elements::{
    self as dioxus_elements,
    events::{
        MouseEvent,
        PointerEvent,
    },
};
use freya_hooks::{
    use_id,
    use_node_signal,
};
use torin::prelude::CursorPoint;

#[derive(Clone)]
struct DraggableCanvasLayout(ReadOnlySignal<NodeReferenceLayout>);

#[derive(Clone)]
struct DraggableCanvasRegistry(Signal<Vec<usize>>);

#[component]
pub fn DraggableCanvas(
    children: Element,
    width: Option<String>,
    height: Option<String>,
) -> Element {
    let (reference, layout) = use_node_signal();
    use_context_provider(|| DraggableCanvasLayout(layout));
    use_context_provider(|| DraggableCanvasRegistry(Signal::new(Vec::new())));
    rsx!(
        rect {
            reference,
            width,
            height,
            {children}
        }
    )
}

#[component]
pub fn Draggable(children: Element) -> Element {
    let mut position = use_signal(|| CursorPoint::new(0., 0.));
    let mut dragging_position = use_signal::<Option<CursorPoint>>(|| None);
    let DraggableCanvasLayout(layout) = use_context::<DraggableCanvasLayout>();
    let DraggableCanvasRegistry(mut registry) = use_context::<DraggableCanvasRegistry>();
    let id = use_id::<DraggableCanvasLayout>();

    use_hook(move || {
        registry.write().push(id);
    });

    use_drop(move || {
        registry.write().retain(|i| *i != id);
    });

    let onglobalmousemove = move |e: MouseEvent| {
        if let Some(dragging_position) = dragging_position() {
            position.set(CursorPoint::new(
                e.screen_coordinates.x - dragging_position.x,
                e.screen_coordinates.y - dragging_position.y,
            ));
            e.stop_propagation();
        }
    };

    let onmousedown = move |e: MouseEvent| {
        dragging_position.set(Some(CursorPoint::new(
            e.element_coordinates.x + layout.read().area.min_x() as f64,
            e.element_coordinates.y + layout.read().area.min_y() as f64,
        )));
        e.stop_propagation();
        let mut registry = registry.write();
        registry.retain(|i| *i != id);
        registry.insert(0, id);
    };

    let oncaptureglobalpointerup = move |e: PointerEvent| {
        if dragging_position.read().is_some() {
            e.stop_propagation();
            e.prevent_default();
            dragging_position.set(None);
        }
    };

    let (left, top) = position().to_tuple();

    let layer = registry
        .read()
        .iter()
        .rev()
        .position(|i| *i == id)
        .map(|layer| layer * 1024)
        .unwrap_or_default();

    rsx!(
        rect {
            position: "absolute",
            position_left: "{left}",
            position_top: "{top}",
            onglobalmousemove,
            onmousedown,
            oncaptureglobalpointerup,
            layer: "-{layer}",
            {children}
        }
    )
}
