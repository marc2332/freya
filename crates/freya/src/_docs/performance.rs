//! # Performance
//!
//! Collection of things to avoid and improve to have a better performance.
//!
//! ### 1. Using use_effect to synchronize state
//! The `use_effect` hook is sometimes missused as a synchronization between states
//!
//! ```rust
//! # use freya::prelude::*;
//! fn app() -> Element {
//!     let mut state = use_signal(|| 1);
//!     let mut double_state = use_signal(|| 1);
//!
//!     use_effect(move || {
//!         // Update double_state whenever `state` changes
//!         double_state.set(state() * 2)
//!     });
//!
//!     rsx!(
//!         label {
//!             onclick: move |_| state += 1,
//!             "{state} * 2 = {double_state}"
//!         }
//!     )
//! }
//! ```
//!
//! This is bad because we are storing a derived value (double_state) in an unnecessary reactive wrapper (signal).
//! The flow would have been:
//! ```ignore
//!                              (initial) -> state: 0 , double_state: 0
//!                   (state gets updated) -> state: 1 , double_state: 0
//! (effect runs and updates double_state) -> state: 1 , double_state: 1
//! ```
//!
//!
//! #### Manual signal derivation
//!
//! We can simply create a temporary variable in which to store the derived value from the signal.
//! Because we are reading `double_state`, whenever `state` changes this component function will reerun, so `double_state` will always be up to date.
//!
//! ```rust
//! # use freya::prelude::*;
//! fn app() -> Element {
//!     let mut state = use_signal(|| 1);
//!     let double_state = state() * 2;
//!
//!     rsx!(
//!         label {
//!             onclick: move |_| state += 1,
//!             "{state} * 2 = {double_state}"
//!         }
//!     )
//! }
//! ```
//!
//! Now, the flow would be:
//! ```ignore
//!                                     (initial) -> state: 0 , double_state: 0
//! (state gets updated and double_state derived) -> state: 1 , double_state: 1
//! ```
//!
//! ### Reactive signal derivation
//!
//! We can also use `use_memo` to memoize derived values. This is very useful for values that are expensive to compute (which isn't the case with simple numeric operation)
//!
//! ```rust
//! # use freya::prelude::*;
//! fn app() -> Element {
//!     let mut state = use_signal(|| 1);
//!     let double_state = use_memo(move || state() * 2);
//!
//!     rsx!(
//!         label {
//!             onclick: move |_| state += 1,
//!             "{state} * 2 = {double_state}"
//!         }
//!     )
//! }
//! ```
//!
//! The flow would be:
//! ```ignore
//!                                                    (initial) -> state: 0 , double_state: 0
//! (state gets updated and double_state memo run synchronously) -> state: 1 , double_state: 1
//! ```
