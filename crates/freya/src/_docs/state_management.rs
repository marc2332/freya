//! # State Management
//!
//! Dioxus and Freya apps, have multiple ways of state management.
//!
//! See the different features:
//!
//! - [Signals](self::signals)
//! - [Global Signals](self:::global_signals)
//! - [Lifecycle](self:::lifecycle)
//! - [Context](self:::context)
//! - [Memoization](self:::memoization)

pub mod context;
pub mod global_signals;
pub mod memoization;
pub mod signals;
