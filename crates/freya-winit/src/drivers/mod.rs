#[cfg(any(target_os = "linux", target_os = "windows", target_os = "android"))]
mod gl;
#[cfg(target_os = "macos")]
mod metal;
#[cfg(any(target_os = "linux", target_os = "windows"))]
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
    #[cfg(any(target_os = "linux", target_os = "windows", target_os = "android"))]
    OpenGl(gl::OpenGLDriver),
    #[cfg(target_os = "macos")]
    Metal(metal::MetalDriver),
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    Vulkan(vulkan::VulkanDriver),
}

impl GraphicsDriver {
    #[allow(clippy::needless_return)]
    pub fn new(
        event_loop: &ActiveEventLoop,
        window_attributes: WindowAttributes,
        gpu_resource_cache_limit: usize,
    ) -> (Self, Window) {
        // Metal (macOS)
        #[cfg(target_os = "macos")]
        {
            let (driver, window) =
                metal::MetalDriver::new(event_loop, window_attributes, gpu_resource_cache_limit);

            return (Self::Metal(driver), window);
        }

        // OpenGL only on Android.
        #[cfg(target_os = "android")]
        {
            let (driver, window) =
                gl::OpenGLDriver::new(event_loop, window_attributes, gpu_resource_cache_limit);

            return (Self::OpenGl(driver), window);
        }

        // Linux: Vulkan by default, set FREYA_RENDERER=opengl to force OpenGL.
        // Windows: OpenGL by default, set FREYA_RENDERER=vulkan to force Vulkan.
        #[cfg(all(not(target_os = "macos"), not(target_os = "android")))]
        {
            let renderer = std::env::var("FREYA_RENDERER");

            let use_vulkan = if cfg!(target_os = "windows") {
                renderer.is_ok_and(|v| v.eq_ignore_ascii_case("vulkan"))
            } else {
                !renderer.is_ok_and(|v| v.eq_ignore_ascii_case("opengl"))
            };

            if use_vulkan {
                let vk_attrs = window_attributes.clone();
                match vulkan::VulkanDriver::new(event_loop, vk_attrs, gpu_resource_cache_limit) {
                    Ok((driver, window)) => return (Self::Vulkan(driver), window),
                    Err(err) => {
                        tracing::warn!(
                            "Vulkan initialization failed, falling back to OpenGL: {err}"
                        );
                    }
                }
            }

            let (driver, window) =
                gl::OpenGLDriver::new(event_loop, window_attributes, gpu_resource_cache_limit);

            return (Self::OpenGl(driver), window);
        }
    }

    pub fn present(
        &mut self,
        _size: PhysicalSize<u32>,
        window: &Window,
        render: impl FnOnce(&mut SkiaSurface),
    ) {
        match self {
            #[cfg(any(target_os = "linux", target_os = "windows", target_os = "android"))]
            Self::OpenGl(gl) => gl.present(window, render),
            #[cfg(target_os = "macos")]
            Self::Metal(mtl) => mtl.present(_size, window, render),
            #[cfg(any(target_os = "linux", target_os = "windows"))]
            Self::Vulkan(vk) => vk.present(_size, window, render),
        }
    }

    /// The name of the active graphics driver.
    pub fn name(&self) -> &'static str {
        match self {
            #[cfg(any(target_os = "linux", target_os = "windows", target_os = "android"))]
            Self::OpenGl(_) => "OpenGL",
            #[cfg(target_os = "macos")]
            Self::Metal(_) => "Metal",
            #[cfg(any(target_os = "linux", target_os = "windows"))]
            Self::Vulkan(_) => "Vulkan",
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        match self {
            #[cfg(any(target_os = "linux", target_os = "windows", target_os = "android"))]
            Self::OpenGl(gl) => gl.resize(size),
            #[cfg(target_os = "macos")]
            Self::Metal(mtl) => mtl.resize(size),
            #[cfg(any(target_os = "linux", target_os = "windows"))]
            Self::Vulkan(vk) => vk.resize(size),
        }
    }
}
