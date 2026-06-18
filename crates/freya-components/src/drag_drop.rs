use freya_core::{
    prelude::*,
    scope_id::ScopeId,
};
use torin::prelude::*;

#[derive(Clone, Copy)]
enum DragPhase {
    Idle,
    Pressing(CursorPoint),
    Dragging(CursorPoint),
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

/// Properties for the [`DragZone`] component.
#[derive(Clone, PartialEq)]
pub struct DragZone<T: Clone + 'static + PartialEq> {
    /// Element visible when dragging the element. This follows the cursor.
    drag_element: Option<Element>,
    /// Inner children for the DropZone.
    children: Element,
    /// Data that will be handled to the destination [`DropZone`].
    data: T,
    /// Show the children when dragging. Defaults to `true`.
    show_while_dragging: bool,
    /// Minimum distance in pixels the cursor must move before dragging starts. Defaults to `4.0`.
    drag_threshold: f64,
    key: DiffKey,
}

impl<T: Clone + PartialEq + 'static> KeyExt for DragZone<T> {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl<T: Clone + PartialEq + 'static> DragZone<T> {
    pub fn new(data: T, children: impl Into<Element>) -> Self {
        Self {
            data,
            children: children.into(),
            drag_element: None,
            show_while_dragging: true,
            drag_threshold: 4.0,
            key: DiffKey::default(),
        }
    }

    pub fn show_while_dragging(mut self, show_while_dragging: bool) -> Self {
        self.show_while_dragging = show_while_dragging;
        self
    }

    pub fn drag_element(mut self, drag_element: impl Into<Element>) -> Self {
        self.drag_element = Some(drag_element.into());
        self
    }

    pub fn drag_threshold(mut self, drag_threshold: f64) -> Self {
        self.drag_threshold = drag_threshold;
        self
    }
}

impl<T: Clone + PartialEq> Component for DragZone<T> {
    fn render(&self) -> impl IntoElement {
        let mut drags = use_drag::<T>();
        let mut phase = use_state(|| DragPhase::Idle);
        let data = self.data.clone();
        let drag_threshold = self.drag_threshold;

        let on_global_pointer_move = move |e: Event<PointerEventData>| match phase() {
            DragPhase::Dragging(_) => {
                phase.set(DragPhase::Dragging(e.global_location()));
            }
            DragPhase::Pressing(press_point) => {
                let current = e.global_location();
                let dx = current.x - press_point.x;
                let dy = current.y - press_point.y;

                if (dx * dx + dy * dy).sqrt() >= drag_threshold {
                    phase.set(DragPhase::Dragging(current));
                    *drags.write() = Some(data.clone());
                }
            }
            DragPhase::Idle => {}
        };

        let on_pointer_down = move |e: Event<PointerEventData>| {
            if e.data().button() != Some(MouseButton::Left) {
                return;
            }
            phase.set(DragPhase::Pressing(e.global_location()));
        };

        let on_global_pointer_press = move |_: Event<PointerEventData>| {
            if !matches!(phase(), DragPhase::Idle) {
                phase.set(DragPhase::Idle);
                *drags.write() = None;
            }
        };

        let dragging_position = match phase() {
            DragPhase::Dragging(pos) => Some(pos),
            _ => None,
        };

        rect()
            .on_global_pointer_press(on_global_pointer_press)
            .on_global_pointer_move(on_global_pointer_move)
            .on_pointer_down(on_pointer_down)
            .maybe_child((dragging_position.zip(self.drag_element.clone())).map(
                |(position, drag_element)| {
                    let (x, y) = position.to_f32().to_tuple();
                    rect()
                        .position(Position::new_global())
                        .layer(Layer::Overlay)
                        .interactive(false)
                        .width(Size::px(0.))
                        .height(Size::px(0.))
                        // Extend by 1. so that the cursor click can reach the drop zone
                        .offset_x(x + 1.)
                        .offset_y(y + 1.)
                        .child(drag_element)
                },
            ))
            .maybe_child(
                (self.show_while_dragging || dragging_position.is_none())
                    .then(|| self.children.clone()),
            )
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

#[derive(PartialEq, Clone)]
pub struct DropZone<T: 'static + PartialEq + Clone> {
    children: Element,
    on_drop: EventHandler<T>,
    on_drag_over: Option<EventHandler<bool>>,
    width: Size,
    height: Size,
    key: DiffKey,
}

impl<T: Clone + PartialEq + 'static> KeyExt for DropZone<T> {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl<T: PartialEq + Clone + 'static> DropZone<T> {
    pub fn new(children: impl Into<Element>, on_drop: impl Into<EventHandler<T>>) -> Self {
        Self {
            children: children.into(),
            on_drop: on_drop.into(),
            on_drag_over: None,
            width: Size::auto(),
            height: Size::auto(),
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
                if let Some(current_drags) = &*drags.read() {
                    on_drop.call(current_drags.clone());
                }
                if drags.read().is_some() {
                    *drags.write() = None;
                    if let Some(on_drag_over) = &on_drag_over {
                        on_drag_over.call(false);
                    }
                }
            }
        };

        rect()
            .on_mouse_up(on_mouse_up)
            .width(self.width.clone())
            .height(self.height.clone())
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
            .child(self.children.clone())
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
