use std::sync::{
    Arc,
    atomic::{
        AtomicBool,
        Ordering,
    },
};

use crate::prelude::consume_root_context;

#[derive(Clone)]
pub struct ScreenReader {
    is_on: Arc<AtomicBool>,
}

impl Default for ScreenReader {
    fn default() -> Self {
        Self::new()
    }
}

impl ScreenReader {
    pub fn new() -> Self {
        Self {
            is_on: Arc::default(),
        }
    }

    pub fn get() -> Self {
        consume_root_context::<Self>()
    }

    pub fn set(&self, on: bool) {
        self.is_on.store(on, Ordering::Relaxed);
    }

    /// Check if the OS screen reader is running right now.
    pub fn is_on(&self) -> bool {
        self.is_on.load(Ordering::Relaxed)
    }
}
