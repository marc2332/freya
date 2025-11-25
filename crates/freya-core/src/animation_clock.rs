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

#[derive(Clone)]
pub struct AnimationClock(Arc<AtomicU32>);

impl Default for AnimationClock {
    fn default() -> Self {
        Self::new()
    }
}

impl AnimationClock {
    pub const DEFAULT_SPEED: f32 = 1.0;
    pub const MIN_SPEED: f32 = 0.05;
    pub const MAX_SPEED: f32 = 5.0;

    pub fn get() -> Self {
        consume_root_context()
    }

    pub fn new() -> Self {
        Self(Arc::new(AtomicU32::new(Self::DEFAULT_SPEED.to_bits())))
    }

    pub fn speed(&self) -> f32 {
        let bits = self.0.load(Ordering::Relaxed);
        (f32::from_bits(bits).clamp(Self::MIN_SPEED, Self::MAX_SPEED) * 100.0).round() / 100.0
    }

    pub fn set_speed(&self, speed: f32) {
        let speed = speed.clamp(Self::MIN_SPEED, Self::MAX_SPEED);
        self.0.store(speed.to_bits(), Ordering::Relaxed);
        info!("Animation clock speed changed to {:.2}x", speed);
    }

    pub fn correct_elapsed_duration(&self, elapsed: Duration) -> Duration {
        let scaled_secs = elapsed.as_secs_f32() * self.speed();
        Duration::from_secs_f32(scaled_secs)
    }
}
