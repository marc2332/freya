use crate::{
    events::DomEvent,
    prelude::{EventName, NativePlatformState, PlatformEvent, PotentialEvent},
};
pub use accesskit::NodeId as AccessibilityId;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::watch;

/// Send platform updates from the platform
pub type NativePlatformSender = watch::Sender<NativePlatformState>;

/// Receive updates by the platform
pub type NativePlatformReceiver = watch::Receiver<NativePlatformState>;

/// Emit events to the VirtualDOM
pub type EventEmitter = UnboundedSender<DomEvent>;

/// Receive events to be emitted to the VirtualDOM
pub type EventReceiver = UnboundedReceiver<DomEvent>;

/// Queued list of events to be processed by Freya.
pub type EventsQueue = SmallVec<[PlatformEvent; 2]>;

/// Potential events that might be emitted.
pub type PotentialEvents = FxHashMap<EventName, Vec<PotentialEvent>>;
