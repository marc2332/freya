use crate::events::{DomEvent, FreyaEvent};
use dioxus_native_core::NodeId;

pub use accesskit::NodeId as AccessibilityId;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::watch;

pub type FocusSender = watch::Sender<AccessibilityId>;
pub type FocusReceiver = watch::Receiver<AccessibilityId>;
pub type EventEmitter = UnboundedSender<DomEvent>;
pub type EventReceiver = UnboundedReceiver<DomEvent>;
pub type EventsQueue = SmallVec<[FreyaEvent; 2]>;
pub type NodesEvents = FxHashMap<String, Vec<(NodeId, FreyaEvent)>>;
