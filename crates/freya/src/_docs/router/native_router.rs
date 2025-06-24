//! # Native Router
//!
//! By default, [Freya Router](freya_router) is pretty simple, so there are certain integrations that are not provided by default, such as as back and forward navigation with the mouse buttons.
//! For things like this exists [`NativeRouter`](freya_router::components::NativeRouter), a thin wrapper component that adds these missing integrations.
//!
//! You simply need to wrap your `Router` content inside the [`NativeRouter`](freya_router::components::NativeRouter) component.
//!
//! Example (based on the example from [router](crate::_docs::router)):
//! ```rust, no_run
//! # use freya::prelude::*;
//! # use freya_router::prelude::*;
//! # use freya_components::Link;
//! # #[allow(non_snake_case)]
//! fn AppSidebar() -> Element {
//!     rsx!(
//!         NativeRouter {
//!             Body {
//!                 Link {
//!                     to: Route::Home,
//!                     label {
//!                         "Home"
//!                     }
//!                 },
//!                 Link {
//!                     to: Route::Other,
//!                     label {
//!                         "Other"
//!                     }
//!                 },
//!                 // Rest of app
//!             }
//!         }
//!     )
//! }
//! # #[rustfmt::skip]
//! # #[derive(Routable, Clone, PartialEq)]
//! # pub enum Route {
//! #     #[layout(AppSidebar)]
//! #         #[route("/")]
//! #         Home,
//! #         #[route("/other")]
//! #         Other,
//! #     #[end_layout]
//! #     #[route("/..route")]
//! #     PageNotFound { }, // Handle 404 routes.
//! # }
//! #
//! # #[component]
//! # fn Home() -> Element {
//! #     rsx!(
//! #         label {
//! #             "Home Page"
//! #         }
//! #     )
//! # }
//! #
//! # #[component]
//! # fn Other() -> Element {
//! #     rsx!(
//! #         label {
//! #             "Other Page"
//! #         }
//! #     )
//! # }
//! #
//! # #[component]
//! # fn PageNotFound() -> Element {
//! #     rsx!(
//! #         label {
//! #             "404"
//! #         }
//! #     )
//! # }
//! ```
