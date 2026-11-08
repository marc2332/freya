use crate::{
    CursorPoint,
    NameOfEvent,
    NodeKey,
};

pub trait SourceEvent: Clone + PartialEq {
    type Name: NameOfEvent;
    type Key: NodeKey;

    fn is_pressed(&self) -> bool;
    fn is_moved(&self) -> bool;
    fn is_touch_released(&self) -> bool;
    fn is_pointer_exit(&self) -> bool {
        false
    }

    fn is_drag_over(&self) -> bool {
        self.as_event_name().is_drag_over()
    }

    fn is_drag_leave(&self) -> bool {
        self.as_event_name().is_drag_leave()
    }

    fn is_drag(&self) -> bool {
        let name = self.as_event_name();
        self.is_drag_over() || self.is_drag_leave() || name.is_drag_enter() || name.is_drop()
    }

    /// The accepted drop target, or `None` if this drop was rejected.
    fn drop_target(&self) -> Option<Self::Key> {
        None
    }

    /// Whether these events can be combined.
    fn can_coalesce_with(&self, _next: &Self) -> bool {
        false
    }

    /// Combines a newer compatible event, replacing this event by default.
    fn coalesce_with(&mut self, next: Self) {
        *self = next;
    }

    fn try_location(&self) -> Option<CursorPoint>;

    fn as_event_name(&self) -> Self::Name;
}
