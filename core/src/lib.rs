use dioxus_native_core::NodeId;
use dom_events::DomEvent;
use freya_events::FreyaEvent;

use rustc_hash::FxHashMap;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use torin::prelude::Area;

pub mod dom_events;
pub mod events;
pub mod events_processor;
pub mod freya_events;
pub mod layers;
pub mod layout;
pub mod node;
pub mod render;
pub mod viewports;

pub type EventEmitter = UnboundedSender<DomEvent>;
pub type EventReceiver = UnboundedReceiver<DomEvent>;
pub type EventsQueue = Vec<FreyaEvent>;
pub type NodesEvents = FxHashMap<String, Vec<(NodeId, FreyaEvent)>>;
pub type ViewportsCollection = FxHashMap<NodeId, (Option<Area>, Vec<NodeId>)>;

pub mod prelude {
    pub use crate::dom_events::*;
    pub use crate::events::*;
    pub use crate::events_processor::*;
    pub use crate::freya_events::*;
    pub use crate::layers::*;
    pub use crate::layout::*;
    pub use crate::node::*;
    pub use crate::render::*;
    pub use crate::viewports::*;

    pub use crate::EventEmitter;
    pub use crate::EventReceiver;
    pub use crate::EventsQueue;
    pub use crate::NodesEvents;
    pub use crate::ViewportsCollection;
}
