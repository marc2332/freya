use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
};

use freya_core::prelude::*;
use futures_lite::StreamExt;

use crate::{
    cache::ResourceCache,
    content::HtmlContent,
    net::FetchRequest,
    state::BlitzState,
};

type ResourceInterceptor = Callback<url::Url, Option<Bytes>>;

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
/// let mut handle = use_hook(|| HtmlHandle::create(HtmlSource::url("https://example.com")));
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

impl HtmlHandle {
    /// A handle starting at `initial`.
    pub fn create(initial: HtmlSource) -> Self {
        let mut handle = Self::create_empty();
        handle.push(initial);
        handle
    }

    /// A handle with an empty history.
    pub fn create_empty() -> Self {
        Self::initialize(false)
    }

    /// A handle with an empty history, backed by global states.
    pub fn create_global_empty() -> Self {
        Self::initialize(true)
    }

    fn initialize(global: bool) -> Self {
        let resource_cache = ResourceCache::get();
        let (wake_tx, wake_rx) = async_channel::bounded(1);
        let (nav_tx, mut nav_rx) = futures_channel::mpsc::unbounded::<String>();
        let (fetch_tx, mut fetch_rx) = futures_channel::mpsc::unbounded::<FetchRequest>();
        let view = Rc::new(RefCell::new(BlitzState::new(wake_tx, nav_tx, fetch_tx)));
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

        let interceptor = handle.interceptor;
        let fetch_task = async move {
            while let Some((url, handler)) = fetch_rx.next().await {
                let resolver = interceptor.peek().clone();
                if let Some(bytes) = resolver.and_then(|resolver| resolver.call(url.clone())) {
                    handler.bytes(url.to_string(), bytes);
                    continue;
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

        let navigation_task = async move {
            while let Some(url) = nav_rx.next().await {
                handle.navigate(url);
            }
        };

        if global {
            spawn_global(fetch_task);
            spawn_global(navigation_task);
        } else {
            spawn(fetch_task);
            spawn(navigation_task);
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

    /// Resolves absolute resource URLs locally, returning `None` to use the cache or HTTP.
    pub fn resource_interceptor(
        mut self,
        interceptor: impl FnMut(url::Url) -> Option<Bytes> + 'static,
    ) -> Self {
        self.interceptor.set(Some(Callback::new(interceptor)));
        self
    }

    /// Load the URL, discarding any forward history.
    pub fn navigate(&mut self, url: impl Into<String>) {
        self.push(HtmlSource::Url(url.into()));
    }

    /// Load inline HTML + CSS and its configuration, discarding any forward history.
    pub fn load_html(&mut self, html: impl Into<HtmlContent>) {
        self.push(HtmlSource::Html(html.into()));
    }

    /// Replaces the current document, configuration, and history entry, discarding forward history.
    pub fn replace_html(&mut self, html: impl Into<HtmlContent>) {
        let source = HtmlSource::Html(html.into());
        self.load(&source);
        let mut history = self.history.write();
        let index = history.index;
        history.entries.truncate(index);
        history.entries.push(source);
    }

    /// Go back one entry in the history.
    pub fn back(&mut self) {
        if self.can_go_back() {
            self.history.write().index -= 1;
            self.reload();
        }
    }

    /// Go forward one entry in the history.
    pub fn forward(&mut self) {
        if self.can_go_forward() {
            self.history.write().index += 1;
            self.reload();
        }
    }

    /// Load the current history entry again.
    pub fn reload(&self) {
        let source = {
            let history = self.history.peek();
            history.entries.get(history.index).cloned()
        };
        if let Some(source) = source {
            self.load(&source);
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

    fn push(&mut self, source: HtmlSource) {
        self.load(&source);
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

    fn load(&self, source: &HtmlSource) {
        match source {
            HtmlSource::Url(url) => {
                spawn((*self).load_url(url.clone()));
            }
            HtmlSource::Html(content) => self.view().borrow_mut().load(
                &content.html,
                content.base_url.as_ref().map(ToString::to_string),
            ),
        }
    }

    async fn load_url(self, url: String) {
        let view = self.view();
        let fetched = match url::Url::parse(&url) {
            Ok(parsed) => freya_components::http::fetch(parsed).await,
            Err(err) => Err(err.into()),
        };
        match fetched {
            Ok(bytes) => view
                .borrow_mut()
                .load(&String::from_utf8_lossy(&bytes), Some(url)),
            Err(err) => tracing::error!("Failed to load {url}: {err}"),
        }
    }
}
