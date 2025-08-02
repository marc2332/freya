#![doc(
    html_logo_url = "https://freyaui.dev/logo.svg",
    html_favicon_url = "https://freyaui.dev/logo.svg"
)]
//! # Freya
//!
//! **Freya** is a declarative, cross-platform GUI Rust library, powered by 🧬 [Dioxus](https://dioxuslabs.com) and 🎨 [Skia](https://skia.org/).
//!
//! **It does not use any web tech**, check the [Differences with Dioxus](https://github.com/marc2332/freya/tree/main?tab=readme-ov-file#differences-with-dioxus).
//!
//! ### Basics
//! - [Introduction](self::_docs::introduction)
//! - [Dioxus Fundamentals](self::_docs::dioxus_fundamentals)
//!     - [UI](self::_docs::ui)
//!     - [Elements Overview](self::_docs::elements)
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
//! - [Theming](self::_docs::theming)
//! - [i18n](self::_docs::i18n)
//! - [Accessibility](self::hooks::use_focus)
//! - [Router](self::_docs::router)
//!     - [Native Router](self::_docs::router::native_router)
//! - [Third Party State Managemement](self::_docs::third_party_state)
//! - [Devtools](self::_docs::devtools)
//! - [Performance Tips](self::_docs::performance)
//!
//! ### Advanced
//! - [Animations](self::hooks::use_animation)
//! - [Text Editing](self::hooks::use_editable)
//! - [Unit Testing of Components](freya_testing)
//!
//! ### API References
//! - [Elements and attributes](self::elements#structs)
//! - [Events](self::events#functions)
//! - [Built-in Components](self::components)
//! - [Built-in Components Gallery](self::components::gallery)
//! - [Built-in Hooks](self::hooks)
//!
//! ## Features flags
//!
//! - `devtools`: enables the devtools server.
//! - `use_camera`: enables the [use_camera](self::hooks::use_camera) hook.
//! - `network-image`: enables the [NetworkImage](self::components::NetworkImage) component.
//! - `custom-tokio-rt`: disables the default Tokio runtime created by Freya.
//! - `performance-overlay`: enables the performance overlay plugin.
//! - `disable-zoom-shortcuts`: disables the default zoom shortcuts.
//! - `disable-animation-shortcuts`: disables the default animation clock shortcuts.

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
    pub use freya_core::*;
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

/// Plot
#[cfg(feature = "plot")]
pub mod plot {
    pub use plotters;
    pub use skia_plotters_backend::*;
}

/// Useful imports.
pub mod prelude {
    pub use dioxus;
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
    pub use freya_core::{
        custom_attributes::{
            dynamic_bytes,
            static_bytes,
            CanvasRunnerContext,
            CustomAttributeValues,
        },
        platform::*,
        platform_state::*,
        types::AccessibilityId,
        window_config::{
            OnCloseResponse,
            WindowConfig,
        },
    };
    pub use freya_elements::{
        self as dioxus_elements,
        events::*,
    };
    pub use freya_hooks::*;
    pub use freya_winit::*;
    pub use torin::prelude::*;

    pub use crate::{
        launch::*,
        plugins::*,
    };
}
