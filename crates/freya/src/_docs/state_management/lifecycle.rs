//! # Lifecycle
//!
//! Dioxus offers hooks to manage the different lifecycle situations of components.
//!
//! ## Component created
//! You can run certain logic when the component is created (also known as mounted or instanciated) for the first time by using the `use_hook` hook.
//!
//! ```rust
//! # use freya::prelude::*;
//! fn app() -> Element {
//!     use_hook(|| {
//!         println!("Component running for the first time!");
//!     });
//!
//!     Ok(VNode::placeholder())
//! }
//! ```
//!
//! ## Component destroyment
//!
//! Run some logic when the component is being destroyed.
//!
//! ```rust
//! # use freya::prelude::*;
//! fn app() -> Element {
//!     use_drop(|| {
//!         println!("Component is being dropped.");
//!     });
//!
//!     Ok(VNode::placeholder())
//! }
//! ```
//!
//! ## Side effects
//!
//! Run some logic when a signal is changed.
//!
//! ```rust
//! # use freya::prelude::*;
//! fn app() -> Element {
//!     let mut signal = use_signal(|| 1);
//!
//!     use_effect(move || {
//!         // Because we are reading this signal
//!         // now the effect is subscribed to any change
//!         let value = signal();
//!         println!("Value of signal is {value}");
//!     });
//!
//!     Ok(VNode::placeholder())
//! }
//! ```
//!
//! ## Side effects with dependencies
//!
//! Run some logic when some values change.
//!
//! ```rust
//! # use freya::prelude::*;
//! fn Comp(data: u8) -> Element {
//!     let mut signal = use_signal(|| 1);
//!
//!     // Manually specify non-signal values that we might want to react to
//!     // Such as props
//!     use_effect(use_reactive(&data, |data| {
//!         println!("The data is {data}");
//!     }));
//!
//!     // When you need multiple values you can pass a tuple
//!     use_effect(use_reactive(&(signal(), data), |(value, data)| {
//!         println!("Values are {value} and {data}");
//!     }));
//!
//!     Ok(VNode::placeholder())
//! }
//! ```
//!
//! #### You can now learn about [Memoization](crate::_docs::state_management::memoization).
