use std::{
    sync::{
        Arc,
        atomic::{
            AtomicU32,
            Ordering,
        },
    },
    time::Duration,
};

use tracing::info;

use crate::prelude::consume_root_context;

/// Controls the animation speed of this window.
#[derive(Clone)]
pub struct AnimationClock(Arc<AtomicU32>);

#[doc(hidden)]
pub trait New: sealed::Sealed {
    fn new() -> Self;
}

impl New for AnimationClock {
    fn new() -> Self {
        Self(Arc::new(AtomicU32::new(Self::DEFAULT_SPEED.to_bits())))
    }
}

mod sealed {
    pub trait Sealed {}
    impl Sealed for crate::animation_clock::AnimationClock {}
}

impl AnimationClock {
    /// Normal speed.
    pub const DEFAULT_SPEED: f32 = 1.0;
    /// Lowest allowed speed.
    pub const MIN_SPEED: f32 = 0.05;
    /// Highest allowed speed.
    pub const MAX_SPEED: f32 = 5.0;
    /// Speed that makes animations finish instantly.
    pub const DISABLED_SPEED: f32 = 999_999_999.0;

    /// Retrieve the current [`AnimationClock`] from the root context.
    pub fn get() -> Self {
        consume_root_context()
    }

    /// Get the current speed.
    pub fn speed(&self) -> f32 {
        let bits = self.0.load(Ordering::Relaxed);
        (f32::from_bits(bits) * 100.0).round() / 100.0
    }

    /// Enable animations.
    pub fn enable(&self) {
        self.0
            .store(Self::DEFAULT_SPEED.to_bits(), Ordering::Relaxed);
        info!("Animation clock speed enabled");
    }

    /// Disable animations.
    pub fn disable(&self) {
        self.0
            .store(Self::DISABLED_SPEED.to_bits(), Ordering::Relaxed);
        info!("Animation clock speed disabled");
    }

    /// Update the speed.
    pub fn set_speed(&self, speed: f32) {
        let speed = speed.clamp(Self::MIN_SPEED, Self::MAX_SPEED);
        self.0.store(speed.to_bits(), Ordering::Relaxed);
        info!("Animation clock speed changed to {:.2}x", speed);
    }

    /// Scale a duration by the current speed.
    pub fn correct_elapsed_duration(&self, elapsed: Duration) -> Duration {
        let scaled_secs = elapsed.as_secs_f32() * self.speed();
        Duration::from_secs_f32(scaled_secs)
    }
}
