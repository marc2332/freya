use std::borrow::Cow;

use url::{
    ParseError,
    Url,
};

/// Inline HTML and its optional base URL for resolving relative resources and links.
/// Static HTML is borrowed, while dynamically generated HTML is owned.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HtmlContent {
    pub(crate) html: Cow<'static, str>,
    pub(crate) base_url: Option<Url>,
}

impl HtmlContent {
    pub fn new(html: impl Into<Cow<'static, str>>) -> Self {
        Self {
            html: html.into(),
            base_url: None,
        }
    }

    /// Sets an absolute base URL, rejecting malformed URLs and opaque URLs such as `data:`.
    pub fn with_base_url(mut self, base_url: impl AsRef<str>) -> Result<Self, ParseError> {
        let base_url = Url::parse(base_url.as_ref())?;
        if base_url.cannot_be_a_base() {
            return Err(ParseError::RelativeUrlWithCannotBeABaseBase);
        }

        self.base_url = Some(base_url);
        Ok(self)
    }
}

impl<T: Into<Cow<'static, str>>> From<T> for HtmlContent {
    fn from(html: T) -> Self {
        Self::new(html)
    }
}
