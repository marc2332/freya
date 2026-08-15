#[doc(hidden)]
pub mod anyrender;
mod component;
mod element;
mod handle;
mod net;
mod state;

pub use component::HtmlView;
pub use handle::{
    HtmlHandle,
    HtmlSource,
    use_html_handle,
};

pub mod prelude {
    pub use crate::{
        component::HtmlView,
        handle::{
            HtmlHandle,
            HtmlSource,
            use_html_handle,
        },
    };
}
