//! # Introduction
//! 
//! **Freya** is a Rust 🦀 library to make GUI applications that are **cross-platform**, which means they will run the same way in Windows, macOS and Linux.
//! 
//! It uses a model where you compose the app by splitting the UI in different components that return pieces of UI or call other components, this is because
//! Freya runs on 🧬 [Dioxus](https://dioxuslabs.com), a renderer-agnostic UI library inspired by ReactJS.
//! 
//! Even though you might have seen that Dioxus renders to HTML, that it uses WASM, or that it uses CSS. this does not apply to Freya. In fact, Freya only uses some of the core
//! crates of Dioxus, which means that you will be writing Dioxus components and using some of its APIs but, the elements, attributes, styling, layout, events, and more things
//! will be provided by Freya.
//! 
//! Freya uses 🎨 [Skia](https://skia.org/) as rendering engine because its a very battle tested library and has great support for a lot of features.
//! 
//! #### Example
//! 
//! ```rust
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
//!            onclick: move |_| count += 1, // **Update** the state
//!            label { "Click to increase -> {count}" } // Display the **state**
//!        }
//!    )
//! }
//! ```