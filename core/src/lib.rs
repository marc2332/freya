use dioxus_native_core::NodeId;
use dom_events::DomEvent;
use freya_events::FreyaEvent;

use accesskit::NodeId as AccessibilityId;
use rustc_hash::FxHashMap;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::watch;
use torin::prelude::Area;

pub mod accessibility;
pub mod accessibility_state;
pub mod dom_events;
pub mod events;
pub mod events_processor;
pub mod freya_events;
pub mod layers;
pub mod layout;
pub mod node;
pub mod render;
pub mod viewports;

pub type FocusSender = watch::Sender<Option<AccessibilityId>>;
pub type FocusReceiver = watch::Receiver<Option<AccessibilityId>>;
pub type EventEmitter = UnboundedSender<DomEvent>;
pub type EventReceiver = UnboundedReceiver<DomEvent>;
pub type EventsQueue = Vec<FreyaEvent>;
pub type NodesEvents = FxHashMap<String, Vec<(NodeId, FreyaEvent)>>;
pub type ViewportsCollection = FxHashMap<NodeId, (Option<Area>, Vec<NodeId>)>;

pub mod prelude {
    pub use crate::accessibility::*;
    pub use crate::accessibility_state::*;
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
    pub use crate::FocusReceiver;
    pub use crate::FocusSender;
    pub use crate::NodesEvents;
    pub use crate::ViewportsCollection;
}
