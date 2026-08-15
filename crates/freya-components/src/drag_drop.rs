use freya_core::{
    prelude::*,
    scope_id::ScopeId,
};
use torin::prelude::*;

#[derive(Clone, Copy)]
enum DragPhase {
    Idle,
    Pressing {
        press_point: CursorPoint,
        offset: CursorPoint,
    },
    Dragging {
        position: CursorPoint,
        offset: CursorPoint,
    },
}

/// Access the global drag state for payloads of type `T`.
///
/// Returns a [`State`] that holds `Some(payload)` while a [`DragZone`] of `T` is being dragged
/// and `None` otherwise. Useful for components that need to react to ongoing drags (for example
/// to display drop targets only while dragging).
pub fn use_drag<T: 'static>() -> State<Option<T>> {
    match try_consume_root_context() {
        Some(s) => s,
        None => {
            let state = State::<Option<T>>::create_in_scope(None, ScopeId::ROOT);
            provide_context_for_scope_id(state, ScopeId::ROOT);
            state
        }
    }
}

/// A container whose children can be dragged into a [`DropZone`] expecting the same payload type.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     DragZone::new(0).drag_element("Dragging").child("Drag me!")
/// }
/// ```
#[derive(Clone, PartialEq)]
pub struct DragZone<T: Clone + 'static + PartialEq> {
    children: Vec<Element>,
    drag_element: Option<Element>,
    data: T,
    show_while_dragging: bool,
    drag_threshold: f64,
    enabled: bool,
    layout: LayoutData,
    key: DiffKey,
}

impl<T: Clone + PartialEq + 'static> ChildrenExt for DragZone<T> {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl<T: Clone + PartialEq + 'static> KeyExt for DragZone<T> {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl<T: Clone + PartialEq + 'static> LayoutExt for DragZone<T> {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.layout
    }
}

impl<T: Clone + PartialEq + 'static> ContainerExt for DragZone<T> {}

impl<T: Clone + PartialEq + 'static> DragZone<T> {
    /// Create a drag zone carrying `data`, which is handed to the destination [`DropZone`].
    pub fn new(data: T) -> Self {
        Self {
            data,
            children: Vec::new(),
            drag_element: None,
            show_while_dragging: true,
            drag_threshold: 4.0,
            enabled: true,
            layout: LayoutData::default(),
            key: DiffKey::default(),
        }
    }

    /// Whether the children stay visible while dragging. Defaults to `true`.
    pub fn show_while_dragging(mut self, show_while_dragging: bool) -> Self {
        self.show_while_dragging = show_while_dragging;
        self
    }

    /// Element visible while dragging, which follows the cursor.
    pub fn drag_element(mut self, drag_element: impl Into<Element>) -> Self {
        self.drag_element = Some(drag_element.into());
        self
    }

    /// Minimum distance in pixels the cursor must move before dragging starts. Defaults to `4.0`.
    pub fn drag_threshold(mut self, drag_threshold: f64) -> Self {
        self.drag_threshold = drag_threshold;
        self
    }

    /// Whether dragging can start. Defaults to `true`.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

