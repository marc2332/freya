//! Render inline or remote HTML and CSS in a Freya component using Blitz.
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! use freya_html::prelude::*;
//!
//! # fn app() -> impl IntoElement {
//! let handle = use_html(|| HtmlSource::html("<h1>Hello!</h1>"));
//! HtmlViewer::new(handle).expanded()
//! # }
//! ```
//!
//! Use [`HtmlSource::url`] to load a remote page. For HTTP requests outside the viewer,
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
mod component;
mod element;
mod handle;
mod net;
mod state;

pub use component::HtmlViewer;
pub use handle::{
    HtmlHandle,
    HtmlSource,
    use_html,
};

pub mod prelude {
    pub use crate::{
        component::HtmlViewer,
        handle::{
            HtmlHandle,
            HtmlSource,
            use_html,
        },
    };
}
