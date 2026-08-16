use std::{
    cell::RefCell,
    rc::Rc,
};

use freya_core::prelude::*;
use reqwest::blocking::Client;

use crate::{
    net::{
        fetch_html,
        http_client,
    },
    state::BlitzState,
};

/// A document for an [HtmlViewer](crate::HtmlViewer), either remote or inline.
#[derive(Clone, PartialEq)]
pub enum HtmlSource {
    Url(String),
    Html(String),
}

impl HtmlSource {
    /// A remote document fetched from `url`.
    pub fn url(url: impl Into<String>) -> Self {
        Self::Url(url.into())
    }

    /// An inline HTML + CSS string.
    pub fn html(html: impl Into<String>) -> Self {
        Self::Html(html.into())
    }
}

struct HtmlHistory {
    entries: Vec<HtmlSource>,
    index: usize,
}

/// Controls an [HtmlViewer](crate::HtmlViewer), lets you navigate programmatically
/// and inspect the browsing history. Link clicks and form submissions inside
/// the document are recorded into it as well.
///
/// ```rust, no_run
/// # use freya::prelude::*;
/// # use freya_html::prelude::*;
/// # fn app() -> impl IntoElement {
/// let mut handle = use_html_handle(|| HtmlSource::url("https://example.com"));
///
/// rect()
///     .child(Button::new().child("Back").on_press(move |_| handle.back()))
///     .child(HtmlViewer::new(handle))
/// # }
/// ```
#[derive(PartialEq, Clone, Copy)]
pub struct HtmlHandle {
    history: State<HtmlHistory>,
    view: State<Option<Rc<RefCell<BlitzState>>>>,
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
        Self {
            history: State::create(HtmlHistory {
                entries: Vec::new(),
                index: 0,
            }),
            view: State::create(None),
        }
    }

    /// A handle with an empty history, backed by global states.
    pub fn create_global_empty() -> Self {
        Self {
            history: State::create_global(HtmlHistory {
                entries: Vec::new(),
                index: 0,
            }),
            view: State::create_global(None),
        }
    }

    /// Attaches the view's document state and loads the current source into it.
    pub(crate) fn attach(&mut self, view: Rc<RefCell<BlitzState>>) {
        *self.view.write() = Some(view);
        self.reload();
    }

    /// Load the URL, discarding any forward history.
    pub fn navigate(&mut self, url: impl Into<String>) {
        self.push(HtmlSource::Url(url.into()));
    }

    /// Load the inline HTML + CSS string, discarding any forward history.
    pub fn load_html(&mut self, html: impl Into<String>) {
        self.push(HtmlSource::Html(html.into()));
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
        if let Some(source) = self.current_source() {
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

    fn current_source(&self) -> Option<HtmlSource> {
        let history = self.history.peek();
        history.entries.get(history.index).cloned()
    }

    /// Loads `source` into the attached view.
    fn load(&self, source: &HtmlSource) {
        let Some(view) = self.view.peek().clone() else {
            return;
        };
        match source {
            HtmlSource::Url(url) => {
                spawn(load_url(view, url.clone(), http_client()));
            }
            HtmlSource::Html(html) => view.borrow_mut().load(html, None),
        }
    }
}

async fn load_url(view: Rc<RefCell<BlitzState>>, url: String, client: Client) {
    let (fetched, url) = blocking::unblock(move || (fetch_html(&client, &url), url)).await;
    match fetched {
        Ok(html) => view.borrow_mut().load(&html, Some(url)),
        Err(err) => tracing::error!("Failed to load {url}: {err}"),
    }
}

/// Creates an [HtmlHandle] starting at the source returned by `init`.
pub fn use_html_handle(init: impl FnOnce() -> HtmlSource) -> HtmlHandle {
    use_hook(move || HtmlHandle::create(init()))
}
