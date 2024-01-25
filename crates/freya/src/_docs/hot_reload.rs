//! # Hot reload
//!
//! Freya supports Dioxus hot reload, which means that you can update the `layout` and `styling` of your app on the fly, without having to recompile your project.
//!
//! ## Setup
//!
//! Before launching your app, you need to initialize the hot-reload context:
//!
//! ```rust, no_run
//! use freya::prelude::*;
//! use freya::hotreload::FreyaCtx;
//!
//! fn main() {
//!     dioxus_hot_reload::hot_reload_init!(Config::<FreyaCtx>::default());
//!
//!     launch(app);
//! }
//!
//! # fn app() -> Element {
//! #     None
//! # }
//! ```
//!
//! That is it!
