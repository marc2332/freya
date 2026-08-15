use freya_core::prelude::*;

/// A document for an [HtmlView](crate::HtmlView), either remote or inline.
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

/// Controls an [HtmlView](crate::HtmlView), lets you navigate programmatically
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
///     .child(HtmlView::new(handle).expanded())
/// # }
/// ```
#[derive(PartialEq, Clone, Copy)]
pub struct HtmlHandle {
    history: State<HtmlHistory>,
}

impl HtmlHandle {
    pub(crate) fn new(initial: HtmlSource) -> Self {
        Self {
            history: State::create(HtmlHistory {
                entries: vec![initial],
                index: 0,
            }),
        }
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
        }
    }

    /// Go forward one entry in the history.
    pub fn forward(&mut self) {
        if self.can_go_forward() {
            self.history.write().index += 1;
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
        let mut history = self.history.write();
        let index = history.index + 1;
        history.entries.truncate(index);
        history.entries.push(source);
        history.index = index;
    }

    /// Current source paired with its history position.
    pub(crate) fn location(&self) -> Option<(usize, HtmlSource)> {
        let history = self.history.read();
        history
            .entries
            .get(history.index)
            .map(|source| (history.index, source.clone()))
    }
}

/// Creates an [HtmlHandle] starting at the source returned by `init`.
pub fn use_html_handle(init: impl FnOnce() -> HtmlSource) -> HtmlHandle {
    use_hook(move || HtmlHandle::new(init()))
}
