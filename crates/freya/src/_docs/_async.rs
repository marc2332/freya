//! # Async
//!
//! Freya runs futures on its own executor so async tasks can update reactive
//! state directly. The async primitives below are the building blocks: use them
//! for one-off tasks like timers, cancelable background work, or fire-and-forget
//! requests.
//!
//! ## `spawn`
//!
//! [`spawn`](crate::prelude::spawn) starts a future tied to the **current
//! component scope**. When the component unmounts, the task is automatically
//! cancelled.
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! #[derive(PartialEq)]
//! struct DelayedCounter;
//!
//! impl Component for DelayedCounter {
//!     fn render(&self) -> impl IntoElement {
//!         let mut count = use_state(|| 0);
//!
//!         Button::new()
//!             .on_press(move |_| {
//!                 spawn(async move {
//!                     // Some async work...
//!                     *count.write() += 1;
//!                 });
//!             })
//!             .child(format!("Count: {}", count.read()))
//!     }
//! }
//! ```
//!
//! `spawn` returns a [`TaskHandle`](crate::prelude::TaskHandle) that you can
//! use to cancel the task manually. Calling [`.owned()`](crate::prelude::TaskHandle::owned)
//! upgrades it to an [`OwnedTaskHandle`](crate::prelude::OwnedTaskHandle) that
//! cancels the task when the last clone is dropped.
//!
//! ## `spawn_forever`
//!
//! [`spawn_forever`](crate::prelude::spawn_forever) is like `spawn` but the task
//! is attached to the **root scope**. It keeps running until the app exits or
//! you cancel it explicitly, regardless of the component that started it.
//!
//! Use it for app-wide background work (a sync loop, periodic refreshes, etc.)
//! that should outlive the component that spawned it.
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! # fn app() -> impl IntoElement {
//! use_hook(|| {
//!     spawn_forever(async {
//!         loop {
//!             // App-level background loop
//!         }
//!     });
//! });
//! # rect()
//! # }
//! ```
//!
//! ## `use_future`
//!
//! [`use_future`](crate::prelude::use_future) is a hook around `spawn` that
//! exposes the future's progress as reactive state. It is convenient when you
//! want to render different UI for *pending*, *loading*, and *fulfilled*
//! states without managing the task by hand.
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! #[derive(PartialEq)]
//! struct Greeting;
//!
//! impl Component for Greeting {
//!     fn render(&self) -> impl IntoElement {
//!         let future = use_future(|| async {
//!             // Some async work...
//!             "Hello!".to_string()
//!         });
//!
//!         match &*future.state() {
//!             FutureState::Pending | FutureState::Loading => "Loading...".to_string(),
//!             FutureState::Fulfilled(text) => text.clone(),
//!         }
//!     }
//! }
//! ```
//!
//! The returned [`FutureTask`](crate::prelude::FutureTask) exposes
//! [`start`](crate::prelude::FutureTask::start) and
//! [`cancel`](crate::prelude::FutureTask::cancel) so you can restart or stop
//! the future later.
