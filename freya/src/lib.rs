//! # Freya
//! Build native & cross-platform GUI applications using 🦀 Rust.
//!
//! Powered by [🧬 Dioxus](https://dioxuslabs.com) and [🎨 Skia](https://skia.org/).
//! ```no_run
//! use freya::prelude::*;
//!
//! fn main(){
//!     launch(app);
//! }
//!
//! fn app(cx: Scope) -> Element {
//!    let mut count = use_state(cx, || 0);
//!
//!    render!(
//!        container {
//!            height: "100%",
//!            width: "100%",
//!            background: "rgb(35, 35, 35)",
//!            color: "white",
//!            padding: "12",
//!            onclick: move |_| count += 1,
//!            label { "Click to increase -> {count}" }
//!        }
//!    )
//! }
//!
//! ```

pub mod launch;

pub use dioxus;
pub use freya_components;
pub use freya_elements as dioxus_elements;
pub use freya_hooks;
pub use freya_renderer::WindowConfig;

pub mod prelude {
    pub use crate::launch::*;
    pub use dioxus_core::prelude::*;
    pub use dioxus_core_macro::*;
    pub use dioxus_hooks::*;
    pub use dioxus_hot_reload::{self, hot_reload_init, Config};
    pub use freya_components::*;
    pub use freya_elements as dioxus_elements;
    pub use freya_elements::events_data::*;
    pub use freya_elements::*;
    pub use freya_hooks::*;
    pub use freya_node_state::{bytes_to_data, CustomAttributeValues};
    pub use freya_renderer::WindowConfig;
    pub use tracing;
}
