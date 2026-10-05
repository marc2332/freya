use std::sync::{
    Arc,
    Mutex,
};

use blitz_traits::net::Bytes;
use rustc_hash::FxHashMap;
use url::Url;

#[derive(Clone, Default)]
pub(crate) struct ResourceCache {
    resources: Arc<Mutex<FxHashMap<Url, Bytes>>>,
}

impl ResourceCache {
    pub fn get(&self, url: &Url) -> Option<Bytes> {
        self.resources
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .get(url)
            .cloned()
    }

    pub fn insert(&self, url: Url, bytes: Bytes) {
        self.resources
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .insert(url, bytes);
    }
}
