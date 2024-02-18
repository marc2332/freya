use crate::events::{DomEvent, FreyaEvent};
pub use accesskit::NodeId as AccessibilityId;
use dioxus_native_core::NodeId;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::watch;

/// Send focus updates to the Accessibility provider.
pub type FocusSender = watch::Sender<AccessibilityId>;

/// Receive updates by the platform of the focused elements
pub type FocusReceiver = watch::Receiver<AccessibilityId>;

/// Emit events to the VirtualDOM
pub type EventEmitter = UnboundedSender<DomEvent>;

/// Receive events to be emitted to the VirtualDOM
pub type EventReceiver = UnboundedReceiver<DomEvent>;

/// Queued list of events to be processed by Freya.
pub type EventsQueue = SmallVec<[FreyaEvent; 2]>;

/// Potential events that might be emitted.
pub type PotentialEvents = FxHashMap<String, Vec<(NodeId, FreyaEvent)>>;
