//!  UI
//!
//! Freya uses a declarive model for the UI, which means that you do not use imperative APIs but instead you write in a semi-markup language (integrated with Rust) by using the `rsx!()` macro from Dioxus.
//!
//! For example, this is how a simple component would look like in Freya:
//!
//! ```rust
//! # use freya::prelude::*;
//! fn app() -> Element {
//!     rsx!(
//!         rect {
//!             background: "red",
//!             width: "100%",
//!             onclick: |_| println!("Clicked!"),
//!             label {
//!                 "Hello, World!"
//!             }
//!         }
//!     )
//! }
//! ```
//!
//! Notice that the `app` component is returning an [`Element`](dioxus_core::Element) created by the [`rsx!()`](dioxus_core_macro::rsx!()) macro. So, in other words, the [`Element`](dioxus_core::Element) contains the UI of that component.
//! Every time the component reruns the [`rsx!()`](dioxus_core_macro::rsx!()) will be called again and thus generate a new UI.
//!
//! ### [`rsx!()`](dioxus_core_macro::rsx!())
//!
//! This macro is not a standalone-language or anything like that. It is simply a macro to easily declare how we want the UI to look like. You can still use normal Rust code inside.
//!
//! The structure for RSX looks like this:
//!
//! ```rust
//! # use freya::prelude::*;
//! # rsx!(
//! // Element, always in lower case
//! rect {
//!     // Attribute for the element `rect`
//!     background: "red",
//!     // Attribute for the element `rect`
//!     width: "100%",
//!     // Event handler for the element `rect`, can be a function or a closure
//!     onclick: |_| println!("Clicked!"),
//!     // Element child of `rect`
//!     label {
//!         // Text Element for the element `label`
//!         "Hello, World!"
//!     }
//!     // Component child of `rect`, always in PascalCase
//!     CoolComp {
//!         // Prop for the component `CoolComp`
//!         prop: 123
//!     }
//! }
//! #)
//! ```
//!
//! You can reference variables inside the RSX as well:
//!
//! ```rust
//! # use freya::prelude::*;
//! let onclick = |_| {
//!     println!("Clicked");
//! };
//!
//! let width = "100%";
//! let name = "World";
//!
//! rsx!(
//!     rect {
//!         background: "red",
//!         width,
//!         onclick,
//!         label {
//!             "Hello, {name}!"
//!         }
//!         label {
//!         "{1 + 1} is 2"
//!         }
//!     }
//! ) #;
//! ```
//!
//! Or just use if, for-loops, etc.. Inside of the RSX:
//!
//! ```rust
//! # use freya::prelude::*;
//! let show_text = false;
//!
//! rsx!(
//!     rect {
//!         for i in 0..5 {
//!             label {
//!             // Looped elements must have an unique ID specified through
//!                 // the `key` attribute so Dioxus is able to identify them
//!                 key: "{i}",
//!                 "Value -> {i}"
//!             }
//!         }
//!         // When this condition is not met the inner element will
//!         // simply not be rendered
//!         if show_text {
//!             label {
//!                 "Hello, World!"
//!             }
//!         }
//!     }
//! ) #;
//! ```
