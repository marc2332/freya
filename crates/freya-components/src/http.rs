use freya_core::prelude::*;
use reqwest::blocking::Client;

/// Shared blocking HTTP client used to fetch remote assets.
pub(crate) struct Http;

impl Http {
    /// Returns the shared [`Client`], lazily creating it in the root context on first use.
    pub(crate) fn get() -> Client {
        try_consume_root_context::<Client>().unwrap_or_else(|| {
            let client = Client::builder()
                .build()
                .expect("Failed to build the HTTP client.");
            provide_root_context(client.clone());
            client
        })
    }
}
