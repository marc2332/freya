//! # Platforms
//!
//! Freya supports multiple desktop platforms, each with a specific graphics backend configuration.
//! The rendering is powered by [Skia](https://skia.org/) through the `skia-safe` bindings.
//!
//! ## Supported Platforms
//!
//! | Platform | Graphics Backend |
//! |----------|-----------------|
//! | Linux | Vulkan (preferred), OpenGL (fallback) |
//! | Windows | Vulkan (preferred), OpenGL (fallback) |
//! | macOS | Metal |
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
