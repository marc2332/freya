//! # Native Router
//!
//! Even though Freya supports Dioxus Router, there are certain integrations that it does not provide, such as as back and forward navigation with the mouse buttons.
//! For things like this exists `NativeRouter`, a thin wrapper component that adds these missing integrations.
//!
//! You simply need to wrap your `Router` content inside the `NativeRouter` component.
//!
//! Example (based on the example from [router](../router.md)):
//! ```rust
//! #[allow(non_snake_case)]
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
//!                 ...
//!             }
//!         }
//!     )
//! }
//! ```
