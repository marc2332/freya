//! # Routing
//!
//! Declarative, in-memory routing for Freya applications. Define a route enum with
//! `#[derive(Routable)]`, mount a [Router](components::Router), and render the
//! active page with an [Outlet](components::Outlet).
//!
//! Enable Freya's `router` feature and import the reexport with
//! `use freya::router::*;`. `freya::components::Link` provides declarative
//! navigation. Use [RouterContext](prelude::RouterContext) for navigation from event
//! handlers and other imperative code in a component.
//!
//! # Navigation
//!
//! `Link::new(Route::Settings)` pushes an internal route when pressed. To navigate
//! in an event handler, obtain the context in an event handler and
//! call [`RouterContext::push`](prelude::RouterContext::push) or
//! [`RouterContext::replace`](prelude::RouterContext::replace).
//!
//! ```rust,no_run
//! # use freya::{components::Button, prelude::*, router::*};
//! # #[derive(PartialEq)]
//! # struct Home;
//! # impl Component for Home {
//! #     fn render(&self) -> impl IntoElement { "Home" }
//! # }
//! # #[derive(PartialEq)]
//! # struct Settings;
//! # impl Component for Settings {
//! #     fn render(&self) -> impl IntoElement {
//! let router = RouterContext::get();
//! Button::new()
//!     .on_press(move |_| {
//!         let _ = router.push(Route::Home);
//!     })
//!     .child("Save")
//! #     }
//! # }
//! # #[derive(Routable, Clone, PartialEq)]
//! # enum Route { #[route("/")] Home, #[route("/settings")] Settings }
//! ```
//!
//! Use `freya::components::NativeRouterExt` with `rect().native_router()` to
//! handle native back and forward mouse buttons.
//!
//! See the [basic router example](https://github.com/marc2332/freya/blob/main/examples/feature_router.rs),
//! [nested routes and history example](https://github.com/marc2332/freya/blob/main/examples/feature_router_complex.rs),
//! and [shared multi-window router example](https://github.com/marc2332/freya/blob/main/examples/feature_multi_window_router.rs).
//!
//! # Example
//!
//! A minimal router that switches between two routes.
//!
//! ```rust
//! use freya::{
//!     prelude::*,
//!     router::*,
//! };
//!
//! fn app() -> impl IntoElement {
//!     Router::<Route>::new(|| RouterConfig::default().with_initial_path(Route::Home))
//! }
//!
//! #[derive(PartialEq)]
//! struct Layout;
//! impl Component for Layout {
//!     fn render(&self) -> impl IntoElement {
//!         rect().center().expanded().child(Outlet::<Route>::new())
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
    pub(crate) mod outlet;
    pub use outlet::{
        OutletContext,
        use_outlet_context,
    };
    pub(crate) mod router;
    pub use router::{
        ExternalNavigationFailure,
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
            Outlet,
            Router,
            use_share_router,
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
