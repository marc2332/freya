//! # Freya Radio 🧬
//!
//! Fully-typed global state management with a topics subscription system for Freya.
//!
//! Freya Radio provides a powerful way to manage global state in Freya applications
//! with fine-grained control over which components re-render when the state changes.
//!
//! ## Key Concepts
//!
//! - **RadioStation**: The central hub that holds the global state and manages subscriptions.
//! - **RadioChannel**: Defines channels for subscribing to specific types of state changes.
//! - **Radio**: A reactive handle to the state for a specific channel.
//!
//! ## Basic Usage
//!
//! ```rust,no_run
//! use freya::prelude::*;
//! use freya_radio::prelude::*;
//!
//! #[derive(Default, Clone)]
//! struct AppState {
//!     count: i32,
//! }
//!
//! #[derive(PartialEq, Eq, Clone, Debug, Copy, Hash)]
//! enum AppChannel {
//!     Count,
//! }
//!
//! impl RadioChannel<AppState> for AppChannel {}
//!
//! fn app() -> impl IntoElement {
//!     // Initialize the radio station
//!     use_init_radio_station::<AppState, AppChannel>(AppState::default);
//!
//!     rect().child(Counter {})
//! }
//!
//! #[derive(PartialEq)]
//! struct Counter {}
//!
//! impl Component for Counter {
//!     fn render(&self) -> impl IntoElement {
//!         // Subscribe to the Count channel
//!         let mut radio = use_radio(AppChannel::Count);
//!
//!         rect()
//!             .child(format!("Count: {}", radio.read().count))
//!             .child(
//!                 Button::new()
//!                     .on_press(move |_| radio.write().count += 1)
//!                     .child("+"),
//!             )
//!     }
//! }
//! ```
//!
//! ## Advanced Features
//!
//! - **Multiple Channels**: Subscribe to different channels for different types of updates.
//! - **Derived Channels**: Notify multiple channels from a single write operation.
//! - **Reducers**: Implement action-based state updates.
//! - **Multi-window**: Share state across multiple windows using global radio stations.
//!
//! See the examples in the repository for more advanced usage patterns.

pub mod hooks;

pub mod prelude {
    pub use crate::hooks::*;
}
