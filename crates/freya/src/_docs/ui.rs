//! # UI
//!
//! Freya uses a [declarative](https://en.wikipedia.org/wiki/Declarative_programming) model for the UI, in the form a semi-markup language using the `rsx!()` macro from Dioxus, it integrates very well with normal Rust syntax too.
//!
//! For example, this is how a simple component would look like in Freya:
//!
//! ```rust, no_run
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
//! Every time the component function reruns the [`rsx!()`](dioxus_core_macro::rsx!()) will be called again and thus generate a new UI.
//!
//! ### [`rsx!()`](dioxus_core_macro::rsx!())
//!
//! This macro is not a standalone language, it let define the structure, design and dynamism of the UI. It also integrates very well with Rust code.
//!
//! The structure for RSX looks like this:
//!
//! ```rust, no_run
//! # use freya::prelude::*;
//! # { rsx!(
//! // Element, always in lower case
//! rect {
//!     // Attribute for the element `rect`
//!     background: "red",
//!     // Another attribute for the element `rect`
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
//! # )};
//!
//! # #[component]
//! # fn CoolComp(prop: i32) -> Element { Ok(VNode::placeholder()) }
//! ```
//!
//! You can reference variables inside the RSX as well:
//!
//! ```rust, no_run
//! # use freya::prelude::*;
//! let onclick = |_| {
//!     println!("Clicked");
//! };
//!
//! let width = "100%";
//! let name = "World";
//!
//! # {
//! rsx!(
//!     rect {
//!         background: "red",
//!         width,
//!         onclick,
//!         label {
//!             "Hello, {name}!"
//!         }
//!         label {
//!             "{1 + 1} is 2"
//!         }
//!     }
//! )
//! # };
//! ```
//!
//! Or just use if, for-loops, etc... inside of the RSX:
//!
//! ```rust, no_run
//! # use freya::prelude::*;
//! let show_text = false;
//!
//! # {
//! rsx!(
//!     rect {
//!         for i in 0..5 {
//!             label {
//!                 // Looped elements must have an unique ID specified through
//!                 // the `key` attribute so Dioxus is able to properly diff them
//!                 // between multiple reruns
//!                 key: "{i}",
//!                 "Value -> {i}"
//!             }
//!         }
//!         // When this condition is not met the inner rsx will simply not be rendered
//!         if show_text {
//!             label {
//!                 "Hello, World!"
//!             }
//!         }
//!     }
//! )
//! # };
//! ```
