use std::time::{
    Duration,
    Instant,
};

use async_io::Timer;
use freya_core::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub struct Timeout {
    elapsed: State<bool>,
    instant: State<Instant>,
}

impl Timeout {
    /// Check if the timeout has passed its specified [Duration].
    pub fn elapsed(&self) -> bool {
        (self.elapsed)()
    }

    /// Reset the timer.
    pub fn reset(&mut self) {
        self.instant.set_if_modified(Instant::now());
        self.elapsed.set_if_modified(false);
    }
}

/// Create a timeout with a given [Duration].
/// This is useful to dinamically render a UI if only the timeout has not elapsed yet.
///
/// You can reset it by calling [Timeout::reset],
/// use [Timeout::elapsed] to check if it has timed out or not.
pub fn use_timeout(duration: impl FnOnce() -> Duration) -> Timeout {
    use_hook(|| {
        let duration = duration();
        let mut elapsed = State::create(false);
        let instant = State::create(Instant::now());

        spawn(async move {
            loop {
                Timer::after(duration).await;
                if instant.read().elapsed() >= duration && !elapsed() {
                    elapsed.set(true);
                }
            }
        });

        Timeout { elapsed, instant }
    })
}
