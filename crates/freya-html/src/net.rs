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

pub(crate) enum HtmlRequest {
    Navigate(Url),
    FetchResource(Url, Box<dyn NetHandler>),
}

pub(crate) struct HtmlRequestProvider(pub UnboundedSender<HtmlRequest>);

impl NetProvider for HtmlRequestProvider {
    fn fetch(&self, _doc_id: usize, request: Request, handler: Box<dyn NetHandler>) {
        let _ = self
            .0
            .unbounded_send(HtmlRequest::FetchResource(request.url, handler));
    }
}

impl NavigationProvider for HtmlRequestProvider {
    fn navigate_to(&self, options: NavigationOptions) {
        let _ = self.0.unbounded_send(HtmlRequest::Navigate(options.url));
    }
}

/// Signals Freya to re-render when the document requests a redraw.
pub(crate) struct FreyaShellProvider {
    pub redraw: Arc<AtomicBool>,
    pub wake: async_channel::Sender<()>,
}

impl ShellProvider for FreyaShellProvider {
    fn request_redraw(&self) {
        self.redraw.store(true, Ordering::Relaxed);
        let _ = self.wake.try_send(());
    }
}
