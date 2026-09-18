//! # Optimizing
//!
//! ## Features
//!
//! Freya enables `gpu` rendering and `accessibility` features by default. Disabling these can optimize the binary size of your application.
//!
//! ```toml
//! # Software renderer + no accessibility backend
//! [dependencies]
//! freya = { version = "...", default-features = false, features = ["winit"] }
//! ```
//! You can always enable one of the features individually if you wanted, e.g
//!
//! ```toml
//! # No accessibility backend
//! [dependencies]
//! freya = { version = "...", default-features = false, features = ["winit", "gpu"] }
//! ```
//!
//! ## Renderer preference
//!
//! See [`Platforms`](crate::_docs::platforms) for the renderer used by each operating system.
//! Choose a renderer with [`crate::prelude::WindowConfig::with_renderer`]. `Auto` is the default and respects
//! `FREYA_RENDERER` before applying the platform defaults. The other options are `Software`,
//! `OpenGl`, and `Vulkan`.
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! launch(
//!     LaunchConfig::new().with_window(
//!         WindowConfig::new(app).with_renderer(RendererPreference::Software),
//!     ),
//! );
//! # fn app() -> impl IntoElement { "" }
//! ```
//!
//! Use `RendererPreference::Software` to avoid GPU initialization, or select a specific GPU
//! backend when debugging or targeting a known platform. The `gpu` feature is required for GPU
//! backends.
//!
//! ## GPU cache
//!
//! Set the Skia GPU resource cache limit with
//! [`crate::prelude::LaunchConfig::with_gpu_resource_cache_limit`]. It applies to every window
//! and defaults to [`crate::prelude::GpuResourceCacheLimit::Normal`] (256 MiB).
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! launch(
//!     LaunchConfig::new()
//!         .with_gpu_resource_cache_limit(GpuResourceCacheLimit::Small)
//!         .with_window(WindowConfig::new(app)),
//! );
//! # fn app() -> impl IntoElement { "" }
//! ```
//!
//! Use `Small`, `Normal`, or `Large`, or create a custom limit with
//! `GpuResourceCacheLimit::from_mb(512)`. Lower limits reduce GPU memory use. Larger limits can
//! help apps that use many cached images, paths, or other GPU resources.
//!
//! ## Image and SVG viewers
//!
//! Avoid decoding images larger than their display size with
//! [`crate::components::ImageViewer::decode_mode`]. [`crate::components::DecodeMode::FromLayout`]
//! is the default and uses the pixel-sized layout when both dimensions are fixed. Use
//! [`crate::components::DecodeMode::Custom`] to cap the decoded size, or
//! [`crate::components::DecodeMode::Source`] to keep the source size.
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! # let source = ImageSource::from(("image", &[] as &[u8]));
//! ImageViewer::new(source)
//!     .decode_mode(DecodeMode::Custom(Size2D::new(800., 600.)))
//!     .sampling_mode(SamplingMode::Bilinear);
//! ```
//!
//! Set [`crate::prelude::SamplingMode`] with `sampling_mode` to choose the image scaling filter. Cheaper filters
//! such as `Nearest` or `Bilinear` reduce rendering work, while `Mitchell` and `CatmullRom` favor
//! quality.
//!
//! [`crate::components::SvgViewer`] rasterizes SVGs at their layout size. Set
//! [`crate::components::SvgViewer::parallel`] to `true` to rasterize local SVGs in a background
//! thread. Remote SVGs already rasterize in the background.
