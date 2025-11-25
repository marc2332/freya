//! # Hooks
//!
//! Hooks are special functions that let you tap into Freya's reactivity and lifecycle system.
//! They can **only** be called at the top level of your component's `render` method (not inside event handlers, loops, or conditionals).
//!
//! Hooks are always prefixed with `use_`, for example: [`use_animation`](crate::animation::use_animation), [`use_state`](crate::prelude::use_state), etc.
//!
//! ## Rules of Hooks
//!
//! Hooks are not ordinary functions. To ensure correct behavior, follow these rules:
//!
//! ### 1. Only call hooks at the top level of your `render` method
//!
//! Hooks must always be called in the same order on every render.
//!
//! ❌ **Incorrect:**
//! ```rust
//! # use freya::prelude::*;
//! #[derive(PartialEq)]
//! struct MyComponent(bool);
//! impl Render for MyComponent {
//!     fn render(&self) -> impl IntoElement {
//!         if self.0 {
//!             let state = use_state(|| self.0);
//!             // ...
//!         }
//!         rect()
//!     }
//! }
//! ```
//!
//! ✅ **Correct:**
//! ```rust
//! # use freya::prelude::*;
//! #[derive(PartialEq)]
//! struct MyComponent(bool);
//! impl Render for MyComponent {
//!     fn render(&self) -> impl IntoElement {
//!         let state = use_state(|| self.0);
//!         rect()
//!     }
//! }
//! ```
//!
//! Or, move state up to the parent component for even simpler code:
//! ```rust
//! # use freya::prelude::*;
//! #[derive(PartialEq)]
//! struct MyComponent(bool);
//! impl Render for MyComponent {
//!     fn render(&self) -> impl IntoElement {
//!         rect()
//!     }
//! }
//! ```
//!
//! ### 2. Only call hooks inside `render` methods
//!
//! Hooks **cannot** be called inside event handlers, async blocks, or outside of components.
//!
//! ❌ **Incorrect:**
//! ```rust
//! # use freya::prelude::*;
//! struct MyComponent;
//! impl Render for MyComponent {
//!     fn render(&self) -> impl IntoElement {
//!         let onclick = |_| {
//!             let state = use_state(|| false); // ❌ Not allowed here
//!         };
//!         rect().on_mouse_up(onclick).child("Hello, World!")
//!     }
//! }
//! ```
//!
//! ✅ **Correct:**
//! ```rust
//! # use freya::prelude::*;
//! struct MyComponent;
//! impl Render for MyComponent {
//!     fn render(&self) -> impl IntoElement {
//!         let mut state = use_state(|| false);
//!         let onclick = move |_| {
//!             state.set(true);
//!         };
//!         rect().on_mouse_up(onclick).child("Hello, World!")
//!     }
//! }
//! ```
//!
//! ### 3. Do not call hooks inside loops
//!
//! The number of hook calls must not change between renders.
//!
//! ❌ **Incorrect:**
//! ```rust
//! # use freya::prelude::*;
//! struct MyComponent;
//! impl Render for MyComponent {
//!     fn render(&self) -> impl IntoElement {
//!         for i in 0..5 {
//!             let state = use_state(|| i); // ❌ Not allowed in a loop
//!         }
//!         rect().child("Hello, World!")
//!     }
//! }
//! ```
//!
//! ✅ **Correct:**
//! ```rust
//! # use freya::prelude::*;
//! struct MyComponent;
//! impl Render for MyComponent {
//!     fn render(&self) -> impl IntoElement {
//!         let state = use_state(|| (0..5).collect::<Vec<_>>());
//!         rect().child("Hello, World!")
//!     }
//! }
//! ```
