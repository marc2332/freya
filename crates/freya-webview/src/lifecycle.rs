use std::sync::{
    Arc,
    Mutex,
};

use freya_core::prelude::try_consume_root_context;
use torin::prelude::Area;

use crate::registry::{
    WebViewConfig,
    WebViewId,
};

#[derive(Debug, Clone)]
pub enum WebViewLifecycleEvent {
    Resized {
        id: WebViewId,
        area: Area,
        config: WebViewConfig,
    },
    Close {
        id: WebViewId,
    },
    Hide {
        id: WebViewId,
    },
}

pub type WebViewEvents = Arc<Mutex<Vec<WebViewLifecycleEvent>>>;

pub struct WebViewManager;

impl WebViewManager {
    fn get() -> WebViewEvents {
        try_consume_root_context()
            .expect("WebViewManager failed to initialize. You must load the WebViewPlugin.")
    }

    pub fn close(id: WebViewId) {
        Self::get()
            .lock()
            .unwrap()
            .push(WebViewLifecycleEvent::Close { id });
    }
}
