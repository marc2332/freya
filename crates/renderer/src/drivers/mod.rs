mod gl;
mod metal;

pub use gl::*;
pub use metal::*;

use freya_engine::prelude::Surface as SkiaSurface;
use glutin::surface::GlSurface;
use metal::MetalDriver;
use winit::{
    dpi::PhysicalSize,
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes},
};

use crate::LaunchConfig;

pub enum GraphicsDriver {
    OpenGl(OpenGLDriver),
    Metal(MetalDriver),
}

impl GraphicsDriver {
    pub fn new<State: Clone + 'static>(
        event_loop: &ActiveEventLoop,
        window_attributes: WindowAttributes,
        config: &LaunchConfig<State>,
    ) -> (Self, Window, SkiaSurface) {
        let (driver, window, surface) = OpenGLDriver::new(event_loop, window_attributes, config);
        (Self::OpenGl(driver), window, surface)
    }

    pub fn make_current(&mut self) {
        match self {
            Self::OpenGl(gl) => gl.make_current(),
            Self::Metal(_) => {}
        }
    }

    pub fn flush_and_submit(&mut self) {
        match self {
            Self::OpenGl(gl) => {
                gl.gr_context.flush_and_submit();
                gl.gl_surface.swap_buffers(&gl.gl_context).unwrap();
            }
            Self::Metal(mtl) => {
                if let Some(drawable) = mtl.metal_layer.next_drawable() {
                    mtl.gr_context.flush_and_submit();

                    let command_buffer = mtl.command_queue.new_command_buffer();
                    command_buffer.present_drawable(drawable);
                    command_buffer.commit();
                }
            }
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) -> (SkiaSurface, SkiaSurface) {
        match self {
            Self::OpenGl(gl) => gl.resize(size),
            Self::Metal(mtl) => mtl.resize(size),
        }
    }
}
