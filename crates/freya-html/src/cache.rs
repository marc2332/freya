use std::sync::{
    Arc,
    Mutex,
};

use blitz_traits::net::Bytes;
use freya_core::prelude::GlobalContexts;
use rustc_hash::FxHashMap;
use url::Url;

/// Shared cache for resources downloaded by HTML viewers.
#[derive(Clone, Default)]
pub struct ResourceCache {
    resources: Arc<Mutex<FxHashMap<Url, Bytes>>>,
}

impl ResourceCache {
    /// Retrieves or initializes the shared cache in the current app context.
    pub fn get() -> Self {
        GlobalContexts::get().get_context_or_insert(Self::default)
    }

    pub fn read(&self, url: &Url) -> Option<Bytes> {
        self.resources
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .get(url)
            .cloned()
    }

    pub fn set(&self, url: Url, bytes: Bytes) {
        self.resources
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .insert(url, bytes);
    }

    pub fn invalidate(&self, url: &Url) {
        self.resources
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .remove(url);
    }

    pub fn clear(&self) {
        self.resources
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .clear();
    }
}
