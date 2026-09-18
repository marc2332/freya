//! # Optimizing
//!
//! Freya enables `gpu` rendering and `accessibility` features by default. Disabling these can optimize the binary size of your application.
//!
//! ```toml
//! # Software renderer + no accessibility backend
//! [dependencies]
//! freya = { version = "...", default-features = false, features = ["winit"] }
//! ```
//! You can always enable one of the features individually if you wanted, e.g
//!
//! ```toml
//! # No accessibility backend
//! [dependencies]
//! freya = { version = "...", default-features = false, features = ["winit", "gpu"] }
//! ```
