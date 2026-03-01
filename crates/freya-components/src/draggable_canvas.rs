use freya_core::prelude::*;
use torin::{
    prelude::{
        Area,
        CursorPoint,
        Position,
        Size2D,
    },
    size::Size,
};

#[derive(Clone)]
struct DraggableCanvasRegistry(State<Vec<usize>>);

/// A canvas container that allows draggable elements within it.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     DraggableCanvas::new().child(Draggable::new().child("Draggable item"))
/// }
/// ```
#[derive(PartialEq)]
pub struct DraggableCanvas {
    children: Vec<Element>,
    layout: LayoutData,
    key: DiffKey,
}

impl KeyExt for DraggableCanvas {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
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
            key: DiffKey::None,
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

impl Component for DraggableCanvas {
    fn render(&self) -> impl IntoElement {
        let mut layout = use_state(Area::default);
        use_provide_context(|| DraggableCanvasRegistry(State::create(Vec::new())));
        let focus = use_focus();
        let mut offset = use_state(CursorPoint::zero);
        let mut dragging_position = use_state::<Option<CursorPoint>>(|| None);

        let on_mouse_move = move |e: Event<MouseEventData>| {
            if let Some(dragging_position) = dragging_position() {
                offset.set((e.element_location - dragging_position).to_point());
                e.stop_propagation();
            }
        };

        let on_pointer_down = move |e: Event<PointerEventData>| {
            dragging_position.set(Some((offset() - e.element_location()).to_point()));
            e.stop_propagation();
        };

        let on_global_mouse_up = move |e: Event<MouseEventData>| {
            if dragging_position.read().is_some() {
                e.stop_propagation();
                e.prevent_default();
                dragging_position.set(None);
            }
        };

        let on_wheel = move |e: Event<WheelEventData>| {
            let mut current_offset = offset.write();
            current_offset.x += e.delta_x;
            current_offset.y += e.delta_y;
        };

        let (offset_x, offset_y) = offset().to_tuple();

        rect()
            .layout(self.layout.clone())
            .on_sized(move |e: Event<SizedEventData>| layout.set(e.visible_area))
            .on_mouse_move(on_mouse_move)
            .on_pointer_down(on_pointer_down)
            .on_global_mouse_up(on_global_mouse_up)
            .on_wheel(on_wheel)
            .offset_x(offset_x as f32)
            .offset_y(offset_y as f32)
            .a11y_id(focus.a11y_id())
            .a11y_role(AccessibilityRole::ScrollView)
            .a11y_builder(move |node| {
                node.set_scroll_x(offset_x);
                node.set_scroll_y(offset_y)
            })
            .scrollable(true)
            .overflow(Overflow::Clip)
            .children(self.children.clone())
    }
    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

#[derive(PartialEq)]
pub struct Draggable {
    initial_position: CursorPoint,
    children: Vec<Element>,
    key: DiffKey,
}

impl Default for Draggable {
    fn default() -> Self {
        Self::new()
    }
}

impl Draggable {
    pub fn new() -> Self {
        Self {
            initial_position: CursorPoint::zero(),
            children: vec![],
            key: DiffKey::None,
        }
    }

