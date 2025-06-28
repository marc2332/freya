use crate::{
    NameOfEvent,
    NodeKey,
};

pub trait EmmitableEvent: Clone + PartialEq + Eq + Ord {
    type Name: NameOfEvent;
    type Key: NodeKey;

    /// Get the name of this event.
    fn name(&self) -> Self::Name;
    /// Get the name of the source event of this event. For example, the source event of a mouseenter would be mouse movement.
    fn source(&self) -> Self::Name;
    /// The node key targeted by this event.
    fn key(&self) -> Self::Key;
}
