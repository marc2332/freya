use std::sync::{
    Arc,
    atomic::{
        AtomicU64,
        Ordering,
    },
};

pub use accesskit::{
    NodeId as AccessibilityId,
    Role as AccessibilityRole,
};

#[derive(Clone)]
pub struct AccessibilityGenerator {
    counter: Arc<AtomicU64>,
}

impl Default for AccessibilityGenerator {
    fn default() -> Self {
        Self {
            counter: Arc::new(AtomicU64::new(1)), // Must start at 1 because 0 is reserved for the Root
        }
    }
}

impl AccessibilityGenerator {
    pub fn new_id(&self) -> u64 {
        self.counter.fetch_add(1, Ordering::Relaxed)
    }
}