    pub fn initial_position(mut self, initial_position: impl Into<CursorPoint>) -> Self {
        self.initial_position = initial_position.into();
        self
    }
}

impl KeyExt for Draggable {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl ChildrenExt for Draggable {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl Component for Draggable {
    fn render(&self) -> impl IntoElement {
        let mut position = use_state(|| self.initial_position);
        let mut dragging_position = use_state::<Option<CursorPoint>>(|| None);
        let DraggableCanvasRegistry(mut registry) = use_consume::<DraggableCanvasRegistry>();
        let id = use_id::<DraggableCanvas>();

        use_hook(move || {
            registry.write().push(id);
        });

        use_drop(move || {
            registry.write().retain(|i| *i != id);
        });

        let on_global_mouse_move = move |e: Event<MouseEventData>| {
            if let Some(dragging_position) = dragging_position() {
                position.set((e.global_location - dragging_position).to_point());
                e.stop_propagation();
            }
        };

        let on_pointer_down = move |e: Event<PointerEventData>| {
            dragging_position.set(Some((e.global_location() - position()).to_point()));
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

        let (left, top) = position().to_f32().to_tuple();

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
            .position(Position::new_absolute().left(left).top(top))
            .layer(layer as i16)
            .children(self.children.clone())
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

/// Which edge/corner is currently being resized.
#[derive(Clone, Copy, PartialEq, Debug)]
enum ResizeEdge {
    Right,
    Bottom,
    BottomRight,
}

#[derive(PartialEq)]
pub struct ResizableDraggable {
    initial_position: CursorPoint,
    initial_size: Size2D,
    handle_size: f32,
    corner_size: f32,
    children: Vec<Element>,
    key: DiffKey,
}

impl ResizableDraggable {
    pub fn new(initial_size: impl Into<Size2D>) -> Self {
        Self {
            initial_position: CursorPoint::zero(),
            initial_size: initial_size.into(),
            handle_size: 4.,
            corner_size: 12.,
            children: vec![],
            key: DiffKey::None,
        }
    }

    pub fn initial_position(mut self, initial_position: impl Into<CursorPoint>) -> Self {
        self.initial_position = initial_position.into();
        self
    }
}

impl KeyExt for ResizableDraggable {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl ChildrenExt for ResizableDraggable {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl Component for ResizableDraggable {
    fn render(&self) -> impl IntoElement {
        let mut position = use_state(|| self.initial_position);
        let mut size = use_state(|| self.initial_size);
        let mut dragging_position = use_state::<Option<CursorPoint>>(|| None);
        let mut resizing = use_state::<Option<(ResizeEdge, CursorPoint)>>(|| None);
        let DraggableCanvasRegistry(mut registry) = use_consume::<DraggableCanvasRegistry>();
        let id = use_id::<DraggableCanvas>();

        use_hook(move || {
            registry.write().push(id);
        });

        use_drop(move || {
            registry.write().retain(|i| *i != id);
        });

        let on_global_mouse_move = move |e: Event<MouseEventData>| {
            if let Some(dragging_position) = dragging_position() {
                position.set((e.global_location - dragging_position).to_point());
                e.stop_propagation();
            }
            if let Some((edge, start_point)) = resizing() {
                // For now hardcoded, but I could probably just make it customizable
                const MIN: f32 = 20.;

                let delta = (e.global_location - start_point).to_f32().to_point();
                let (current_width, current_height) = size().to_tuple();
                let new_width = if matches!(edge, ResizeEdge::Right | ResizeEdge::BottomRight) {
                    (current_width + delta.x).max(MIN)
                } else {
                    current_width
                };
                let new_height = if matches!(edge, ResizeEdge::Bottom | ResizeEdge::BottomRight) {
                    (current_height + delta.y).max(MIN)
                } else {
                    current_height
                };
                size.set((new_width, new_height).into());
                resizing.set(Some((edge, e.global_location)));
                e.stop_propagation();
            }
        };

        let on_pointer_down = move |e: Event<PointerEventData>| {
            // Don't start dragging if a resize was just initiated on a handle
            if resizing.read().is_some() {
                return;
            }
            dragging_position.set(Some((e.global_location() - position()).to_point()));
            e.stop_propagation();
            let mut registry_write = registry.write();
            registry_write.retain(|i| *i != id);
            registry_write.insert(0, id);
        };

        let on_capture_global_mouse_up = move |e: Event<MouseEventData>| {
            if dragging_position.read().is_some() {
                e.stop_propagation();
                e.prevent_default();
                dragging_position.set(None);
            }
            if resizing.read().is_some() {
                e.stop_propagation();
                e.prevent_default();
                resizing.set(None);
                Cursor::set(CursorIcon::default());
            }
        };

        let (left, top) = position().to_f32().to_tuple();
        let (width, height) = size().to_tuple();

        let layer = registry
            .read()
            .iter()
            .rev()
            .position(|i| *i == id)
            .map(|layer| layer * 1024)
            .unwrap_or_default();

        let handle = self.handle_size;
        let corner = self.corner_size;

        let right_handle = rect()
            .width(Size::px(handle))
            .height(Size::px(height - corner))
            .position(Position::new_absolute().right(-handle).top(0.))
            .background(Color::WHITE)
            .opacity(0.)
            .on_pointer_enter(move |_: Event<PointerEventData>| {
                Cursor::set(CursorIcon::ColResize);
            })
            .on_pointer_leave(move |_: Event<PointerEventData>| {
                if resizing().is_none() {
                    Cursor::set(CursorIcon::default());
                }
            })
            .on_pointer_down(move |e: Event<PointerEventData>| {
                e.stop_propagation();
                resizing.set(Some((ResizeEdge::Right, e.global_location())));
                let mut registry = registry.write();
                registry.retain(|i| *i != id);
                registry.insert(0, id);
            });

        let bottom_handle = rect()
            .width(Size::px(width - corner))
            .height(Size::px(handle))
            .position(Position::new_absolute().left(0.).bottom(-handle))
            .background(Color::WHITE)
            .opacity(0.)
            .on_pointer_enter(move |_: Event<PointerEventData>| {
                Cursor::set(CursorIcon::RowResize);
            })
            .on_pointer_leave(move |_: Event<PointerEventData>| {
                if resizing().is_none() {
                    Cursor::set(CursorIcon::default());
                }
            })
            .on_pointer_down(move |e: Event<PointerEventData>| {
                e.stop_propagation();
                resizing.set(Some((ResizeEdge::Bottom, e.global_location())));
                let mut registry = registry.write();
                registry.retain(|i| *i != id);
                registry.insert(0, id);
            });

        let corner_handle = rect()
            .width(Size::px(corner))
            .height(Size::px(corner))
            .position(Position::new_absolute().right(-handle).bottom(-handle))
            .background(Color::WHITE)
            .opacity(0.)
            .on_pointer_enter(move |_: Event<PointerEventData>| {
                Cursor::set(CursorIcon::SeResize);
            })
            .on_pointer_leave(move |_: Event<PointerEventData>| {
                if resizing().is_none() {
                    Cursor::set(CursorIcon::default());
                }
            })
            .on_pointer_down(move |e: Event<PointerEventData>| {
                e.stop_propagation();
                resizing.set(Some((ResizeEdge::BottomRight, e.global_location())));
                let mut registry = registry.write();
                registry.retain(|i| *i != id);
                registry.insert(0, id);
            });

        rect()
            .on_global_mouse_move(on_global_mouse_move)
            .on_pointer_down(on_pointer_down)
            .on_capture_global_mouse_up(on_capture_global_mouse_up)
            .position(Position::new_absolute().left(left).top(top))
            .width(Size::px(width))
            .height(Size::px(height))
            .layer(layer as i16)
            .child(
                rect()
                    .overflow(Overflow::Clip)
                    .children(self.children.clone()),
            )
            .child(right_handle)
            .child(bottom_handle)
            .child(corner_handle)
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
