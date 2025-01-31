#![doc(
    html_logo_url = "https://freyaui.dev/logo.svg",
    html_favicon_url = "https://freyaui.dev/logo.svg"
)]
//! # Freya
//!
//! **Freya** is a declarative, cross-platform GUI Rust library, powered by 🧬 [Dioxus](https://dioxuslabs.com) and 🎨 [Skia](https://skia.org/).
//!
//! **It does not use any web tech**, check the [Differences with Dioxus](https://book.freyaui.dev/differences_with_dioxus.html).
//!
//! ### Basics
//! - [Introduction](self::_docs::introduction)
//! - [Dioxus Fundamentals](self::_docs::dioxus_fundamentals)
//!     - [UI](self::_docs::ui)
//!     - [Components](self::_docs::components_and_props)
//!     - [Hooks](self::_docs::hooks)
//!     - [State Management](self::_docs::state_management)
//!         - [Signals](self::_docs::state_management::signals)
//!         - [Global Signals](self::_docs::state_management::global_signals)
//!         - [Lifecycle](self::_docs::state_management::lifecycle)
//!         - [Context](self::_docs::state_management::context)
//!         - [Memoization](self::_docs::state_management::memoization)
//!     - [Async Tasks](self::_docs::async_tasks)
//!
//! ### Learn
//! - [Development Setup](self::_docs::development_setup)
//! - [Elements Overview](self::_docs::elements)
//! - [Theming](self::_docs::theming)
//! - [i18n](self::_docs::i18n)
//! - [Accessibility](self::_docs::accessibility)
//! - [Text Editing](self::_docs)
//! - [Animations](self::_docs)
//! - [Router](self::_docs::router)
//!     - [Native Router](self::_docs::router::native_router)
//!     - [Animated transitions](self::_docs::router::animated_transitions)
//! - [Native Menus](self::_docs)
//! - [Third Party State Managemement](self::_docs::third_party_state)
//! - [Unit Testing for Components](freya_testing)
//! - [Devtools](self::_docs::devtools)
//! - [Performance Tips](self::_docs::performance)
//!
//! ### API References
//! - [Elements and attributes](self::elements#structs)
//! - [Events](self::events#functions)
//! - [Built-in Components](self::components)
//! - [Built-in Hooks](self::hooks)
//!
//! ## Features flags
//!
//! - `devtools`: enables a side panel to inspect your App tree, styles and computed layout.
//! - `use_camera`: enables the `use_camera` hook.
//! - `log`: enables internal logs.

/// Freya docs.
#[cfg(doc)]
pub mod _docs;

/// Dioxus library.
pub use dioxus;
pub use dioxus_core;
#[cfg(doc)]
pub use freya_elements::_docs as elements_docs;

/// Launch your app.
pub mod launch;

/// Collection of components.
///
/// Go to [Gallery](freya_components::gallery) to see previews of the components.
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

/// Core APIs.
pub mod core {
    pub use freya_core::*;
}

/// Elements, attributes and events definitions.
pub use freya_elements::elements;
/// Events data.
pub use freya_elements::events;
pub use torin;

pub mod plugins;

/// Useful imports.
pub mod prelude {
    pub use dioxus_core::{
        prelude::*,
        {
            self,
        },
    };
    pub use dioxus_core_macro::*;
    pub use dioxus_hooks::*;
    pub use dioxus_signals::*;
    pub use freya_components::*;
    pub use freya_core::prelude::{
        AccessibilityId,
        PreferredTheme,
    };
    pub use freya_elements::{
        self as dioxus_elements,
        events::*,
    };
    pub use freya_hooks::*;
    pub use freya_node_state::{
        dynamic_bytes,
        static_bytes,
        CustomAttributeValues,
    };
    pub use freya_renderer::*;
    pub use torin::prelude::*;

    pub use crate::{
        launch::*,
        plugins::*,
    };
}
