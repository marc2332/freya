//! # Introduction
//!
//! **Freya** is a Rust 🦀 library to make GUI applications that are [**cross-platform**](https://en.wikipedia.org/wiki/Cross-platform_software), targeting Windows, macOS and Linux.
//!
//! Freya uses a [declarative](https://en.wikipedia.org/wiki/Declarative_programming) model for the UI, and components to encapculate the UI in reusable pieces of code. This is because
//! Freya uses the core crates 🧬 [Dioxus](https://dioxuslabs.com), a renderer-agnostic components-based and declarative UI library inspired by ReactJS.
//!
//! You might have seen that Dioxus render to HTML, use CSS/JavaScript/WASM/WebView/WGPU. this does not apply to Freya.
//!
//! As Freya only uses some of the core crates of Dioxus, you will be writing Dioxus components and using some of its APIs but, the elements, attributes, styling, layout, events,
//! and the rest of things will be provided by Freya.
//!
//! Freya also uses 🎨 [Skia](https://skia.org/) as rendering engine because its a very battle tested library and has great support for a lot of features.
//!
//! #### Example
//!
//! ```rust, no_run
//! # use freya::prelude::*;
//! fn main() {
//!     // **Start** your app by specifying the root component and some config parameters
//!     launch_with_props(app, "Counter", (400.0, 350.0));
//! }
//!
//! fn app() -> Element {
//!    // Define a **state**
//!    let mut count = use_signal(|| 0);
//!
//!    // Declare the **UI**
//!    rsx!(
//!        rect {
//!            height: "100%",
//!            width: "100%",
//!            background: "rgb(35, 35, 35)",
//!            color: "white",
//!            padding: "12",
//!            onclick: move |_| count += 1, // **Update** the state on click events
//!            label { "Click to increase -> {count}" } // Display the **state**
//!        }
//!    )
//! }
//! ```
//!
//! You can continue to learn more in [UI](self::_docs::ui).
