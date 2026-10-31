use alloc::collections::VecDeque;
use core::{
    ops::Add,
    time::Duration,
};

use crate::SourceEvent;

/// Buffers events and combines repeated movement until the next update.
pub struct EventBuffer<Source, Timestamp = Duration> {
    events: VecDeque<Source>,
    deadline: Option<Timestamp>,
    last_update: Option<Timestamp>,
    pub interval: Duration,
}

impl<Source, Timestamp> EventBuffer<Source, Timestamp>
where
    Source: SourceEvent,
    Timestamp: Copy + Ord + Add<Duration, Output = Timestamp>,
{
    pub fn new(interval: Duration) -> Self {
        Self {
            events: VecDeque::new(),
            deadline: None,
            last_update: None,
            interval,
        }
    }

    /// Queues samples while making discrete input ready for immediate processing.
    pub fn push(&mut self, events: impl IntoIterator<Item = Source>, now: Timestamp) {
        for event in events {
            let continuous = event.can_coalesce_with(&event);
            if self
                .events
                .back()
                .is_some_and(|previous| previous.can_coalesce_with(&event))
            {
                self.events.pop_back();
            }
            self.events.push_back(event);

            let deadline = if continuous {
                self.last_update
                    .map(|last| (last + self.interval).max(now))
                    .unwrap_or(now)
            } else {
                now
            };
            self.deadline = Some(
                self.deadline
                    .map(|previous| previous.min(deadline))
                    .unwrap_or(deadline),
            );
        }
    }

    pub fn deadline(&self) -> Option<Timestamp> {
        self.deadline
    }

    pub fn take(&mut self, now: Timestamp) -> VecDeque<Source> {
        self.deadline = None;
        if !self.events.is_empty() {
            self.last_update = Some(now);
        }
        core::mem::take(&mut self.events)
    }
}
