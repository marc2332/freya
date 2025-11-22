use freya_core::{
    prelude::*,
    scope_id::ScopeId,
};
use torin::prelude::*;

fn use_drag<T: 'static>() -> State<Option<T>> {
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
}

impl<T: Clone + PartialEq + 'static> DragZone<T> {
    pub fn new(data: T, children: impl Into<Element>) -> Self {
        Self {
            data,
            children: children.into(),
            drag_element: None,
            show_while_dragging: true,
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
}

impl<T: Clone + PartialEq> Render for DragZone<T> {
    fn render(&self) -> impl IntoElement {
        let mut drags = use_drag::<T>();
        let mut position = use_state::<Option<CursorPoint>>(|| None);
        let data = self.data.clone();

        let on_global_mouse_move = move |e: Event<MouseEventData>| {
            if position.read().is_some() {
                position.set(Some(e.global_location));
            }
        };

        let on_pointer_down = move |e: Event<PointerEventData>| {
            if e.data().button() != Some(MouseButton::Left) {
                return;
            }
            position.set(Some(e.global_location()));
            *drags.write() = Some(data.clone());
        };

        let on_global_mouse_up = move |_: Event<MouseEventData>| {
            if position.read().is_some() {
                position.set(None);
                *drags.write() = None;
            }
        };

        rect()
            .on_global_mouse_up(on_global_mouse_up)
            .on_global_mouse_move(on_global_mouse_move)
            .on_pointer_down(on_pointer_down)
            .maybe_child((position.read().zip(self.drag_element.clone())).map(
                |(position, drag_element)| {
                    let (x, y) = position.to_f32().to_tuple();
                    rect()
                        .position(Position::new_global())
                        .width(Size::px(0.))
                        .height(Size::px(0.))
                        // Extend by 1. so that the cursor click can reach the drop zone
                        .offset_x(x + 1.)
                        .offset_y(y + 1.)
                        .child(drag_element)
                },
            ))
            .maybe_child(
                (self.show_while_dragging || position.read().is_none())
                    .then(|| self.children.clone()),
            )
    }
}

#[derive(PartialEq, Clone)]
pub struct DropZone<T: 'static + PartialEq + Clone> {
    children: Element,
    on_drop: EventHandler<T>,
    width: Size,
    height: Size,
}

impl<T: PartialEq + Clone + 'static> DropZone<T> {
    pub fn new(children: impl Into<Element>, on_drop: impl Into<EventHandler<T>>) -> Self {
        Self {
            children: children.into(),
            on_drop: on_drop.into(),
            width: Size::auto(),
            height: Size::auto(),
        }
    }
}

impl<T: Clone + PartialEq + 'static> Render for DropZone<T> {
    fn render(&self) -> impl IntoElement {
        let mut drags = use_drag::<T>();
        let on_drop = self.on_drop.clone();

        let on_mouse_up = move |e: Event<MouseEventData>| {
            e.stop_propagation();
            if let Some(current_drags) = &*drags.read() {
                on_drop.call(current_drags.clone());
            }
            if drags.read().is_some() {
                *drags.write() = None;
            }
        };

        rect()
            .on_mouse_up(on_mouse_up)
            .width(self.width.clone())
            .height(self.height.clone())
            .child(self.children.clone())
    }
}
