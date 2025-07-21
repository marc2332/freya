//! # Async
//!
//! You may run asynchronous code through the different APIs Dioxus provide.
//!
//! Using third-party libraries such as tokio to spawn tasks could work but it's not recommended, these will not work with the lifecycling of the components.
//!
//! ### `spawn`
//!
//! With [`spawn`](dioxus_core::spawn) you can spawn an **async task** (Also known as green threads), this is primarily targeted for custom hooks or when you want to run some async code dynamically such as from an event listener.
//!
//! **Important:** Tasks spawned with `spawn` will be cancelled when the component their were created is dropped.
//! If you want to have an async tasks not attached to the component you may use [`spawn_forever`](dioxus_core::spawn_forever).
//!
//! ```rust
//! # use freya::prelude::*;
//! fn app() -> Element {
//!     rsx!(Button {
//!         onclick: |_| {
//!             if 1 == 1 {
//!                 spawn(async move {
//!                     println!("Hello, World fom an async task!");
//!                 });
//!             }
//!         }
//!     })
//! }
//! ```
//!
//! You can also use hooks like [`use_future`](dioxus::prelude::use_future) or [`use_resource`](dioxus::prelude::use_resource).