impl<T: Clone + PartialEq> Component for DragZone<T> {
    fn render(&self) -> impl IntoElement {
        let mut drags = use_drag::<T>();
        let mut phase = use_state(|| DragPhase::Idle);
        let mut drag_element_size = use_state(|| None::<Size2D>);
        let data = self.data.clone();
        let drag_threshold = self.drag_threshold;

        let on_global_pointer_move = move |e: Event<PointerEventData>| match phase() {
            DragPhase::Dragging { offset, .. } => {
                phase.set(DragPhase::Dragging {
                    position: e.global_location(),
                    offset,
                });
            }
            DragPhase::Pressing {
                press_point,
                offset,
            } => {
                let current = e.global_location();
                let dx = current.x - press_point.x;
                let dy = current.y - press_point.y;

                if (dx * dx + dy * dy).sqrt() >= drag_threshold {
                    phase.set(DragPhase::Dragging {
                        position: current,
                        offset,
                    });
                    *drags.write() = Some(data.clone());
                }
            }
            DragPhase::Idle => {}
        };

        let on_pointer_down = move |e: Event<PointerEventData>| {
            if e.data().button() != Some(MouseButton::Left) {
                return;
            }
            phase.set(DragPhase::Pressing {
                press_point: e.global_location(),
                offset: e.element_location(),
            });
        };

        let on_global_pointer_press = move |_: Event<PointerEventData>| {
            if !matches!(phase(), DragPhase::Idle) {
                phase.set(DragPhase::Idle);
                *drags.write() = None;
            }
        };

        let dragging = match phase() {
            DragPhase::Dragging { position, offset } => Some((position, offset)),
            _ => None,
        };

        rect()
            .layout(self.layout.clone())
            .on_global_pointer_press(on_global_pointer_press)
            .on_global_pointer_move(on_global_pointer_move)
            .maybe(self.enabled, |el| el.on_pointer_down(on_pointer_down))
            .maybe_child((dragging.zip(self.drag_element.clone())).map(
                |((position, offset), drag_element)| {
                    let size = *drag_element_size.read();
                    let anchor = size.map_or(offset, |size| {
                        offset.min(CursorPoint::new(size.width as f64, size.height as f64))
                    });
                    let (x, y) = (position - anchor).to_f32().to_tuple();
                    rect()
                        .position(Position::new_global())
                        .layer(Layer::Overlay)
                        .interactive(false)
                        .opacity(if size.is_some() { 1. } else { 0. })
                        // Extend by 1. so that the cursor click can reach the drop zone
                        .offset_x(x + 1.)
                        .offset_y(y + 1.)
                        .on_sized(move |e: Event<SizedEventData>| {
                            drag_element_size.set_if_modified(Some(e.area.size))
                        })
                        .child(drag_element)
                },
            ))
            .maybe(self.show_while_dragging || dragging.is_none(), |el| {
                el.children(self.children.clone())
            })
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

/// A container that receives the payload of a [`DragZone`] released over it.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     DropZone::new(|data: usize| println!("Dropped {data}")).child("Drop here!")
/// }
/// ```
#[derive(PartialEq, Clone)]
pub struct DropZone<T: 'static + PartialEq + Clone> {
    children: Vec<Element>,
    on_drop: EventHandler<T>,
    on_drag_over: Option<EventHandler<bool>>,
    layout: LayoutData,
    key: DiffKey,
}

impl<T: Clone + PartialEq + 'static> ChildrenExt for DropZone<T> {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl<T: Clone + PartialEq + 'static> KeyExt for DropZone<T> {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl<T: Clone + PartialEq + 'static> LayoutExt for DropZone<T> {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.layout
    }
}

impl<T: Clone + PartialEq + 'static> ContainerExt for DropZone<T> {}

impl<T: PartialEq + Clone + 'static> DropZone<T> {
    /// Create a drop zone, `on_drop` is called with the payload of the released [`DragZone`].
    pub fn new(on_drop: impl Into<EventHandler<T>>) -> Self {
        Self {
            children: Vec::new(),
            on_drop: on_drop.into(),
            on_drag_over: None,
            layout: LayoutData::default(),
            key: DiffKey::default(),
        }
    }

    /// Called with `true` when a drag enters this zone and `false` when it leaves or is dropped.
    /// Only fires while a drag of `T` is in progress, so it is handy for showing drop previews.
    pub fn on_drag_over(mut self, on_drag_over: impl Into<EventHandler<bool>>) -> Self {
        self.on_drag_over = Some(on_drag_over.into());
        self
    }
}

impl<T: Clone + PartialEq + 'static> Component for DropZone<T> {
    fn render(&self) -> impl IntoElement {
        let mut drags = use_drag::<T>();
        let on_drop = self.on_drop.clone();
        let on_drag_over = self.on_drag_over.clone();

        let on_mouse_up = {
            let on_drag_over = on_drag_over.clone();
            move |e: Event<MouseEventData>| {
                e.stop_propagation();
                let payload = (*drags.read()).clone();
                if let Some(payload) = payload {
                    on_drop.call(payload);
                    *drags.write() = None;
                    if let Some(on_drag_over) = &on_drag_over {
                        on_drag_over.call(false);
                    }
                }
            }
        };

        rect()
            .layout(self.layout.clone())
            .on_mouse_up(on_mouse_up)
            .map(on_drag_over, move |el, on_drag_over| {
                el.on_pointer_enter({
                    let on_drag_over = on_drag_over.clone();
                    move |_| {
                        if drags.read().is_some() {
                            on_drag_over.call(true);
                        }
                    }
                })
                .on_pointer_leave(move |_| {
                    if drags.read().is_some() {
                        on_drag_over.call(false);
                    }
                })
            })
            .children(self.children.clone())
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
