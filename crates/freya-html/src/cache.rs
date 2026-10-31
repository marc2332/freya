use std::{
    collections::VecDeque,
    sync::{
        Arc,
        Mutex,
    },
};

use blitz_traits::net::Bytes;
use freya_core::prelude::GlobalContexts;
use url::Url;

#[derive(Default)]
struct CachedResources {
    entries: VecDeque<(Url, Bytes)>,
    bytes: usize,
}

impl CachedResources {
    fn remove(&mut self, url: &Url) -> Option<(Url, Bytes)> {
        let index = self
            .entries
            .iter()
            .position(|(cached_url, _)| cached_url == url)?;
        let entry = self.entries.remove(index)?;
        self.bytes -= entry.1.len();
        Some(entry)
    }
}

/// Shared LRU cache with a 32 MiB default body budget.
#[derive(Clone)]
pub struct ResourceCache {
    resources: Arc<Mutex<CachedResources>>,
    max_bytes: usize,
}

impl Default for ResourceCache {
    fn default() -> Self {
        Self::new(32 * 1024 * 1024)
    }
}

impl ResourceCache {
    /// Limits cached bodies to `max_bytes`, with zero disabling caching.
    ///
    /// ```no_run
    /// use freya_core::prelude::GlobalContexts;
    /// use freya_html::ResourceCache;
    ///
    /// // Configure before creating HTML viewers.
    /// GlobalContexts::get().insert_context(ResourceCache::new(64 * 1024 * 1024));
    /// ```
    pub fn new(max_bytes: usize) -> Self {
        Self {
            resources: Arc::new(Mutex::new(CachedResources::default())),
            max_bytes,
        }
    }

    /// Retrieves or initializes the shared cache in the current app context.
    pub fn get() -> Self {
        GlobalContexts::get().get_context_or_insert(Self::default)
    }

    /// Returns cached bytes and marks the resource as recently used.
    pub fn read(&self, url: &Url) -> Option<Bytes> {
        let mut resources = self
            .resources
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        let index = resources
            .entries
            .iter()
            .position(|(cached_url, _)| cached_url == url)?;
        let entry = resources.entries.remove(index)?;
        let bytes = entry.1.clone();
        resources.entries.push_back(entry);
        Some(bytes)
    }

    /// Stores resource bytes by URL, evicting least recently used entries to fit the budget.
    pub fn set(&self, url: Url, bytes: Bytes) {
        let mut resources = self
            .resources
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        resources.remove(&url);
        if bytes.is_empty() || bytes.len() > self.max_bytes {
            return;
        }

        while resources.bytes > self.max_bytes - bytes.len()
            && let Some((_, evicted)) = resources.entries.pop_front()
        {
            resources.bytes -= evicted.len();
        }
        resources.bytes += bytes.len();
        resources.entries.push_back((url, bytes));
    }

    pub fn invalidate(&self, url: &Url) {
        self.resources
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .remove(url);
    }

    pub fn clear(&self) {
        *self
            .resources
            .lock()
            .unwrap_or_else(|error| error.into_inner()) = CachedResources::default();
    }
}
