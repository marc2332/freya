//! # Devtools
//!
//! Freya ships with a companion devtools application that lets you inspect and debug your running app in real time.
//! With it you can browse the node tree, inspect element styles, layout, and text styles, highlight elements on hover, and control animation speed.
//!
//! ## Enabling Devtools
//!
//! The devtools server is gated behind the `devtools` feature flag.
//! The recommended approach is to declare a feature in **your own crate** that forwards to Freya's, so devtools are never included in default or release builds:
//!
//! ```toml
//! # In your Cargo.toml
//! [features]
//! devtools = ["freya/devtools"]
//! ```
//!
//! Then run your app with the feature enabled:
//!
//! ```sh
//! cargo run --features devtools
//! ```
//!
//! No code changes are needed. When the feature is active, Freya's `launch` function automatically registers the devtools server plugin.
//!
//! ## Running the Devtools App
//!
//! The devtools UI is a separate standalone application (`freya-devtools-app`).
//! Install it once with:
//!
//! ```sh
//! cargo install freya-devtools-app
//! ```
//!
//! Then, while your app is running with the `devtools` feature enabled, start the devtools app:
//!
//! ```sh
//! freya-devtools-app
//! ```
//!
//! The devtools app will connect to your running application automatically. If the app is not running yet it will keep retrying until it connects.
//!
//! ## Limitations
//!
//! Only **one** Freya application with devtools enabled can run at a time.
//! The embedded server listens on a fixed port (`7354` on localhost), so launching a second devtools-enabled app will fail to bind that port.
//! Make sure to close any previous devtools-enabled instance before starting a new one.
