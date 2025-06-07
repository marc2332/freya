#[cfg(feature = "gl")]
mod gl;
#[cfg(feature = "vulkan")]
mod vulkan;

use freya_engine::prelude::Surface as SkiaSurface;
use tracing::info;
use winit::{
    dpi::PhysicalSize,
    event_loop::ActiveEventLoop,
    window::{
        Window,
        WindowAttributes,
    },
};

use crate::LaunchConfig;

pub enum GraphicsDriver {
    #[cfg(feature = "gl")]
    #[allow(dead_code)]
    OpenGl(gl::OpenGLDriver),
    #[cfg(feature = "vulkan")]
    Vulkan(vulkan::VulkanDriver),
}

impl GraphicsDriver {
    pub fn new<State: Clone + 'static>(
        event_loop: &ActiveEventLoop,
        window_attributes: WindowAttributes,
        config: &LaunchConfig<State>,
    ) -> (Self, Window) {
        #[cfg(feature = "vulkan")]
        {
            let (driver, window) = vulkan::VulkanDriver::new(event_loop, window_attributes, config);
            info!("Using vulkan.");

            return (Self::Vulkan(driver), window);
        }

        #[cfg(feature = "gl")]
        #[allow(unreachable_code, clippy::needless_return)]
        {
            let (driver, window) = gl::OpenGLDriver::new(event_loop, window_attributes, config);
            info!("Using OpenGL.");

            return (Self::OpenGl(driver), window);
        }

        #[cfg(not(all(feature = "vulkan", feature = "gl")))]
        #[allow(unreachable_code)]
        {
            unimplemented!("Enable `gl` or `vulkan` features.")
        }
    }

    #[allow(unused)]
    pub fn present(
        &mut self,
        size: PhysicalSize<u32>,
        window: &Window,
        render: impl FnOnce(&mut SkiaSurface, &mut SkiaSurface),
    ) {
        match self {
            #[cfg(feature = "gl")]
            Self::OpenGl(gl) => gl.present(render),
            #[cfg(feature = "vulkan")]
            Self::Vulkan(vk) => vk.present(size, window, render),
            #[cfg(not(all(feature = "vulkan", feature = "gl")))]
            _ => unimplemented!("Enable `gl` or `vulkan` features."),
        }
    }

    #[allow(unused)]
    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        match self {
            #[cfg(feature = "gl")]
            Self::OpenGl(gl) => gl.resize(size),
            #[cfg(feature = "vulkan")]
            Self::Vulkan(vk) => vk.resize(),
            #[cfg(not(all(feature = "vulkan", feature = "gl")))]
            _ => unimplemented!("Enable `gl` or `vulkan` features."),
        }
    }
}
