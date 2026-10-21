//! # Platforms
//!
//! Freya supports multiple desktop platforms plus experimental Android and Web support, each with a
//! specific graphics backend configuration. The rendering is powered by [Skia](https://skia.org/) through the
//! `skia-safe` bindings.
//!
//! ## Supported Platforms
//!
//! | Platform | Graphics Backend |
//! |----------|-----------------|
//! | Linux | Vulkan (preferred), OpenGL (fallback), Software (fallback) |
//! | Windows | Vulkan (preferred), OpenGL (fallback), Software (fallback) |
//! | macOS | Metal, Software (fallback) |
//! | Android (experimental) | OpenGL, Software (fallback) |
//! | Web (experimental) | WebGL |
//!
//! ## Rendering Backends
//!
//! ### Vulkan (Linux, Windows)
//!
//! On Wayland, auto mode first tries Vulkan on the compositor's preferred GPU from DMA-BUF feedback.
//! If that GPU cannot be matched or initialized, Freya tries OpenGL with the platform's GPU selection.
//! If OpenGL also fails, Freya falls back to software rendering.
//! On other Linux desktops and Windows, auto mode prefers an integrated Vulkan GPU when available.
//!
//! ### OpenGL (Linux, Windows)
//!
//! Used as a fallback on Linux and Windows when the preferred Vulkan GPU cannot initialize.
//! For debugging purposes, you can force OpenGL by setting the `FREYA_RENDERER` environment variable:
//!
//! ```sh
//! FREYA_RENDERER=opengl cargo run
//! ```
//!
//! ### Metal (macOS)
//!
//! The best graphics backend for macOS.
//!
//! ### OpenGL (Android)
//!
//! Used to render on Android, where Freya draws through Skia's OpenGL backend.
//!
//! ### WebGL (Web)
//!
//! Used to render in the browser, where Freya draws into a `canvas` element through Skia's OpenGL
//! backend on top of WebGL 2.
//!
//! ### Software (all platforms)
//!
//! CPU-based rendering used as a last-resort fallback when no GPU backend could be initialized.
//! You can also force it at runtime:
//!
//! ```sh
//! FREYA_RENDERER=software cargo run
//! ```
//!
//! To build a software-only app without any of the GPU backends and their dependencies,
//! disable the default `gpu` feature:
//!
//! ```toml
//! freya = { version = "...", default-features = false, features = ["winit"] }
//! ```
//!
//! ## Android
//!
//! Android support is highly experimental.
//!
//! Building for Android requires the Android SDK, the NDK and `cargo-ndk`. See the
//! [`android`](https://github.com/marc2332/freya/tree/main/examples/android) example for a complete
//! project setup and step-by-step build instructions.
//!
//! ## Web
//!
//! Web support is experimental. Apps are compiled to WebAssembly with Emscripten and run in a
//! `canvas` element. See the [`web`](https://github.com/marc2332/freya/tree/main/examples/web)
//! example for the project setup and build instructions, it is what runs at
//! [freyaui.dev/demo](https://freyaui.dev/demo).
