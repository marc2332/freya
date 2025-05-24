//! # Hooks
//!
//! Hooks are special functions that must be called inside of Components.
//!
//! These let you manage things like state or lifecycle of your component.
//!
//! They are usually prefixed with `use`, e.g [`use_animation`](crate::hooks::use_animation), [`use_signal`](dioxus::hooks::use_signal), [`use_effect`](dioxus::hooks::use_signal), [`use_memo`]((dioxus::hooks::use_memo)), etc.
//!
//! # Rules of Hooks
//!
//! Even though hooks appear to be normal functions they are in fact special so you cannot just call them however you want.
//!
//! ## 1. They cannot be called conditionally
//!
//! You cannot do the following because hooks need to maintain their order between multiple renders.
//!
//! So, if the same component is calling 3 different hooks in the first render, and then in the next render if just calls 2, it would be breaking this rule.
//!
//! ❌:
//! ```rust
//! # use freya::prelude::*;
//! #[component]
//! fn MyComponent(value: bool) -> Element {
//!     let is_enabled = if value {
//!         // This should be moved out of the conditional
//!         let state = use_signal(|| value);
//!
//!         state()
//!     } else {
//!         true
//!     };
//!
//!     Ok(VNode::placeholder())
//! }
//! ```
//!
//! ✅:
//! ```rust
//! # use freya::prelude::*;
//! #[component]
//! fn MyComponent(initial_value: bool) -> Element {
//!     let is_enabled = use_signal(move || initial_value);
//!
//!     rsx!( label { "{is_enabled}" } )
//! }
//! ```
//!
//! Or even better, move the state up ✅:
//! ```rust
//! # use freya::prelude::*;
//! #[component]
//! fn MyComponent(value: bool) -> Element {
//!     rsx!( label { "{is_enabled}" } )
//! }
//! ```
//!
//! ## 2. They can only be called inside of Component functions
//!
//! You cannot call them inside of event handlers, futures, etc.
//!
//! ❌:
//! ```rust
//! # use freya::prelude::*;
//! #[component]
//! fn MyComponent() -> Element {
//!     let onclick = |_| {
//!          let state = use_signal(|| false);
//!     };
//!
//!     rsx!(
//!         label {
//!             onclick,
//!             "Hello, World!"
//!         }
//!     )
//! }
//! ```
//!
//! ✅:
//! ```rust
//! # use freya::prelude::*;
//! #[component]
//! fn MyComponent() -> Element {
//!     let mut state = use_signal(|| false);
//!
//!     let onclick = move |_| {
//!          state.set(true);
//!     };
//!
//!     rsx!(
//!         label {
//!             onclick,
//!             "Hello, World!"
//!         }
//!     )
//! }
//! ```
//!
//! ## 3. They cannot be called in loops
//!
//! Hooks cannot be called in loops as the numbers of iterations might change between renders.
//!
//! ❌:
//! ```rust
//! # use freya::prelude::*;
//! #[component]
//! fn MyComponent() -> Element {
//!     for i in 0..5 {
//!         let state = use_signal(|| i);
//!     }
//!
//!     rsx!( p { "hi" } )
//! }
//! ```
//!
//! ✅:
//! ```rust
//! # use freya::prelude::*;
//! #[component]
//! fn MyComponent() -> Element {
//!     let state = use_signal(|| (0..5).into_iter().collect::<Vec<_>>());
//!
//!     rsx!( p { "hi" } )
//! }
//! ```
//!
//! #### You can learn more in depth now about [State Management](crate::_docs::state_management).
