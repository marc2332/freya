use crate::{
    notify::BroadcastNotify,
    prelude::consume_root_context,
};

pub type RenderingTickerSender = BroadcastNotify;

/// Receives frame notifications.
#[derive(Clone)]
pub struct RenderingTicker {
    notify: BroadcastNotify,
}

impl RenderingTicker {
    pub fn get() -> Self {
        consume_root_context()
    }

    pub fn new() -> (RenderingTickerSender, Self) {
        let notify = BroadcastNotify::new();
        (notify.clone(), Self { notify })
    }

    /// Wait until the next frame should be processed.
    pub async fn tick(&self) {
        self.notify.notified().await;
    }
}
