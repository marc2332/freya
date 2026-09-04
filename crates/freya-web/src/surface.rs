use std::ffi::CString;

use freya_engine::prelude::{
    Canvas,
    ColorType,
    DirectContext,
    Format,
    FramebufferInfo,
    Interface,
    Surface,
    SurfaceOrigin,
    backend_render_targets,
    direct_contexts,
    wrap_backend_render_target,
};
use torin::geometry::Size2DFixed;

use crate::emscripten::*;

/// Skia surface backed by the WebGL context of the `#canvas` element.
pub struct WebSurface {
    context: DirectContext,
    framebuffer_info: FramebufferInfo,
    surface: Surface,
    size: Size2DFixed,
}

impl WebSurface {
    pub fn new(size: Size2DFixed) -> Option<Self> {
        if !Self::create_webgl_context() {
            tracing::error!("Failed to create the WebGL context of the #canvas element.");
            return None;
        }

        unsafe {
            gl::load_with(|symbol| {
                let Ok(symbol) = CString::new(symbol) else {
                    return std::ptr::null();
                };
                emscripten_GetProcAddress(symbol.as_ptr()) as *const _
            });
        }

        let interface = Interface::new_native()?;
        let mut context = direct_contexts::make_gl(interface, None)?;

        let mut framebuffer = 0;
        unsafe { gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut framebuffer) };

        let framebuffer_info = FramebufferInfo {
            fboid: framebuffer.try_into().ok()?,
            format: Format::RGBA8.into(),
            ..Default::default()
        };

        let surface = Self::create_surface(&mut context, framebuffer_info, size)?;

        Some(Self {
            context,
            framebuffer_info,
            surface,
            size,
        })
    }

    pub fn resize(&mut self, size: Size2DFixed) {
        if self.size == size {
            return;
        }

        if let Some(surface) = Self::create_surface(&mut self.context, self.framebuffer_info, size) {
            self.surface = surface;
            self.size = size;
        }
    }

    pub fn canvas(&mut self) -> &Canvas {
        self.surface.canvas()
    }

    /// Submits the recorded commands, the browser presents the buffer itself.
    pub fn present(&mut self) {
        self.context.flush_and_submit();
    }

    fn create_webgl_context() -> bool {
        let mut attributes = EmscriptenWebGLContextAttributes::default();
        unsafe { emscripten_webgl_init_context_attributes(&mut attributes) };

        attributes.alpha = false;
        attributes.stencil = true;
        attributes.major_version = 2;
        attributes.antialias = false;
        attributes.depth = false;

        let context = unsafe { emscripten_webgl_create_context(TARGET_CANVAS, &attributes) };
        if context <= 0 {
            return false;
        }

        unsafe { emscripten_webgl_make_context_current(context) == EMSCRIPTEN_RESULT_SUCCESS }
    }

    fn create_surface(
        context: &mut DirectContext,
        framebuffer_info: FramebufferInfo,
        size: Size2DFixed,
    ) -> Option<Surface> {
        let backend_render_target = backend_render_targets::make_gl(
            (size.width.max(1), size.height.max(1)),
            1,
            8,
            framebuffer_info,
        );

        wrap_backend_render_target(
            context,
            &backend_render_target,
            SurfaceOrigin::BottomLeft,
            ColorType::RGBA8888,
            None,
            None,
        )
    }
}
