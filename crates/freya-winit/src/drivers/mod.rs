#[cfg(feature = "opengl")]
mod gl;
#[cfg(all(feature = "metal", target_os = "macos"))]
mod metal;
#[cfg(feature = "vulkan")]
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

use crate::config::WindowConfig;

#[allow(clippy::large_enum_variant)]
pub enum GraphicsDriver {
    #[cfg(feature = "opengl")]
    OpenGl(gl::OpenGLDriver),
    #[cfg(all(feature = "metal", target_os = "macos"))]
    Metal(metal::MetalDriver),
    #[cfg(feature = "vulkan")]
    Vulkan(vulkan::VulkanDriver),
}

impl GraphicsDriver {
    #[allow(clippy::needless_return)]
    pub fn new(
        event_loop: &ActiveEventLoop,
        window_attributes: WindowAttributes,
        window_config: &WindowConfig,
    ) -> (Self, Window) {
        // macOS: Always use Metal
        #[cfg(target_os = "macos")]
        {
            #[cfg(feature = "metal")]
            {
                let (driver, window) =
                    metal::MetalDriver::new(event_loop, window_attributes, window_config);

                return (Self::Metal(driver), window);
            }

            #[cfg(not(feature = "metal"))]
            compile_error!(
                "The `metal` feature is required on macOS. Enable it in your Cargo.toml."
            );
        }

        // Linux/Windows: Use Vulkan by default with OpenGL as fallback.
        // Set FREYA_RENDERER=opengl to force OpenGL.
        #[cfg(not(target_os = "macos"))]
        {
            let force_opengl =
                std::env::var("FREYA_RENDERER").is_ok_and(|v| v.eq_ignore_ascii_case("opengl"));

            if !force_opengl {
                #[cfg(feature = "vulkan")]
                {
                    let vk_attrs = window_attributes.clone();
                    match vulkan::VulkanDriver::new(event_loop, vk_attrs, window_config) {
                        Ok((driver, window)) => return (Self::Vulkan(driver), window),
                        Err(err) => {
                            tracing::warn!(
                                "Vulkan initialization failed, falling back to OpenGL: {err}"
                            );
                        }
                    }
                }
            }

            #[cfg(feature = "opengl")]
            {
                let (driver, window) =
                    gl::OpenGLDriver::new(event_loop, window_attributes, window_config);

                return (Self::OpenGl(driver), window);
            }

            #[cfg(not(feature = "opengl"))]
            panic!(
                "Vulkan initialization failed and OpenGL feature is not enabled. Enable the `opengl` or `vulkan` feature."
            );
        }
    }

    pub fn present(
        &mut self,
        size: PhysicalSize<u32>,
        window: &Window,
        render: impl FnOnce(&mut SkiaSurface),
    ) {
        match self {
            #[cfg(feature = "opengl")]
            Self::OpenGl(gl) => gl.present(window, render),
            #[cfg(all(feature = "metal", target_os = "macos"))]
            Self::Metal(mtl) => mtl.present(size, window, render),
            #[cfg(feature = "vulkan")]
            Self::Vulkan(vk) => vk.present(size, window, render),
        }
    }

    /// The name of the active graphics driver.
    pub fn name(&self) -> &'static str {
        match self {
            #[cfg(feature = "opengl")]
            Self::OpenGl(_) => "OpenGL",
            #[cfg(all(feature = "metal", target_os = "macos"))]
            Self::Metal(_) => "Metal",
            #[cfg(feature = "vulkan")]
            Self::Vulkan(_) => "Vulkan",
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        match self {
            #[cfg(feature = "opengl")]
            Self::OpenGl(gl) => gl.resize(size),
            #[cfg(all(feature = "metal", target_os = "macos"))]
            Self::Metal(mtl) => mtl.resize(size),
            #[cfg(feature = "vulkan")]
            Self::Vulkan(vk) => vk.resize(size),
        }
    }
}
