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
//! | Linux | Vulkan (preferred), OpenGL (fallback) |
//! | Windows | Vulkan (preferred), OpenGL (fallback) |
//! | macOS | Metal |
//! | Android (experimental) | OpenGL |
//! | Web (experimental) | WebGL |
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
//! ### WebGL (Web)
//!
//! Used to render in the browser, where Freya draws into a `canvas` element through Skia's OpenGL
//! backend on top of WebGL 2.
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
