use std::{rc::Rc, sync::Arc};

use dioxus_core::ScopeState;
use tokio::sync::broadcast;

pub type Ticker = broadcast::Receiver<()>;

pub fn use_ticker(cx: &ScopeState) -> Ticker {
    cx.consume_context::<Arc<Ticker>>().unwrap().resubscribe()
}