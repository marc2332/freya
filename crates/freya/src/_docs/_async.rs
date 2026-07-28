//! # Async
//!
//! Freya has its own async runtime, so tasks can update reactive state directly.
//! These are the async primitives, each one documents when to use it and how in
//! its own definition.
//!
//! ## Tasks
//!
//! - [`spawn`](crate::prelude::spawn) runs a future attached to the current
//!   component scope, it gets cancelled when that component unmounts. Use it for
//!   async work owned by a component.
//! - [`spawn_forever`](crate::prelude::spawn_forever) runs a future attached to
//!   the root scope, it keeps running until the app exits. Use it for app-wide
//!   background work that must outlive the component that started it.
//!
//! Both return a [`TaskHandle`](crate::prelude::TaskHandle) to cancel the task
//! manually. [`.owned()`](crate::prelude::TaskHandle::owned) upgrades it to an
//! [`OwnedTaskHandle`](crate::prelude::OwnedTaskHandle) that cancels the task
//! when its last clone is dropped.
//!
//! ## Hooks
//!
//! - [`use_future`](crate::prelude::use_future) wraps `spawn` and exposes the
//!   progress of the future as reactive state through a
//!   [`FutureTask`](crate::prelude::FutureTask), so you can render the
//!   [`Pending`](crate::prelude::FutureState::Pending),
//!   [`Loading`](crate::prelude::FutureState::Loading) and
//!   [`Fulfilled`](crate::prelude::FutureState::Fulfilled) cases without
//!   managing the task by hand.
//!
//! ## See also
//!
//! - [Tokio Integration](crate::_docs::tokio_integration) to use crates that
//!   depend on Tokio.
//! - [State Management](crate::_docs::state_management) for caching and syncing
//!   async data with Freya Query.
