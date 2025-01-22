//! # Context
//!
//! Dioxus offers a way to pass data from parent components to its descendents in a way to avoid [**Prop Drilling**](#prop-drilling).
//!
//! ## Prop Drilling
//!
//! **Prop drilling** is when you want to pass a certain data from one parent component to some nested component, and you start to declare the same prop in each one of the components in between the parent and the target component. This causes a huge unnecessary boilerplate that can be used by using the Context API.
//!
//! ```rust
//! # use freya::prelude::*;
//! // Parent component
//! #[component]
//! fn CompA() -> Element {
//!     rsx!(
//!         CompB {
//!             value: 2
//!         }
//!     )
//! }
//!
//! // This component needs the value just so it can pass it to the next component
//! #[component]
//! fn CompB(value: usize) -> Element {
//!     rsx!(
//!         CompC {
//!             value
//!         }
//!     )
//! }
//!
//! // Same as CompB
//! #[component]
//! fn CompC(value: usize) -> Element {
//!     rsx!(
//!         CompD {
//!             value
//!         }
//!     )
//! }
//!
//! // Finally ! Our target component
//! #[component]
//! fn CompD(value: usize) -> Element {
//!     rsx!(
//!         label {
//!             "{value}"
//!         }
//!     )
//! }
//! ```
//!
//! ## Context
//!
//! You can initialize a context that will be identified by its type and be allowed to be accessed from any desdendent from where you intialized it.
//!
//! ```rust
//! # use freya::prelude::*;
//! // Parent component
//! #[component]
//! fn CompA() -> Element {
//!     // Initialize the context with `1` usize as value
//!     // Any component desdendant of `CompA` will be allowed to access this component
//!     use_context_provider(|| 1);
//!
//!     rsx!(
//!         CompB { }
//!     )
//! }
//!
//! #[component]
//! fn CompB() -> Element {
//!     rsx!(
//!         CompC { }
//!     )
//! }
//!
//! #[component]
//! fn CompC() -> Element {
//!     rsx!(
//!         CompD { }
//!     )
//! }
//!
//! // Our target component
//! #[component]
//! fn CompD() -> Element {
//!     // Retrieve our context as we know its type
//!     let value = use_context::<usize>();
//!
//!     rsx!(
//!         label {
//!             "{value}"
//!         }
//!     )
//! }
//! ```
//!
//! ### Avoid having context with same type
//!
//! Because Context are identified by their type, you cannot do the following:
//!
//! ```rust
//! # use freya::prelude::*;
//! // Parent component
//! #[component]
//! fn CompA() -> Element {
//!     use_context_provider(|| 1);
//!     use_context_provider(|| 2);
//!     use_context_provider(|| 3);
//!
//!     // All these 3 contexts share the same type, `usize`, so each context will replace any other context defined previously, which means that only the third context will actually be accessible
//!
//!     rsx!(
//!         CompB { }
//!     )
//! }
//!
//! # #[component]
//! # fn CompB() -> Element {
//! #    Ok(VNode::placeholder())
//! # }
//! ```
//!
//! If you really need to the tree contexts split you can wrap them in different types so each one gets an unique type instead of just `usize`.
//!
//! ```rust
//! # use freya::prelude::*;
//!
//! #[derive(Clone)]
//! struct ValueA(pub usize);
//!
//! #[derive(Clone)]
//! struct ValueB(pub usize);
//!
//! #[derive(Clone)]
//! struct ValueC(pub usize);
//!
//! // Parent component
//! #[component]
//! fn CompA() -> Element {
//!     use_context_provider(|| ValueA(1));
//!     use_context_provider(|| ValueB(2));
//!     use_context_provider(|| ValueC(3));
//!
//!     // All these 3 contexts share the same type, `usize`, so each context will replace any other context defined previously, which means that only the third context will actually be accessible
//!
//!     rsx!(
//!         CompB { }
//!     )
//! }
//!
//! #[component]
//! fn CompB() -> Element {
//!     let value_a = use_context::<ValueA>();
//!     let value_b = use_context::<ValueB>();
//!     let value_c = use_context::<ValueC>();
//!     rsx!(
//!         label {
//!             "{value_a.0} {value_b.0} {value_c.0}"
//!         }
//!     )
//! }
//! ```
