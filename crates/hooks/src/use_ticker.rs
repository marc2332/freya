use std::sync::Arc;

use dioxus_core::ScopeState;
use tokio::sync::broadcast;

pub type Ticker = broadcast::Receiver<()>;

#[derive(Clone)]
pub struct UseTicker {
    ticker: Arc<Ticker>,
}

impl UseTicker {
    pub fn new(ticker: Arc<Ticker>) -> Self {
        Self { ticker }
    }

    pub fn new_subscriber(&self) -> Ticker {
        self.ticker.resubscribe()
    }
}

pub fn use_ticker(cx: &ScopeState) -> UseTicker {
    UseTicker::new(
        cx.consume_context::<Arc<Ticker>>()
            .expect("This is not expected, and likely a bug. Please, report it."),
    )
}
