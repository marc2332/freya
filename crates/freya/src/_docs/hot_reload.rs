//! # Hot Reload
//!
//! Freya supports hot reloading: while your app is running, changes to your
//! code can be patched into the live process so you see updates without a
//! full rebuild and restart.
//!
//! It is powered by [`subsecond`](https://crates.io/crates/subsecond) and is
//! driven by the `dx` CLI.
//!
//! ## Installing the Dioxus CLI
//!
//! Hot reload is driven by the `dx` binary that ships with the Dioxus CLI.
//! Install it with:
//!
//! ```sh
//! cargo install dioxus-cli
//! ```
//!
//! ## Enabling Hot Reload
//!
//! Hot reload is gated behind the `hotreload` feature flag.
//! The recommended approach is to declare a feature in **your own crate** that
//! forwards to Freya's, so hot reload is never included in default or release
//! builds:
//!
//! ```toml
//! # In your Cargo.toml
//! [features]
//! hotreload = ["freya/hotreload"]
//! ```
//!
//! ## Running Your App with Hot Reload
//!
//! Use `dx serve` with the `--hot-patch` flag and your `hotreload` feature
//! enabled:
//!
//! ```sh
//! dx serve --hot-patch --features hotreload
//! ```
//!
//! `dx` will build and launch your app, then watch your sources. When you save
//! a file, it compiles a patch and pushes it into the running process; Freya
//! re-renders the UI so you can see the change immediately.
//!
//! To hot reload an example instead of the main binary, pass `--example`:
//!
//! ```sh
//! dx serve --hot-patch --features hotreload --example my_example
//! ```
//!
//! ## What Happens on Each Patch
//!
//! When a hot patch arrives, Freya:
//!
//! 1. Cancels every running task spawned with `spawn`.
//! 2. Resets every scope's hook state (`use_state`, `use_memo`, etc.).
//! 3. Marks every scope dirty and triggers a full re-render.
//!
//! In other words, hot reload re-runs your components against the patched
//! code, but **the application state stored in hooks is reset to its initial
//! values** on every patch.
//!
//! ## Limitations
//!
//! Hot reload is a development convenience, not a substitute for a rebuild.
//! Keep these caveats in mind:
//!
//! - **Hook state is reset.** Any value held by `use_state`, `use_memo`, etc.
//!   goes back to its initial value on every patch.
//! - **Tasks are cancelled.** Tasks spawned with `spawn` are dropped when a
//!   patch lands. Long-running background work will need to be restarted by
//!   the components that own it.
//! - **Subsecond patching constraints.** Subsecond patches function bodies in
//!   the running binary. Changes that alter the program's structure (adding
//!   or removing functions, types, or trait impls; changing function
//!   signatures; modifying struct layouts; touching `const`/`static`
//!   initializers) cannot be applied as a patch. When `dx` cannot patch a
//!   change, restart the app to pick it up.
