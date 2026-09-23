//! # Sharing GPU textures with wgpu
//!
//! An external renderer built on wgpu, such as a 3D viewport, can draw into a texture that Freya
//! shows as a regular image, with no copy through the CPU. It needs the `wgpu` feature.
//!
//! Freya does not import textures from a foreign device. Instead wgpu creates the device and Freya
//! builds its swapchain and Skia context on top of it, so both renderers submit to the same queue
//! and stay ordered without any semaphore of their own.
//!
//! ## Launching on a shared device
//!
//! Create the device before launching, give the raw handles to the launch config and register the
//! plugin that exposes the device to components.
//!
//! ```rust,no_run
//! use freya::{
//!     prelude::*,
//!     wgpu::{
//!         WgpuSetup,
//!         WgpuSetupOptions,
//!     },
//! };
//! # fn app() -> impl IntoElement { rect() }
//!
//! async fn launch_shared() -> Result<(), Box<dyn std::error::Error>> {
//!     let setup = WgpuSetup::new(WgpuSetupOptions::default()).await?;
//!
//!     launch(
//!         LaunchConfig::new()
//!             .with_external_gpu_device(setup.external_gpu_device()?)
//!             .with_plugin(setup.plugin())
//!             .with_window(WindowConfig::new(app)),
//!     );
//!
//!     Ok(())
//! }
//! ```
//!
//! Creating the device is the only async step, block on it from `main` with any executor.
//!
//! ## Drawing into a viewport
//!
//! [WgpuViewer](freya_wgpu::WgpuViewer) is the usual way in. It keeps one texture matching the
//! area it fills, recreating it when the area resizes or the device is lost, and calls the renderer
//! at the moment Freya paints, which is the only point where the laid out size is known.
//!
//! ```rust,no_run
//! use freya::{
//!     prelude::*,
//!     wgpu::prelude::*,
//! };
//!
//! fn app() -> impl IntoElement {
//!     WgpuViewer::new(|texture, frame| {
//!         // Record and submit a pass into `texture.view()` with `frame.context`.
//!         let _ = (texture.view(), frame.width, frame.height);
//!     })
//! }
//! ```
//!
//! Submit your work inside that callback rather than from a component render. Both renderers share
//! one queue, so submitting during the paint is what keeps your frame ordered before Skia reads it.
//!
//! ## Bringing your own textures
//!
//! [wgpu_viewport](freya_wgpu::wgpu_viewport) is the element underneath, for renderers that manage
//! several textures or get them from somewhere else. It hands over the size and takes the image to
//! draw, and nothing else is managed for you.
//!
//! ```rust,no_run
//! use freya::{
//!     prelude::*,
//!     wgpu::prelude::*,
//! };
//! # use std::{cell::RefCell, rc::Rc};
//!
//! fn app() -> impl IntoElement {
//!     let texture = use_hook(|| Rc::new(RefCell::new(None::<GpuTexture>)));
//!
//!     wgpu_viewport(move |frame| {
//!         let descriptor = GpuTextureDescriptor::new(frame.width, frame.height);
//!         let mut texture = texture.borrow_mut();
//!         if texture
//!             .as_ref()
//!             .is_none_or(|texture| texture.descriptor() != descriptor)
//!         {
//!             *texture = Some(GpuTexture::new(&frame.context, descriptor));
//!         }
//!
//!         // Render into the texture, then hand its image to Freya.
//!         texture.as_ref()?.image_handle()
//!     })
//! }
//! ```
//!
//! A [GpuTexture](freya_wgpu::GpuTexture) you drop is not destroyed straight away. It is held until
//! the frames still reading it have finished on the GPU, so replacing one mid resize is safe.
//!
//! ## What is supported
//!
//! Sharing works on Vulkan for Linux and Windows, and on Metal for macOS. Windows defaults to the
//! OpenGL driver, so a shared device there switches it to Vulkan.
//!
//! When the device cannot be shared, the app still launches on a device of Freya's own,
//! [WgpuContext::is_shared](freya_wgpu::WgpuContext::is_shared) reports `false` and
//! `image_handle` returns `None`, so components can fall back to something else.
//!
//! A lost device rebuilds the graphics driver and invalidates every texture created on the old one.
//! [WgpuContext::generation](freya_wgpu::WgpuContext::generation) changes when that happens, and
//! [GpuTexture::is_valid](freya_wgpu::GpuTexture::is_valid) reports it per texture.
