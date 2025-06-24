mod gl;

use freya_engine::prelude::Surface as SkiaSurface;
pub use gl::*;
use glutin::surface::GlSurface;
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
        }
    }

    pub fn flush_and_submit(&mut self) {
        match self {
            Self::OpenGl(gl) => {
                gl.gr_context.flush_and_submit();
                gl.gl_surface.swap_buffers(&gl.gl_context).unwrap();
            }
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) -> (SkiaSurface, SkiaSurface) {
        match self {
            Self::OpenGl(gl) => gl.resize(size),
        }
    }
}
