use std::{
    ffi::CString,
    num::NonZeroU32,
};

use freya_engine::prelude::{
    backend_render_targets,
    direct_contexts,
    wrap_backend_render_target,
    ColorType,
    DirectContext,
    Format,
    FramebufferInfo,
    Interface,
    Surface as SkiaSurface,
    SurfaceOrigin,
};
use gl::{
    types::*,
    *,
};
use glutin::{
    config::{
        ConfigTemplateBuilder,
        GlConfig,
    },
    context::{
        ContextApi,
        ContextAttributesBuilder,
        GlProfile,
        NotCurrentGlContext,
        PossiblyCurrentContext,
    },
    display::{
        GetGlDisplay,
        GlDisplay,
    },
    prelude::{
        GlSurface,
        PossiblyCurrentGlContext,
    },
    surface::{
        Surface as GlutinSurface,
        SurfaceAttributesBuilder,
        SwapInterval,
        WindowSurface,
    },
};
use glutin_winit::DisplayBuilder;
use raw_window_handle::HasWindowHandle;
use winit::{
    dpi::PhysicalSize,
    event_loop::ActiveEventLoop,
    window::{
        Window,
        WindowAttributes,
    },
};

use crate::{
    size::WinitSize,
    LaunchConfig,
};

/// Graphics driver using OpenGL.
pub struct OpenGLDriver {
    pub(crate) gr_context: DirectContext,
    pub(crate) gl_surface: GlutinSurface<WindowSurface>,
    pub(crate) gl_context: PossiblyCurrentContext,
    pub(crate) fb_info: FramebufferInfo,
    pub(crate) num_samples: usize,
    pub(crate) stencil_size: usize,
}

impl Drop for OpenGLDriver {
    fn drop(&mut self) {
        if !self.gl_context.is_current() && self.gl_context.make_current(&self.gl_surface).is_err()
        {
            self.gr_context.abandon();
        }
    }
}

impl OpenGLDriver {
    pub fn new<State: Clone + 'static>(
        event_loop: &ActiveEventLoop,
        window_attributes: WindowAttributes,
        config: &LaunchConfig<State>,
    ) -> (Self, Window, SkiaSurface) {
        let template = ConfigTemplateBuilder::new()
            .with_alpha_size(8)
            .with_transparency(config.window_config.transparent);

        let display_builder = DisplayBuilder::new().with_window_attributes(Some(window_attributes));
        let (window, gl_config) = display_builder
            .build(event_loop, template, |configs| {
                configs
                    .reduce(|accum, config| {
                        let transparency_check = config.supports_transparency().unwrap_or(false)
                            & !accum.supports_transparency().unwrap_or(false);

                        if transparency_check || config.num_samples() < accum.num_samples() {
                            config
                        } else {
                            accum
                        }
                    })
                    .unwrap()
            })
            .unwrap();

        let window = window.expect("Could not create window with OpenGL context");

        let window_handle = window.window_handle().unwrap();

        let context_attributes = ContextAttributesBuilder::new()
            .with_profile(GlProfile::Core)
            .build(Some(window_handle.as_raw()));

        let fallback_context_attributes = ContextAttributesBuilder::new()
            .with_profile(GlProfile::Core)
            .with_context_api(ContextApi::Gles(None))
            .build(Some(window_handle.as_raw()));

        let not_current_gl_context = unsafe {
            gl_config
                .display()
                .create_context(&gl_config, &context_attributes)
                .unwrap_or_else(|_| {
                    gl_config
                        .display()
                        .create_context(&gl_config, &fallback_context_attributes)
                        .expect("failed to create context")
                })
        };

        let (width, height): (u32, u32) = window.inner_size().into();

        let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            window_handle.as_raw(),
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
        );

        let gl_surface = unsafe {
            gl_config
                .display()
                .create_window_surface(&gl_config, &attrs)
                .expect("Could not create gl window surface")
        };

        let gl_context = not_current_gl_context
            .make_current(&gl_surface)
            .expect("Could not make GL context current when setting up skia renderer");

        // Try setting vsync.
        gl_surface
            .set_swap_interval(&gl_context, SwapInterval::Wait(NonZeroU32::new(1).unwrap()))
            .ok();

        load_with(|s| {
            gl_config
                .display()
                .get_proc_address(CString::new(s).unwrap().as_c_str())
        });
        let interface = Interface::new_load_with(|name| {
            if name == "eglGetCurrentDisplay" {
                return std::ptr::null();
            }
            gl_config
                .display()
                .get_proc_address(CString::new(name).unwrap().as_c_str())
        })
        .expect("Could not create interface");

        let fb_info = {
            let mut fboid: GLint = 0;
            unsafe { GetIntegerv(FRAMEBUFFER_BINDING, &mut fboid) };

            FramebufferInfo {
                fboid: fboid.try_into().unwrap(),
                format: Format::RGBA8.into(),
                ..Default::default()
            }
        };

        let num_samples = gl_config.num_samples() as usize;
        let stencil_size = gl_config.stencil_size() as usize;
        let size = window.inner_size();

        let mut gr_context =
            direct_contexts::make_gl(interface, None).expect("Could not create direct context");

        let render_target =
            backend_render_targets::make_gl(size.to_skia(), num_samples, stencil_size, fb_info);
        let skia_surface = wrap_backend_render_target(
            &mut gr_context,
            &render_target,
            SurfaceOrigin::BottomLeft,
            ColorType::RGBA8888,
            None,
            None,
        )
        .expect("Could not create skia surface");

        let driver = OpenGLDriver {
            gl_context,
            gl_surface,
            gr_context,
            num_samples,
            stencil_size,
            fb_info,
        };

        (driver, window, skia_surface)
    }

    pub fn make_current(&mut self) {
        self.gl_context.make_current(&self.gl_surface).unwrap();
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) -> (SkiaSurface, SkiaSurface) {
        let render_target = backend_render_targets::make_gl(
            size.to_skia(),
            self.num_samples,
            self.stencil_size,
            self.fb_info,
        );
        let mut surface = wrap_backend_render_target(
            &mut self.gr_context,
            &render_target,
            SurfaceOrigin::BottomLeft,
            ColorType::RGBA8888,
            None,
            None,
        )
        .expect("Could not create skia surface");

        let dirty_surface = surface.new_surface_with_dimensions(size.to_skia()).unwrap();

        self.gl_surface
            .resize(&self.gl_context, size.as_gl_width(), size.as_gl_height());

        (surface, dirty_surface)
    }
}
