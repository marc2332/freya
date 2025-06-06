//! # State Management
//!
//! Here are the multiple built-in ways of state management you can use in Freya:
//!
//! - [Signals](crate::_docs::state_management::signals)
//! - [Global Signals](crate::_docs::state_management::global_signals)
//! - [Lifecycle](crate::_docs::state_management::lifecycle)
//! - [Context](crate::_docs::state_management::context)
//! - [Memoization](crate::_docs::state_management::memoization)

pub mod context;
pub mod global_signals;
pub mod lifecycle;
pub mod memoization;
pub mod signals;
