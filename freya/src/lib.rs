//! # Freya
//! A GUI library for Rust powered by [Skia](https://skia.org/) and [Dioxus](https://dioxuslabs.com).
//! ```no_run
//! use freya::prelude::*;
//!
//! fn main(){
//!     launch(app);
//! }
//!
//! fn app(cx: Scope) -> Element {
//!    let mut count = use_state(&cx, || 0);
//!
//!    render!(
//!        container {
//!            height: "100%",
//!            width: "100%",
//!            background: "rgb(35, 35, 35)",
//!            color: "white",
//!            padding: "25",
//!            onclick: move |_| count += 1,
//!            label { "Click to increase -> {count}" }
//!        }
//!    )
//! }
//!
//! ```
//!

#[cfg(feature = "devtools")]
mod devtools;

pub mod launch;

pub use dioxus;
pub use freya_components;
pub use freya_elements as dioxus_elements;
pub use freya_hooks;
pub use freya_renderer::WindowConfig;

pub mod prelude {
    pub use crate::launch::*;
    pub use dioxus::prelude::*;
    pub use freya_components::*;
    pub use freya_elements as dioxus_elements;
    pub use freya_elements::*;
    pub use freya_hooks::*;
    pub use freya_renderer::WindowConfig;
}
