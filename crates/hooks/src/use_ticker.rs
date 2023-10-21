use std::sync::Arc;

use dioxus_core::ScopeState;
use tokio::sync::broadcast;

pub struct Ticker {
    inner: broadcast::Receiver<()>,
}

impl Ticker {
    pub async fn tick(&mut self) {
        self.inner.recv().await.ok();
    }
}

#[derive(Clone)]
pub struct UseTicker {
    ticker: Arc<broadcast::Receiver<()>>,
}

impl UseTicker {
    pub fn new(ticker: Arc<broadcast::Receiver<()>>) -> Self {
        Self { ticker }
    }

    pub fn new_subscriber(&self) -> Ticker {
        Ticker {
            inner: self.ticker.resubscribe(),
        }
    }
}

pub fn use_ticker(cx: &ScopeState) -> UseTicker {
    UseTicker::new(
        cx.consume_context::<Arc<broadcast::Receiver<()>>>()
            .expect("This is not expected, and likely a bug. Please, report it."),
    )
}
