//! # Router
//!
//! Freya supports the official [Dioxus Router](https://docs.rs/dioxus-router/latest/dioxus_router/), which means you can declare different pages for your app. The only difference is that you will need to use Freya's custom `Link` component.
//!
//! ### Example
//!
//! ```rust
//! # use freya::prelude::*;
//! # use dioxus_router::prelude::{
//! #    Outlet,
//! #    Routable,
//! #    Router,
//! # };
//! fn app() -> Element {
//!     /// We place the router renderer in the root component
//!     rsx!(Router::<Route> {})
//! }
//!
//! // Declare your Routes tree as an enum
//! // Every route must have a component with the same name
//! // So for example, `Home` needs to have a `fn Home(...` component
//! // the `Routable` macro will pick it up automatically
//! // so it must be in the scope.
//! #[derive(Routable, Clone, PartialEq)]
//! #[rustfmt::skip]
//! pub enum Route {
//!     #[layout(AppSidebar)]
//!         #[route("/")]
//!         Home,
//!         #[route("/other")]
//!         Other,
//!     #[end_layout]
//!     #[route("/..route")]
//!     PageNotFound { }, // Handle 404 routes.
//! }
//!
//! // This component is used as container for the router (as it was marked with `#[layout(AppSidebar)]`), which means
//! // That we can render something here that will be rendered no matter what route you are in
//! // Useful for navigation bars.
//! fn AppSidebar() -> Element {
//!     rsx!(
//!         Body {
//!             Link {
//!                 // Specify to what destination you want this Link you point when clicking.
//!                 to: Route::Home,
//!                 label {
//!                     "Home"
//!                 }
//!             },
//!             Link {
//!                 to: Route::Other,
//!                 label {
//!                     "Other"
//!                 }
//!             },
//!             rect {
//!                 main_align: "center",
//!                 cross_align: "center",
//!                 width: "100%",
//!                 height: "100%",
//!                 // This is the place where the routes content will actually be showed.
//!                 Outlet::<Route> {  }
//!             }
//!         }
//!     )
//! }
//!
//! #[component]
//! fn Home() -> Element {
//!     rsx!(
//!         label {
//!             "Home Page"
//!         }
//!     )
//! }
//!
//! #[component]
//! fn Other() -> Element {
//!     rsx!(
//!         label {
//!             "Other Page"
//!         }
//!     )
//! }
//!
//! #[component]
//! fn PageNotFound() -> Element {
//!     rsx!(
//!         label {
//!             "404"
//!         }
//!     )
//! }
//! ```

pub mod animated_transitions;
pub mod native_router;
