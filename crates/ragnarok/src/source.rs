use crate::{
    CursorPoint,
    NameOfEvent,
};

pub trait SourceEvent: Clone + PartialEq {
    type Name: NameOfEvent;

    fn is_pressed(&self) -> bool;
    fn is_moved(&self) -> bool;
    fn is_touch_released(&self) -> bool;
    fn is_pointer_exit(&self) -> bool {
        false
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
