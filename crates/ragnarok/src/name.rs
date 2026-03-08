use std::collections::HashSet;

pub trait NameOfEvent:
    Clone + PartialEq + Eq + std::hash::Hash + Copy + std::fmt::Debug + Eq + Ord
{
    /// Check if this event means that the pointer device as moved while hovering a node.
    fn is_moved(&self) -> bool;
    /// Check if this event means that the pointer device started hovering a node.
    fn is_enter(&self) -> bool;
    /// Check if this enter event should only fire on the deepest (most specific) node,
    /// not on all nodes under the cursor simultaneously.
    fn is_exclusive_enter(&self) -> bool {
        false
    }
    /// Check if this event means that the pointer device was pressed while hovering a node.
    fn is_pressed(&self) -> bool;
    /// Check if this event means that the pointer device was released while hovering and hovering a node.
    fn is_released(&self) -> bool;
    /// Check if this event is global, where global means that an event will be emitted to every node independently of where they are or how they are.
    fn is_global(&self) -> bool;

    /// Check if this event bubbles, where bubbling means that an ancestor of the event node will be called with the same event unless the event node stops the bubbling.
    fn does_bubble(&self) -> bool;
    /// Check if this event can go through solid surfaces, e.g keyboard events.
    fn does_go_through_solid(&self) -> bool;

    /// Check if this leave event corresponds to an exclusive enter
    fn is_exclusive_leave(&self) -> bool {
        false
    }

    /// Create a new event that means the pointer device left a hovering node.
    fn new_leave() -> Self;
    /// Create a new exclusive leave event for when the deepest (entered) node changes
    /// without the node necessarily leaving the cursor area entirely.
    fn new_exclusive_leave() -> Self;
    /// Create a new exclusive leave event for when the deepest (entered) node changes
    /// without the node necessarily leaving the cursor area entirely.
    fn new_exclusive_enter() -> Self;

    /// Get a set of events derived from this event. For example, mouse movement derives into mouse movement + mouse enter.
    fn get_derived_events(&self) -> HashSet<Self>;
    /// Get a set of global events derived from this event.
    fn get_global_events(&self) -> HashSet<Self> {
        HashSet::new()
    }
    /// Get a set of events that will be discarded once this event is cancelled.
    fn get_cancellable_events(&self) -> HashSet<Self> {
        HashSet::from([*self])
    }
}
