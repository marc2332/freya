mod gl;
mod vulkan;

use freya_engine::prelude::Surface as SkiaSurface;
pub use gl::*;
use glutin::surface::GlSurface;
use vulkan::VulkanDriver;
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
    OpenGl(OpenGLDriver),
    Vulkan(VulkanDriver),
}

impl GraphicsDriver {
    pub fn new<State: Clone + 'static>(
        event_loop: &ActiveEventLoop,
        window_attributes: WindowAttributes,
        config: &LaunchConfig<State>,
    ) -> (Self, Window, SkiaSurface) {
        let (driver, window, surface) = VulkanDriver::new(event_loop, window_attributes, config);

        (Self::Vulkan(driver), window, surface)
    }

    pub fn make_current(&mut self) {
        match self {
            Self::OpenGl(gl) => gl.make_current(),
            Self::Vulkan(_) => {}
        }
    }

    pub fn flush_and_submit(&mut self) {
        match self {
            Self::OpenGl(gl) => {
                gl.gr_context.flush_and_submit();
                gl.gl_surface.swap_buffers(&gl.gl_context).unwrap();
            }
            Self::Vulkan(vk) => {
                vk.gr_context.flush_and_submit();
            }
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) -> (SkiaSurface, SkiaSurface) {
        match self {
            Self::OpenGl(gl) => gl.resize(size),
            Self::Vulkan(vk) => vk.resize(size),
        }
    }
}
