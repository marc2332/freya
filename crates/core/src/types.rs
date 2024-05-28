use crate::{
    events::DomEvent,
    prelude::{EventName, PlatformEvent, PotentialEvent},
};
pub use accesskit::NodeId as AccessibilityId;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::watch;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum PreferredTheme {
    #[default]
    /// Use the light variant.
    Light,

    /// Use the dark variant.
    Dark,
}

impl From<winit::window::Theme> for PreferredTheme {
    fn from(value: winit::window::Theme) -> Self {
        match value {
            winit::window::Theme::Light => Self::Light,
            winit::window::Theme::Dark => Self::Dark,
        }
    }
}

pub struct NativePlatformState {
    pub focused_id: AccessibilityId,
    pub preferred_theme: PreferredTheme,
}

/// Send focus updates to the platform
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
