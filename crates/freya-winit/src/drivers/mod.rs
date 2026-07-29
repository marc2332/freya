#[cfg(all(
    any(target_os = "linux", target_os = "windows", target_os = "android"),
    feature = "gpu"
))]
mod gl;
#[cfg(all(target_os = "macos", feature = "gpu"))]
mod metal;
mod software;
#[cfg(all(any(target_os = "linux", target_os = "windows"), feature = "gpu"))]
mod vulkan;

use freya_engine::prelude::Surface as SkiaSurface;
use winit::{
    dpi::PhysicalSize,
    event_loop::ActiveEventLoop,
    window::{
        Window,
        WindowAttributes,
    },
};

#[allow(clippy::large_enum_variant)]
pub enum GraphicsDriver {
    #[cfg(all(
        any(target_os = "linux", target_os = "windows", target_os = "android"),
        feature = "gpu"
    ))]
    OpenGl(gl::OpenGLDriver),
    #[cfg(all(target_os = "macos", feature = "gpu"))]
    Metal(metal::MetalDriver),
    #[cfg(all(any(target_os = "linux", target_os = "windows"), feature = "gpu"))]
    Vulkan(vulkan::VulkanDriver),
    Software(software::SoftwareDriver),
}

impl GraphicsDriver {
    #[allow(clippy::needless_return, unreachable_code, unused_variables)]
    pub fn new(
        event_loop: &ActiveEventLoop,
        window_attributes: WindowAttributes,
        gpu_resource_cache_limit: usize,
    ) -> (Self, Window) {
        let renderer = std::env::var("FREYA_RENDERER")
            .ok()
            .map(|v| v.to_ascii_lowercase());
        let renderer = renderer.as_deref();

        // Opt-in via FREYA_RENDERER=software, available on every platform.
        if renderer == Some("software") {
            match software::SoftwareDriver::new(event_loop, window_attributes.clone()) {
                Ok((driver, window)) => return (Self::Software(driver), window),
                Err(err) => {
                    tracing::warn!(
                        "Software renderer initialization failed, falling back to default: {err}"
                    );
                }
            }
        }

        // Metal (macOS)
        #[cfg(all(target_os = "macos", feature = "gpu"))]
        {
            let (driver, window) =
                metal::MetalDriver::new(event_loop, window_attributes, gpu_resource_cache_limit);

            return (Self::Metal(driver), window);
        }

        // OpenGL only on Android.
        #[cfg(all(target_os = "android", feature = "gpu"))]
        {
            match gl::OpenGLDriver::new(
                event_loop,
                window_attributes.clone(),
                gpu_resource_cache_limit,
            ) {
                Ok((driver, window)) => return (Self::OpenGl(driver), window),
                Err(err) => {
                    tracing::warn!("OpenGL initialization failed, falling back to software: {err}");
                }
            }
        }

        // Linux: Vulkan by default, set FREYA_RENDERER=opengl to force OpenGL.
        // Windows: OpenGL by default, set FREYA_RENDERER=vulkan to force Vulkan.
        // If both fail, falls back to the software renderer.
        #[cfg(all(any(target_os = "linux", target_os = "windows"), feature = "gpu"))]
        {
            let use_vulkan = if cfg!(target_os = "windows") {
                renderer == Some("vulkan")
            } else {
                renderer != Some("opengl")
            };

            if use_vulkan {
                match vulkan::VulkanDriver::new(
                    event_loop,
                    window_attributes.clone(),
                    gpu_resource_cache_limit,
                ) {
                    Ok((driver, window)) => return (Self::Vulkan(driver), window),
                    Err(err) => {
                        tracing::warn!("Vulkan initialization failed, falling back: {err}");
                    }
                }
            }
        }

        #[cfg(all(any(target_os = "linux", target_os = "windows"), feature = "gpu"))]
        {
            match gl::OpenGLDriver::new(
                event_loop,
                window_attributes.clone(),
                gpu_resource_cache_limit,
            ) {
                Ok((driver, window)) => return (Self::OpenGl(driver), window),
                Err(err) => {
                    tracing::warn!("OpenGL initialization failed, falling back to software: {err}");
                }
            }
        }

        let (driver, window) = software::SoftwareDriver::new(event_loop, window_attributes)
            .expect("Failed to initialize software renderer");
        (Self::Software(driver), window)
    }

    pub fn present(
        &mut self,
        _size: PhysicalSize<u32>,
        window: &Window,
        render: impl FnOnce(&mut SkiaSurface),
    ) {
        match self {
            #[cfg(all(
                any(target_os = "linux", target_os = "windows", target_os = "android"),
                feature = "gpu"
            ))]
            Self::OpenGl(gl) => gl.present(window, render),
            #[cfg(all(target_os = "macos", feature = "gpu"))]
            Self::Metal(mtl) => mtl.present(_size, window, render),
            #[cfg(all(any(target_os = "linux", target_os = "windows"), feature = "gpu"))]
            Self::Vulkan(vk) => vk.present(_size, window, render),
            Self::Software(sw) => sw.present(_size, window, render),
        }
    }

    /// The name of the active graphics driver.
    pub fn name(&self) -> &'static str {
        match self {
            #[cfg(all(
                any(target_os = "linux", target_os = "windows", target_os = "android"),
                feature = "gpu"
            ))]
            Self::OpenGl(_) => "OpenGL",
            #[cfg(all(target_os = "macos", feature = "gpu"))]
            Self::Metal(_) => "Metal",
            #[cfg(all(any(target_os = "linux", target_os = "windows"), feature = "gpu"))]
            Self::Vulkan(_) => "Vulkan",
            Self::Software(_) => "Software",
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        match self {
            #[cfg(all(
                any(target_os = "linux", target_os = "windows", target_os = "android"),
                feature = "gpu"
            ))]
            Self::OpenGl(gl) => gl.resize(size),
            #[cfg(all(target_os = "macos", feature = "gpu"))]
            Self::Metal(mtl) => mtl.resize(size),
            #[cfg(all(any(target_os = "linux", target_os = "windows"), feature = "gpu"))]
            Self::Vulkan(vk) => vk.resize(size),
            Self::Software(sw) => sw.resize(size),
        }
    }
}
