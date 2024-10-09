//! # Hooks
//! 
//! Hooks are special functions to be used inside of Components that lets you handle different things like the state or lifecycle of your component. They are usually prefixed with `use`, e.g `use_signal`, `use_effect`, `use_memo`, etc.
//! 
//! # Rules of Hooks
//! 
//! Even though hooks appear to be normal functions they are in fact special so you cannot just call them however you want.
//! 
//! ## 1. They cannot be called conditionally
//! 
//! You cannot do the following because hooks need to maintain their order. So, if one component is calling 3 different hooks in one render, and then in another render if just calls 2, it would be breaking this rule.
//! 
//! ❌:
//! ```rust
//! #[component]
//! fn MyComponent(value: bool) -> Element {
//!     let is_enabled = if value {
//!         // This should be moved out of the conditional
//!         use_signal(|| value)
//!     } else {
//!         true
//!     };
//! 
//!     rsx!(...)
//! }
//! ```
//! 
//! ✅:
//! ```rust
//! #[component]
//! fn MyComponent(initial_value: bool) -> Element {
//!     let is_enabled = use_signal(|| initial_value)
//! 
//!     rsx!(...)
//! }
//! ```
//! 
//! ## 2. They can only be called inside of Component functions
//! 
//! You cannot call them inside of event handlers, futures, etc.
//! 
//! ❌:
//! ```rust
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
//! #[component]
//! fn MyComponent() -> Element {
//!     for i in 0..5 {
//!         let state = use_signal(|| i);
//!     }
//! 
//!     rsx!(...)
//! }
//! ```
//! 
//! ✅:
//! ```rust
//! #[component]
//! fn MyComponent() -> Element {
//!     let state = use_signal(|| (0..5).iter().collect::<Vec<_>>());
//! 
//!     rsx!(...)
//! }
//! ```