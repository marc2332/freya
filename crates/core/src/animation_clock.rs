use std::{
    sync::{
        atomic::{
            AtomicU32,
            Ordering,
        },
        Arc,
    },
    time::Duration,
};

use tracing::info;

#[derive(Clone)]
pub struct AnimationClock(Arc<AtomicU32>); // Stores f32 as bits

impl Default for AnimationClock {
    fn default() -> Self {
        Self::new()
    }
}

impl AnimationClock {
    const DEFAULT_SPEED: f32 = 1.0;
    const MIN_SPEED: f32 = 0.1;
    const MAX_SPEED: f32 = 10.0;

    pub fn new() -> Self {
        Self(Arc::new(AtomicU32::new(Self::DEFAULT_SPEED.to_bits())))
    }

    pub fn speed(&self) -> f32 {
        let bits = self.0.load(Ordering::Relaxed);
        (f32::from_bits(bits).clamp(Self::MIN_SPEED, Self::MAX_SPEED) * 100.0).round() / 100.0
    }

    pub(crate) fn set_speed(&self, speed: f32) {
        let speed = speed.clamp(Self::MIN_SPEED, Self::MAX_SPEED);
        self.0.store(speed.to_bits(), Ordering::Relaxed);
        info!("Animation clock speed changed to {:.2}x", speed);
    }

    pub fn increase_by(&self, factor: f32) {
        let current = self.speed();
        self.set_speed(current + factor);
    }

    pub fn decrease_by(&self, factor: f32) {
        let current = self.speed();
        self.set_speed(current - factor);
    }

    pub fn correct_elapsed_duration(&self, elapsed: Duration) -> Duration {
        let scaled_secs = elapsed.as_secs_f32() * self.speed();
        Duration::from_secs_f32(scaled_secs)
    }
}
