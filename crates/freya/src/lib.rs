#![doc(
    html_logo_url = "https://freyaui.dev/logo.svg",
    html_favicon_url = "https://freyaui.dev/logo.svg"
)]
//! # Freya
//!
//! Build native & cross-platform GUI applications using 🦀 Rust.
//!
//! Powered by [🧬 Dioxus](https://dioxuslabs.com) and [🎨 Skia](https://skia.org/).
//!
//! - [Elements API reference](freya_elements::elements#structs)
//! - [Events API reference](freya_elements::elements#functions)
//! - [Elements guides](freya_elements::_docs)
//! - [Components](freya_components)
//! - [Hooks](freya_hooks)
//! - [Theming](self::_docs::theming)
//! - [Hot reload](self::_docs::hot_reload)
//! - [Testing](freya_testing)
//! - [Animating](freya_hooks::use_animation)
//! - [Devtools](self::_docs::devtools)
//!
//! ```rust,no_run
//! use freya::prelude::*;
//!
//! fn main(){
//!     launch(app);
//! }
//!
//! fn app() -> Element {
//!    let mut count = use_signal(|| 0);
//!
//!    rsx!(
//!        rect {
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
//! ```
//!
//! ## Features flags
//!
//! - `devtools`: enables a side panel to inspect your App tree, styles and computed layout.
//! - `use_camera`: enables the `use_camera` hook.
//! - `log`: enables internal logs.
//!

/// Freya docs.
#[cfg(doc)]
pub mod _docs;

#[cfg(doc)]
pub use freya_elements::_docs as elements_docs;

/// Dioxus library.
pub use dioxus;

pub use dioxus_core;

/// Launch your app.
pub mod launch;

/// Collection of basic components.
pub mod components {
    pub use freya_components::*;
}

/// Useful utilities.
pub mod hooks {
    pub use freya_hooks::*;
}

/// Common data structures and utils.
pub mod common {
    pub use freya_common::*;
}

/// Events data.
pub use freya_elements::events;

/// Elements, attributes and events definitions.
pub use freya_elements::elements;

/// Hot reload configuration.
pub mod hotreload {
    pub use freya_elements::elements::FreyaCtx;
}

pub use torin;

pub mod plugins;

/// Useful imports.
pub mod prelude {
    pub use dioxus_core;
    pub use dioxus_core::prelude::*;
    pub use dioxus_core_macro::*;
    pub use dioxus_hooks::*;
    pub use dioxus_hot_reload::{self, hot_reload_init, Config};
    pub use dioxus_signals::*;

    pub use crate::launch::*;
    pub use crate::plugins::*;
    pub use freya_components::*;
    pub use freya_elements::elements as dioxus_elements;
    pub use freya_elements::events::*;
    pub use freya_hooks::*;
    pub use freya_node_state::{dynamic_bytes, static_bytes, CustomAttributeValues};
    pub use freya_renderer::*;
    pub use torin::prelude::*;
}
