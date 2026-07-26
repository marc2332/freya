use crate::prelude::consume_root_context;

pub type RenderingTickerSender = async_watch::Sender<()>;

/// Receives frame notifications.
#[derive(Clone)]
pub struct RenderingTicker {
    rx: async_watch::Receiver<()>,
}

impl RenderingTicker {
    pub fn get() -> Self {
        consume_root_context()
    }

    pub fn new() -> (RenderingTickerSender, Self) {
        let (tx, rx) = async_watch::channel(());
        (tx, Self { rx })
    }

    /// Wait until the next frame should be processed.
    pub async fn tick(&mut self) {
        self.rx.changed().await.ok();
    }
}
