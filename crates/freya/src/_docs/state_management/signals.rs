//! # Signals
//!
//! Signals are a state management solution built-in into Dioxus. They allow us to store values so that components can read and subscribe to any change done to the stored value. Signals can even be read and mutated from multiple components.
//!
//! They are usually created by using the [`use_signal`](dioxus::prelude::use_signal) hook.
//!
//! ### Example
//!
//! ```rust
//! # use freya::prelude::*;
//! fn app() -> Element {
//!     // The closure passed to `use_signal` will be called only
//!     // the first time this component renders,
//!     // it will return the initial value for the Signal.
//!     // This closure is to prevent having to create the initial value
//!     // every time the component runs again, as it is only needed the first time.
//!     let mut count = use_signal(|| 0);
//!
//!     let onclick = move |_| {
//!         count += 1; // Shorthand for count.write() += 1;
//!         // The moment the signal is mutated, it will notify
//!         // all the components that have a read subscription
//!         // to this signal (in this case, only `app`)
//!         // that there has been a change.
//!         // When that happens these components will render again
//!         // so that an updated UI is produced and ultimately presented to the user.
//!     };
//!
//!     rsx!(
//!         label {
//!             onclick,
//!             "{count}"
//!             // By embedding the `count` signal in here, we are effectively
//!             // creating a read subscription. It is the same as if was doing`"{count.read()}"`.
//!         }
//!     )
//! }
//! ```
