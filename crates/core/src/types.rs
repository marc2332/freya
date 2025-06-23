pub use accesskit::{
    Node as AccessibilityNode,
    NodeId as AccessibilityId,
};
use freya_native_core::{
    events::EventName,
    NodeId,
};
use ragnarok::ProcessedEvents;
use tokio::sync::{
    mpsc::{
        UnboundedReceiver,
        UnboundedSender,
    },
    watch,
};

use crate::{
    events::{
        DomEvent, PlatformEvent
    },
    platform_state::NativePlatformState,
};

/// Send platform updates from the platform
pub type NativePlatformSender = watch::Sender<NativePlatformState>;

/// Receive updates by the platform
pub type NativePlatformReceiver = watch::Receiver<NativePlatformState>;

/// Emit events to the VirtualDOM
pub type EventEmitter =
    UnboundedSender<ProcessedEvents<NodeId, EventName, DomEvent, PlatformEvent>>;

/// Receive events to be emitted to the VirtualDOM
pub type EventReceiver =
    UnboundedReceiver<ProcessedEvents<NodeId, EventName, DomEvent, PlatformEvent>>;

/// Queued list of events to be processed by Freya.
pub type EventsQueue = Vec<PlatformEvent>;
