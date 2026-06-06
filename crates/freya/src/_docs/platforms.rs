//! # Platforms
//!
//! Freya supports multiple desktop platforms plus experimental Android support, each with a specific
//! graphics backend configuration. The rendering is powered by [Skia](https://skia.org/) through the
//! `skia-safe` bindings.
//!
//! ## Supported Platforms
//!
//! | Platform | Graphics Backend |
//! |----------|-----------------|
//! | Linux | Vulkan (preferred), OpenGL (fallback) |
//! | Windows | Vulkan (preferred), OpenGL (fallback) |
//! | macOS | Metal |
//! | Android (experimental) | OpenGL |
//!
//! ## Rendering Backends
//!
//! ### Vulkan (Linux, Windows)
//!
//! The default and preferred rendering backend on Linux and Windows. Vulkan provides modern, high-performance
//! GPU-accelerated rendering. Freya will use Vulkan when available.
//!
//! ### OpenGL (Linux, Windows)
//!
//! Used as a fallback on Linux and Windows when Vulkan is not available or not supported by the hardware.
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
//! ## Android
//!
//! Android support is highly experimental. Soft keyboard and IME support are not yet implemented,
//! so `Input` components and anything relying on keyboard input will not work properly yet.
//!
//! Building for Android requires the Android SDK, the NDK and `cargo-ndk`. See the
//! [`android`](https://github.com/marc2332/freya/tree/main/examples/android) example for a complete
//! project setup and step-by-step build instructions.
