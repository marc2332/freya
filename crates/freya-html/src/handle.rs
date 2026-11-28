use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
};

use freya_core::prelude::*;
use futures_lite::StreamExt;
use url::Url;

use crate::{
    cache::ResourceCache,
    content::HtmlContent,
    net::HtmlRequest,
    state::BlitzState,
};

type ResourceInterceptor = Callback<Url, ResourceResponse>;

/// How to resolve a resource requested by an HTML document.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResourceResponse {
    /// Use the provided content without consulting the cache or network.
    Bytes(Bytes),
    /// Ignore the resource without consulting the cache or network.
    Block,
    /// Fetch normally, using the shared cache for subresources.
    Fetch,
}

/// A document for an [HtmlViewer](crate::HtmlViewer), either remote or inline.
#[derive(Clone, PartialEq)]
pub enum HtmlSource {
    Url(String),
    Html(HtmlContent),
}

impl HtmlSource {
    /// A remote document fetched from `url`.
    pub fn url(url: impl Into<String>) -> Self {
        Self::Url(url.into())
    }

    /// Inline HTML + CSS, optionally configured with an [HtmlContent].
    pub fn html(html: impl Into<HtmlContent>) -> Self {
        Self::Html(html.into())
    }
}

pub struct ElementHit {
    pub tag: String,
    pub attributes: HashMap<String, String>,
}

struct HtmlHistory {
    entries: Vec<HtmlSource>,
    index: usize,
}

/// Controls an [HtmlViewer](crate::HtmlViewer), its navigation and its browsing history.
///
/// ```rust, no_run
/// # use freya::prelude::*;
/// # use freya_html::prelude::*;
/// # fn app() -> impl IntoElement {
/// let mut handle = use_hook(|| {
///     let mut handle = HtmlHandle::create();
///     handle.load(HtmlSource::url("https://example.com"));
///     handle
/// });
///
/// rect()
///     .child(Button::new().child("Back").on_press(move |_| handle.back()))
///     .child(HtmlViewer::new(handle))
/// # }
/// ```
#[derive(PartialEq, Clone, Copy)]
pub struct HtmlHandle {
    history: State<HtmlHistory>,
    view: State<Rc<RefCell<BlitzState>>>,
    pub(crate) wake: State<async_channel::Receiver<()>>,
    interceptor: State<Option<ResourceInterceptor>>,
}

impl Default for HtmlHandle {
    fn default() -> Self {
        Self::create()
    }
}

impl HtmlHandle {
    /// Creates an empty handle, configure it before calling [`Self::load`].
    pub fn create() -> Self {
        Self::initialize(false)
    }

    /// A handle with an empty history, backed by global states.
    pub fn create_global() -> Self {
        Self::initialize(true)
    }

    fn initialize(global: bool) -> Self {
        let resource_cache = ResourceCache::get();
        let (wake_tx, wake_rx) = async_channel::bounded(1);
        let (request_sender, mut requests) = futures_channel::mpsc::unbounded();
        let view = Rc::new(RefCell::new(BlitzState::new(wake_tx, request_sender)));
        let history = HtmlHistory {
            entries: Vec::new(),
            index: 0,
        };
        let mut handle = if global {
            Self {
                history: State::create_global(history),
                view: State::create_global(view),
                wake: State::create_global(wake_rx),
                interceptor: State::create_global(None),
            }
        } else {
            Self {
                history: State::create(history),
                view: State::create(view),
                wake: State::create(wake_rx),
                interceptor: State::create(None),
            }
        };

        let request_task = async move {
            while let Some(request) = requests.next().await {
                let (url, handler) = match request {
                    HtmlRequest::Navigate(url) => {
                        handle.navigate(url.to_string());
                        continue;
                    }
                    HtmlRequest::FetchResource(url, handler) => (url, handler),
                };

                let interceptor = handle.interceptor.peek().clone();
                match interceptor.map_or(ResourceResponse::Fetch, |interceptor| {
                    interceptor.call(url.clone())
                }) {
                    ResourceResponse::Bytes(bytes) => {
                        handler.bytes(url.to_string(), bytes);
                        continue;
                    }
                    ResourceResponse::Block => continue,
                    ResourceResponse::Fetch => {}
                }

                if !matches!(url.scheme(), "http" | "https") {
                    continue;
                }

                if let Some(bytes) = resource_cache.read(&url) {
                    handler.bytes(url.to_string(), bytes);
                    continue;
                }

                let resource_cache = resource_cache.clone();
                let fetch = async move {
                    match freya_components::http::fetch(url.clone()).await {
                        Ok(bytes) => {
                            let resolved_url = url.to_string();
                            resource_cache.set(url, bytes.clone());
                            handler.bytes(resolved_url, bytes);
                        }
                        Err(err) => tracing::warn!("Failed to fetch resource {url}: {err}"),
                    }
                };
                if global {
                    spawn_global(fetch);
                } else {
                    spawn(fetch);
                }
            }
        };

        if global {
            spawn_global(request_task);
        } else {
            spawn(request_task);
        }
        handle
    }

