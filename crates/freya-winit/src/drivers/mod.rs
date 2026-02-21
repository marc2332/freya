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
    OpenGl(gl::OpenGLDriver),
    #[cfg(all(feature = "metal", target_os = "macos"))]
    Metal(metal::MetalDriver),
    #[cfg(feature = "vulkan")]
    Vulkan(vulkan::VulkanDriver),
}

impl GraphicsDriver {
    pub fn new(
        event_loop: &ActiveEventLoop,
        window_attributes: WindowAttributes,
        window_config: &WindowConfig,
    ) -> (Self, Window) {
        // Metal takes priority on macOS (native, best performance)
        #[cfg(all(feature = "metal", target_os = "macos"))]
        {
            let (driver, window) =
                metal::MetalDriver::new(event_loop, window_attributes, window_config);

            return (Self::Metal(driver), window);
        }

        #[cfg(feature = "vulkan")]
        #[allow(unreachable_code)]
        {
            let (driver, window) =
                vulkan::VulkanDriver::new(event_loop, window_attributes, window_config);

            return (Self::Vulkan(driver), window);
        }

        #[allow(unreachable_code)]
        let (driver, window) = gl::OpenGLDriver::new(event_loop, window_attributes, window_config);

        (Self::OpenGl(driver), window)
    }

    #[allow(unused)]
    pub fn present(
        &mut self,
        size: PhysicalSize<u32>,
        window: &Window,
        render: impl FnOnce(&mut SkiaSurface),
    ) {
        match self {
            Self::OpenGl(gl) => gl.present(window, render),
            #[cfg(all(feature = "metal", target_os = "macos"))]
            Self::Metal(mtl) => mtl.present(size, window, render),
            #[cfg(feature = "vulkan")]
            Self::Vulkan(vk) => vk.present(size, window, render),
            _ => unimplemented!("Enable `gl` or `vulkan` features."),
        }
    }

    #[allow(unused)]
    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        match self {
            Self::OpenGl(gl) => gl.resize(size),
            #[cfg(all(feature = "metal", target_os = "macos"))]
            Self::Metal(mtl) => mtl.resize(size),
            #[cfg(feature = "vulkan")]
            Self::Vulkan(vk) => vk.resize(size),
            _ => unimplemented!("Enable `gl` or `vulkan` features."),
        }
    }
}
