//! # Memoization
//!
//! You can memoize values by using the `use_memo` hook. This can be useful to have reactive derived across components or to cache expensive value to compute.
//!
//! ```rust
//! fn app() -> Element {
//!     let mut state = use_signal(|| 1);
//!     // `use_memo` returns a `ReadOnlySignal`, as the name says it is a Signal
//!     // that you can read and subscribe to but you cannot mutate
//!     // as its value can only be changed when the memo runs
//!     let double_state = use_memo(move || {
//!         // Just like `use_effect`, whenever a signal that is read in here is changed, the memo will rerun.
//!         state() * 2
//!     });
//!
//!     rsx!(
//!         label {
//!             onclick: move |_| state += 1,
//!             "{double_state}"
//!         }
//!     )
//! }
//! ```
