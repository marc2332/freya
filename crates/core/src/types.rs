pub use accesskit::{
    Node as AccessibilityNode,
    NodeId as AccessibilityId,
};
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
        PlatformEventName,
        PotentialEvent,
    },
    platform_state::NativePlatformState,
};

/// Send platform updates from the platform
pub type NativePlatformSender = watch::Sender<NativePlatformState>;

/// Receive updates by the platform
pub type NativePlatformReceiver = watch::Receiver<NativePlatformState>;

/// Emit events to the VirtualDOM
pub type EventEmitter = UnboundedSender<(Vec<DomEvent>, FlattenedPotentialEvents)>;

/// Receive events to be emitted to the VirtualDOM
pub type EventReceiver = UnboundedReceiver<(Vec<DomEvent>, FlattenedPotentialEvents)>;

/// Queued list of events to be processed by Freya.
pub type EventsQueue = SmallVec<[PlatformEvent; 2]>;

/// Potential events that might be emitted.
pub type PotentialEvents = FxHashMap<PlatformEventName, Vec<PotentialEvent>>;

pub type FlattenedPotentialEvents = Vec<PotentialEvent>;
