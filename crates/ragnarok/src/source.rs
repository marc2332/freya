use crate::{
    CursorPoint,
    NameOfEvent,
};

pub trait SourceEvent: Clone + PartialEq {
    type Name: NameOfEvent;

    fn is_pressed(&self) -> bool;
    fn is_moved(&self) -> bool;

    fn try_cursor(&self) -> Option<CursorPoint>;

    fn as_event_name(&self) -> Self::Name;
}
