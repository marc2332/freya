use crate::prelude::try_consume_root_context;

pub type RenderingTickerSender = async_broadcast::Sender<()>;

#[derive(Clone)]
pub struct RenderingTicker {
    rx: async_broadcast::Receiver<()>,
}

impl RenderingTicker {
    pub fn get() -> Self {
        try_consume_root_context::<Self>().unwrap()
    }
    pub fn new() -> (async_broadcast::Sender<()>, Self) {
        let (tx, rx) = async_broadcast::broadcast(256);
        (tx, Self { rx })
    }

    pub async fn tick(&mut self) {
        self.rx.recv().await.ok();
    }
}
