//! # Introduction
//!
//! **Freya** is a Rust 🦀 library to make GUI applications that are [**cross-platform**](https://en.wikipedia.org/wiki/Cross-platform_software), targeting Windows, macOS and Linux.
//!
//! Freya uses a [declarative](https://en.wikipedia.org/wiki/Declarative_programming) model for the UI, and components to encapculate the UI in reusable pieces of code.
//!
//! Freya renders using 🎨 [Skia](https://skia.org/) because its a very battle tested library and has great support for a lot of features.
//!
//! #### Example
//!
//! ```rust, no_run
//! # use freya::prelude::*;
//! fn main() {
//!     // **Start** your app with a window and its root component
//!     launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
//! }
//!
//! fn app() -> Element {
//!     // Define a **state**
//!     let mut count = use_state(|| 0);
//!
//!     // Declare the **UI**
//!     rect()
//!         .width(Size::fill())
//!         .height(Size::fill())
//!         .background((35, 35, 35))
//!         .color(Color::WHITE)
//!         .padding(Gaps::new_all(12.))
//!         .on_mouse_up(move |_| *count.write() += 1)
//!         .child(format!("Click to increase -> {}", count.read()))
//!         .into()
//! }
//! ```
