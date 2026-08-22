use freya_core::prelude::*;
use reqwest::blocking::Client;

/// Shared blocking HTTP client used to fetch remote assets.
pub(crate) struct Http;

impl Http {
    /// Returns the shared [`Client`], lazily creating it in the global contexts on first use.
    pub(crate) fn get() -> Client {
        GlobalContexts::get().get_context_or_insert(|| {
            Client::builder()
                .build()
                .expect("Failed to build the HTTP client.")
        })
    }
}
