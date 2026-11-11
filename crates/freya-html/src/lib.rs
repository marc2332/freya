//! Render inline or remote HTML and CSS in a Freya component using Blitz.
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! use freya_html::prelude::*;
//!
//! # fn app() -> impl IntoElement {
//! let handle = use_hook(|| {
//!     let mut handle = HtmlHandle::create();
//!     handle.load(HtmlSource::html("<h1>Hello!</h1>"));
//!     handle
//! });
//! HtmlViewer::new(handle)
//! # }
//! ```
//!
//! Set a base URL for relative links with [`HtmlContent::with_base_url`].
//!
//! Use [`HtmlSource::url`] for remote pages and [`HtmlHandle::fonts`] for local fonts.
//! On web, provide a font for inline documents or load one through CSS `@font-face`.
//!
//! Resolve embedded CSS and other subresources with [`HtmlHandle::resource_interceptor`].
//!
//! ```rust,no_run
//! # use freya_core::prelude::*;
//! # use freya_html::prelude::*;
//! # fn app() -> impl IntoElement {
//! let handle = use_hook(|| {
//!     let mut handle = HtmlHandle::create().resource_interceptor(|url| match url.path() {
//!         "/style.css" => Some(Bytes::from_static(b"body { margin: 0 }")),
//!         _ => None,
//!     });
//!     handle.load(HtmlSource::html(
//!         r#"<link rel="stylesheet" href="https://freya.example/style.css">"#,
//!     ));
//!     handle
//! });
//! HtmlViewer::new(handle)
//! # }
//! ```
//!
//! For HTTP requests outside the viewer,
//! enable Freya's `remote-asset` feature and use [`freya_components::http::fetch`]:
//!
//! ```rust,no_run
//! # async fn load() -> Result<(), Box<dyn std::error::Error>> {
//! let url = freya_components::Url::parse("https://example.com")?;
//! let bytes = freya_components::http::fetch(url).await?;
//! # Ok(())
//! # }
//! ```

#[doc(hidden)]
pub mod anyrender;
mod cache;
mod component;
mod content;
mod element;
mod fonts;
mod handle;
mod net;
mod state;

pub use cache::ResourceCache;
pub use component::HtmlViewer;
pub use content::HtmlContent;
pub use handle::{
    ElementHit,
    HtmlHandle,
    HtmlSource,
};

pub mod prelude {
    pub use crate::{
        cache::ResourceCache,
        component::HtmlViewer,
        content::HtmlContent,
        handle::{
            ElementHit,
            HtmlHandle,
            HtmlSource,
        },
    };
}
