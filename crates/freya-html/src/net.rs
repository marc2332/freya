use std::sync::{
    Arc,
    atomic::{
        AtomicBool,
        Ordering,
    },
};

use blitz_traits::{
    navigation::{
        NavigationOptions,
        NavigationProvider,
    },
    net::{
        Bytes,
        NetHandler,
        NetProvider,
        Request,
    },
    shell::ShellProvider,
};
use freya_core::prelude::{
    provide_root_context,
    try_consume_root_context,
};
use futures_channel::mpsc::UnboundedSender;
use reqwest::blocking::{
    Client,
    Response,
};

type FetchError = Box<dyn std::error::Error + Send + Sync>;

const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64; rv:60.0) Gecko/20100101 Firefox/81.0";

/// App-wide blocking HTTP client.
pub(crate) fn http_client() -> Client {
    try_consume_root_context::<Client>().unwrap_or_else(|| {
        let client = Client::builder()
            .build()
            .expect("Failed to build the HTTP client.");
        provide_root_context(client.clone());
        client
    })
}

pub(crate) struct HttpNetProvider {
    pub client: Client,
}

impl NetProvider for HttpNetProvider {
    fn fetch(&self, _doc_id: usize, request: Request, handler: Box<dyn NetHandler>) {
        let url = request.url;
        if !matches!(url.scheme(), "http" | "https") {
            return;
        }

        let client = self.client.clone();
        blocking::unblock(move || match fetch_bytes(&client, url.as_str()) {
            Ok(bytes) => handler.bytes(url.to_string(), bytes),
            Err(err) => tracing::warn!("Failed to fetch resource {url}: {err}"),
        })
        .detach();
    }
}

fn fetch(client: &Client, url: &str, accept: &str) -> Result<Response, FetchError> {
    Ok(client
        .get(url)
        .header("User-Agent", USER_AGENT)
        .header("Accept", accept)
        .send()?
        .error_for_status()?)
}

fn fetch_bytes(client: &Client, url: &str) -> Result<Bytes, FetchError> {
    Ok(fetch(client, url, "*/*")?.bytes()?)
}

pub(crate) fn fetch_html(client: &Client, url: &str) -> Result<String, FetchError> {
    Ok(fetch(client, url, "text/html,application/xhtml+xml,*/*")?.text()?)
}

/// Signals Freya to re-render when the document requests a redraw.
pub(crate) struct FreyaShellProvider {
    pub redraw: Arc<AtomicBool>,
    pub wake: UnboundedSender<()>,
}

impl ShellProvider for FreyaShellProvider {
    fn request_redraw(&self) {
        self.redraw.store(true, Ordering::Relaxed);
        let _ = self.wake.unbounded_send(());
    }
}

/// Forwards link clicks and form submissions.
pub(crate) struct FreyaNavigationProvider {
    pub navigate: UnboundedSender<String>,
}

impl NavigationProvider for FreyaNavigationProvider {
    fn navigate_to(&self, options: NavigationOptions) {
        let _ = self.navigate.unbounded_send(options.url.to_string());
    }
}
