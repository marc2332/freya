//! **Ragnarok** is a UI events processing library that works by receiving a set of `Source` events
//! (platform events such as mouse movement, touch events, keyboard, etc),
//! deriving `Potential` events which are used to create and update an internal state
//! of the current nodes states (e.g if X node is being hovered or not independently
//! of that node listening for such event or not), and later generating a list of `Emmitable` events
//! that are sent to the consumer of **Raganarok** and ultimately if an `Emmitable` event is cancelled
//! this will discard some of the yet-to-emit `Emmitable` events and possibly affect the internal state of the nodes.

pub mod emmitable;
pub mod executor;
pub mod key;
pub mod measurement;
pub mod measurer;
pub mod name;
pub mod nodes_state;
pub mod potential_event;
pub mod source;

pub use emmitable::*;
pub use executor::*;
pub use key::*;
pub(crate) use measurement::*;
pub use measurer::*;
pub use name::*;
pub use nodes_state::*;
pub(crate) use potential_event::*;
pub use source::*;

pub type CursorPoint = euclid::Point2D<f64, ()>;
pub type Area = euclid::Rect<f32, ()>;
