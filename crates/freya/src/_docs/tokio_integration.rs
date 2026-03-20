//! # Tokio Integration
//!
//! Freya has its own async runtime, but many Rust ecosystem crates depend on
//! [Tokio](https://tokio.rs/): HTTP clients, database drivers, gRPC libraries, and more.
//! This page explains how to set up a Tokio runtime alongside Freya so you can use
//! those crates in your application.
//!
//! ## Setting up the Runtime
//!
//! Create a Tokio runtime in `main()` and enter its context before launching Freya.
//! The [`enter()`](https://docs.rs/tokio/latest/tokio/runtime/struct.Runtime.html#method.enter)
//! guard makes Tokio APIs available on the current thread without
//! requiring `#[tokio::main]`:
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! use tokio::runtime::Builder;
//!
//! fn main() {
//!     let rt = Builder::new_multi_thread().enable_all().build().unwrap();
//!     // Enter the Tokio context so its APIs (channels, timers, etc.) work.
//!     let _rt = rt.enter();
//!
//!     launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
//! }
//!
//! fn app() -> impl IntoElement {
//!     rect()
//! }
//! ```
//!
//! The `_rt` guard must remain alive for the duration of the program.
//! Binding it to a variable in `main()` achieves this.
//!
//! ## What you can use
//!
//! With the Tokio context active, you can use:
//!
//! - **`tokio::sync`** channels (`mpsc`, `watch`, `broadcast`, `oneshot`, etc.)
//! - **`tokio::time`** utilities (`sleep`, `interval`, `timeout`)
//! - Any ecosystem crate that requires a Tokio runtime (e.g. `reqwest`, `sqlx`, `tonic`)
//!
//! These work well with Freya's built-in `spawn()`:
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! # fn app() -> impl IntoElement {
//! let mut count = use_state(|| 0);
//!
//! use_hook(move || {
//!     spawn(async move {
//!         // tokio::time::sleep works because the Tokio context is active
//!         tokio::time::sleep(std::time::Duration::from_secs(2)).await;
//!         count.set(42);
//!     });
//! });
//! # rect()
//! # }
//! ```
//!
//! ## Caveat about `tokio::spawn`
//!
//! `tokio::spawn` runs tasks on the **Tokio** runtime, which is separate from
//! Freya's reactivity system. Tasks spawned there **cannot directly update
//! component state**. Signals, hooks, and other reactive primitives are
//! tied to Freya's own executor.
//!
//! Use `tokio::spawn` only for background work that does not interact with
//! components (e.g. writing to a file, sending a network request whose
//! result you don't need in the UI). For anything that updates the UI, use
//! Freya's [`spawn()`](freya_core::prelude::spawn) instead.
//!
//! ## Watch Channels with `use_track_watcher`
//!
//! You can push data **into** Freya from an external source
//! (a background thread, another runtime, a hardware driver, etc.).
//! `tokio::sync::watch` channels pair well with the
//! [`use_track_watcher`](crate::sdk::use_track_watcher) hook from the `sdk`
//! feature:
//!
//! ```toml
//! [dependencies]
//! freya = { version = "...", features = ["sdk"] }
//! tokio = { version = "1", features = ["sync"] }
//! ```
//!
//! The sender can live on any thread. `use_track_watcher` re-renders the
//! component whenever the watch value changes:
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! # use freya::sdk::use_track_watcher;
//! # use tokio::sync::watch;
//! # use std::time::Duration;
//! fn main() {
//!     let (tx, rx) = watch::channel(0);
//!
//!     // Producer: a plain thread that pushes new values.
//!     std::thread::spawn(move || {
//!         let mut i = 0;
//!         loop {
//!             std::thread::sleep(Duration::from_secs(1));
//!             i += 1;
//!             let _ = tx.send(i);
//!         }
//!     });
//!
//!     launch(LaunchConfig::new().with_window(WindowConfig::new_app(MyApp { rx })))
//! }
//!
//! struct MyApp {
//!     rx: watch::Receiver<i32>,
//! }
//!
//! impl App for MyApp {
//!     fn render(&self) -> impl IntoElement {
//!         // Re-renders this component whenever the watch value changes.
//!         use_track_watcher(&self.rx);
//!
//!         rect()
//!             .expanded()
//!             .center()
//!             .child(format!("Latest value is {}", *self.rx.borrow()))
//!     }
//! }
//! ```
//!
//! ## Examples
//!
//! - [`integration_tokio.rs`](https://github.com/marc2332/freya/tree/main/examples/integration_tokio.rs) - Minimal Tokio runtime setup
//! - [`sdk_watch.rs`](https://github.com/marc2332/freya/tree/main/examples/sdk_watch.rs) - Reactive watch channel with `use_track_watcher`
