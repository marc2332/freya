//! Routing
//!
//! High-level routing utilities for Freya applications. This crate provides
//! components like [outlet](self::components::outlet) and [router](self::components::router), hooks such as [use_route](self::hooks::use_route), and the
//! `Navigator` context to programmatically interact with navigation state.
//!
//! # Example
//!
//! A minimal router that switches between two routes. See `examples/feature_router.rs`
//! for a runnable demo.
//!
//! ```rust
//! use freya::{
//!     prelude::*,
//!     router::prelude::*,
//! };
//!
//! fn app() -> impl IntoElement {
//!     router::<Route>(|| RouterConfig::default().with_initial_path(Route::Home))
//! }
//!
//! #[derive(PartialEq)]
//! struct Layout;
//! impl Component for Layout {
//!     fn render(&self) -> impl IntoElement {
//!         rect().center().expanded().child(outlet::<Route>())
//!     }
//! }
//!
//! #[derive(PartialEq)]
//! struct Home;
//! impl Component for Home {
//!     fn render(&self) -> impl IntoElement {
//!         Link::new(Route::Settings).child("Go Settings")
//!     }
//! }
//!
//! #[derive(PartialEq)]
//! struct Settings;
//! impl Component for Settings {
//!     fn render(&self) -> impl IntoElement {
//!         Link::new(Route::Home).child("Go Home")
//!     }
//! }
//!
//! #[derive(Routable, Clone, PartialEq)]
//! #[rustfmt::skip]
//! pub enum Route {
//!     #[layout(Layout)]
//!         #[route("/")]
//!         Home,
//!         #[route("/settings")]
//!         Settings,
//! }
//! ```
// cannot use forbid, because props derive macro generates #[allow(missing_docs)]
#![allow(non_snake_case)]

mod memory;

pub mod navigation;
pub mod routable;

/// Components interacting with the router.
pub mod components {
    mod outlet;
    pub use outlet::*;

    mod router;
    pub use router::*;

    #[doc(hidden)]
    pub mod child_router;
}

mod contexts {
    pub(crate) mod navigator;
    pub(crate) mod outlet;
    pub use outlet::{
        OutletContext,
        use_outlet_context,
    };
    pub(crate) mod router;
    pub use navigator::*;
    pub(crate) use router::*;
    pub use router::{
        GenericRouterContext,
        ParseRouteError,
        RouterContext,
    };
}

mod router_cfg;

/// Hooks for interacting with the router in components.
pub mod hooks {
    mod use_route;
    pub use use_route::*;
}

/// A collection of useful items most applications might need.
pub mod prelude {
    pub use freya_router_macro::Routable;

    pub use crate::{
        components::{
            outlet,
            router,
        },
        contexts::*,
        hooks::*,
        memory::MemoryHistory,
        navigation::*,
        routable::*,
        router_cfg::RouterConfig,
    };
}

mod utils {
    pub(crate) mod use_router_internal;
}

#[doc(hidden)]
pub mod exports {
    pub use urlencoding;
}
