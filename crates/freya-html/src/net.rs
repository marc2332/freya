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
        NetHandler,
        NetProvider,
        Request,
    },
    shell::ShellProvider,
};
use futures_channel::mpsc::UnboundedSender;
use url::Url;

pub(crate) type FetchRequest = (Url, Box<dyn NetHandler>);

/// Forwards resource requests to the fetching task.
pub(crate) struct HttpNetProvider {
    pub fetch: UnboundedSender<FetchRequest>,
}

impl NetProvider for HttpNetProvider {
    fn fetch(&self, _doc_id: usize, request: Request, handler: Box<dyn NetHandler>) {
        if !matches!(request.url.scheme(), "http" | "https") {
            return;
        }
        let _ = self.fetch.unbounded_send((request.url, handler));
    }
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
