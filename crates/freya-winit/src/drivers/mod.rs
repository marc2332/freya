#[cfg(any(target_os = "linux", target_os = "windows"))]
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
    #[cfg(any(target_os = "linux", target_os = "windows"))]
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
    ) -> (Self, Window) {
        // Metal (macOS)
        #[cfg(target_os = "macos")]
        {
            let (driver, window) =
                metal::MetalDriver::new(event_loop, window_attributes, window_config);

            return (Self::Metal(driver), window);
        }

        // Vulkan by default with OpenGL as fallback.
        // Set FREYA_RENDERER=opengl to force OpenGL.
        #[cfg(not(target_os = "macos"))]
        {
            let force_opengl =
                std::env::var("FREYA_RENDERER").is_ok_and(|v| v.eq_ignore_ascii_case("opengl"));

            if !force_opengl {
                let vk_attrs = window_attributes.clone();
                match vulkan::VulkanDriver::new(event_loop, vk_attrs) {
                    Ok((driver, window)) => return (Self::Vulkan(driver), window),
                    Err(err) => {
                        tracing::warn!(
                            "Vulkan initialization failed, falling back to OpenGL: {err}"
                        );
                    }
                }
            }

            let (driver, window) = gl::OpenGLDriver::new(event_loop, window_attributes);

            return (Self::OpenGl(driver), window);
        }
    }

    pub fn present(
        &mut self,
        size: PhysicalSize<u32>,
        window: &Window,
        render: impl FnOnce(&mut SkiaSurface),
    ) {
        match self {
            #[cfg(any(target_os = "linux", target_os = "windows"))]
            Self::OpenGl(gl) => gl.present(window, render),
            #[cfg(target_os = "macos")]
            Self::Metal(mtl) => mtl.present(size, window, render),
            #[cfg(any(target_os = "linux", target_os = "windows"))]
            Self::Vulkan(vk) => vk.present(size, window, render),
        }
    }

    /// The name of the active graphics driver.
    pub fn name(&self) -> &'static str {
        match self {
            #[cfg(any(target_os = "linux", target_os = "windows"))]
            Self::OpenGl(_) => "OpenGL",
            #[cfg(target_os = "macos")]
            Self::Metal(_) => "Metal",
            #[cfg(any(target_os = "linux", target_os = "windows"))]
            Self::Vulkan(_) => "Vulkan",
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        match self {
            #[cfg(any(target_os = "linux", target_os = "windows"))]
            Self::OpenGl(gl) => gl.resize(size),
            #[cfg(target_os = "macos")]
            Self::Metal(mtl) => mtl.resize(size),
            #[cfg(any(target_os = "linux", target_os = "windows"))]
            Self::Vulkan(vk) => vk.resize(size),
        }
    }
}