    pub(crate) fn view(&self) -> Rc<RefCell<BlitzState>> {
        self.view.peek().clone()
    }

    /// Replaces local fonts and reloads the current document, an empty collection restores defaults.
    pub fn fonts(self, fonts: impl IntoIterator<Item = Bytes>) -> Self {
        self.view().borrow_mut().set_fonts(fonts);
        self.reload();
        self
    }

    /// Intercepts document and subresource URLs before any cache lookup or fetch.
    pub fn resource_interceptor(
        mut self,
        interceptor: impl FnMut(Url) -> ResourceResponse + 'static,
    ) -> Self {
        self.interceptor.set(Some(Callback::new(interceptor)));
        self
    }

    /// Loads a document and adds it to history, discarding forward history.
    pub fn load(&mut self, source: HtmlSource) {
        if !self.load_source(&source) {
            return;
        }
        let mut history = self.history.write();
        let index = if history.entries.is_empty() {
            0
        } else {
            history.index + 1
        };
        history.entries.truncate(index);
        history.entries.push(source);
        history.index = index;
    }

    /// Load the URL, discarding any forward history.
    pub fn navigate(&mut self, url: impl Into<String>) {
        self.load(HtmlSource::Url(url.into()));
    }

    /// Load inline HTML + CSS and its configuration, discarding any forward history.
    pub fn load_html(&mut self, html: impl Into<HtmlContent>) {
        self.load(HtmlSource::Html(html.into()));
    }

    /// Replaces the current document, configuration, and history entry, discarding forward history.
    pub fn replace_html(&mut self, html: impl Into<HtmlContent>) {
        let source = HtmlSource::Html(html.into());
        self.load_source(&source);
        let mut history = self.history.write();
        let index = history.index;
        history.entries.truncate(index);
        history.entries.push(source);
    }

    /// Go back one entry in the history.
    pub fn back(&mut self) {
        let index = self.history.peek().index;
        if let Some(previous) = index.checked_sub(1) {
            self.go_to_history_entry(previous);
        }
    }

    /// Go forward one entry in the history.
    pub fn forward(&mut self) {
        let index = self.history.peek().index;
        self.go_to_history_entry(index + 1);
    }

    fn go_to_history_entry(&mut self, index: usize) {
        let source = self.history.peek().entries.get(index).cloned();
        if let Some(source) = source
            && self.load_source(&source)
        {
            self.history.write().index = index;
        }
    }

    /// Load the current history entry again.
    pub fn reload(&self) {
        let source = {
            let history = self.history.peek();
            history.entries.get(history.index).cloned()
        };
        if let Some(source) = source {
            self.load_source(&source);
        }
    }

    pub fn can_go_back(&self) -> bool {
        self.history.read().index > 0
    }

    pub fn can_go_forward(&self) -> bool {
        let history = self.history.read();
        history.index + 1 < history.entries.len()
    }

    pub fn elements_at(&self, x: f32, y: f32) -> Vec<ElementHit> {
        self.view().borrow_mut().elements_at(x, y)
    }

    /// The URL currently being displayed, `None` for inline documents.
    pub fn current_url(&self) -> Option<String> {
        let history = self.history.read();
        match history.entries.get(history.index) {
            Some(HtmlSource::Url(url)) => Some(url.clone()),
            _ => None,
        }
    }

    fn load_source(&self, source: &HtmlSource) -> bool {
        match source {
            HtmlSource::Url(url) => {
                let parsed = match Url::parse(url) {
                    Ok(parsed) => parsed,
                    Err(err) => {
                        tracing::error!("Failed to load {url}: {err}");
                        return false;
                    }
                };
                let interceptor = self.interceptor.peek().clone();
                match interceptor.map_or(ResourceResponse::Fetch, |interceptor| {
                    interceptor.call(parsed.clone())
                }) {
                    ResourceResponse::Block => return false,
                    ResourceResponse::Bytes(bytes) => self
                        .view()
                        .borrow_mut()
                        .load(&String::from_utf8_lossy(&bytes), Some(url.clone())),
                    ResourceResponse::Fetch => {
                        spawn((*self).load_url(parsed));
                    }
                }
            }
            HtmlSource::Html(content) => self.view().borrow_mut().load(
                &content.html,
                content.base_url.as_ref().map(ToString::to_string),
            ),
        }
        true
    }

    async fn load_url(self, url: Url) {
        let base_url = url.to_string();
        match freya_components::http::fetch(url).await {
            Ok(bytes) => self
                .view()
                .borrow_mut()
                .load(&String::from_utf8_lossy(&bytes), Some(base_url)),
            Err(err) => tracing::error!("Failed to load {base_url}: {err}"),
        }
    }
}
