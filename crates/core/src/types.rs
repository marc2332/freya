pub use accesskit::{
    Node as AccessibilityNode,
    NodeId as AccessibilityId,
};
use freya_native_core::events::EventName;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;
use tokio::sync::{
    mpsc::{
        UnboundedReceiver,
        UnboundedSender,
    },
    watch,
};

use crate::{
    events::{
        DomEvent,
        PlatformEvent,
        PotentialEvent,
    },
    platform_state::NativePlatformState,
};

/// Send platform updates from the platform
pub type NativePlatformSender = watch::Sender<NativePlatformState>;

/// Receive updates by the platform
pub type NativePlatformReceiver = watch::Receiver<NativePlatformState>;

/// Emit events to the VirtualDOM
pub type EventEmitter = UnboundedSender<Vec<DomEvent>>;

/// Receive events to be emitted to the VirtualDOM
pub type EventReceiver = UnboundedReceiver<Vec<DomEvent>>;

/// Queued list of events to be processed by Freya.
pub type EventsQueue = SmallVec<[PlatformEvent; 2]>;

/// Potential events that might be emitted.
pub type PotentialEvents = FxHashMap<EventName, Vec<PotentialEvent>>;
