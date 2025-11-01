use freya_core::prelude::*;
use torin::prelude::{
    Area,
    CursorPoint,
    Position,
};

#[derive(Clone)]
struct DraggableCanvasLayout(State<Area>);

#[derive(Clone)]
struct DraggableCanvasRegistry(State<Vec<usize>>);

#[derive(PartialEq)]
pub struct DraggableCanvas {
    children: Vec<Element>,
    layout: LayoutData,
}

impl Default for DraggableCanvas {
    fn default() -> Self {
        Self::new()
    }
}

impl DraggableCanvas {
    pub fn new() -> Self {
        Self {
            children: vec![],
            layout: LayoutData::default(),
        }
    }
}

impl LayoutExt for DraggableCanvas {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.layout
    }
}

impl ContainerExt for DraggableCanvas {}

impl ChildrenExt for DraggableCanvas {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl Render for DraggableCanvas {
    fn render(&self) -> Element {
        let mut layout = use_state(Area::default);
        use_provide_context(move || DraggableCanvasLayout(layout));
        use_provide_context(|| DraggableCanvasRegistry(State::create(Vec::new())));
        rect()
            .layout(self.layout.clone())
            .on_sized(move |e: Event<SizedEventData>| layout.set(e.visible_area))
            .children(self.children.clone())
            .into()
    }
}

#[derive(PartialEq)]
pub struct Draggable {
    children: Vec<Element>,
}

impl Default for Draggable {
    fn default() -> Self {
        Self::new()
    }
}

impl Draggable {
    pub fn new() -> Self {
        Self { children: vec![] }
    }
}

impl ChildrenExt for Draggable {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl Render for Draggable {
    fn render(&self) -> Element {
        let mut position = use_state(|| CursorPoint::new(0., 0.));
        let mut dragging_position = use_state::<Option<CursorPoint>>(|| None);
        let DraggableCanvasLayout(layout) = use_consume::<DraggableCanvasLayout>();
        let DraggableCanvasRegistry(mut registry) = use_consume::<DraggableCanvasRegistry>();
        let id = use_id::<DraggableCanvasLayout>();

        use_hook(move || {
            registry.write().push(id);
        });

        use_drop(move || {
            registry.write().retain(|i| *i != id);
        });

        let on_global_mouse_move = move |e: Event<MouseEventData>| {
            if let Some(dragging_position) = dragging_position() {
                position.set(CursorPoint::new(
                    e.global_location.x - dragging_position.x,
                    e.global_location.y - dragging_position.y,
                ));
                e.stop_propagation();
            }
        };

        let on_pointer_down = move |e: Event<PointerEventData>| {
            dragging_position.set(Some(CursorPoint::new(
                e.element_location().x + layout.read().min_x() as f64,
                e.element_location().y + layout.read().min_y() as f64,
            )));
            e.stop_propagation();
            let mut registry = registry.write();
            registry.retain(|i| *i != id);
            registry.insert(0, id);
        };

        let on_capture_global_mouse_up = move |e: Event<MouseEventData>| {
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

        rect()
            .on_global_mouse_move(on_global_mouse_move)
            .on_pointer_down(on_pointer_down)
            .on_capture_global_mouse_up(on_capture_global_mouse_up)
            .position(Position::new_absolute().left(left as f32).top(top as f32))
            .layer(layer as i16)
            .children(self.children.clone())
            .into()
    }
}
