use gl::{types::*, *};
use glutin::config::Config;
use glutin::{
    config::GlConfig,
    context::{
        ContextApi, ContextAttributesBuilder, GlProfile, NotCurrentGlContext,
        PossiblyCurrentContext,
    },
    display::{GetGlDisplay, GlDisplay},
    surface::{
        GlSurface, Surface as GlutinSurface, SurfaceAttributesBuilder, SwapInterval, WindowSurface,
    },
};
use std::{ffi::CString, num::NonZeroU32};
use winit::{raw_window_handle::HasWindowHandle, window::Window};
use winit::dpi::PhysicalSize;

use freya_engine::prelude::*;

pub struct SkiaSurface {
    pub(crate) gr_context: DirectContext,
    pub(crate) surface: Surface,
    pub(crate) gl_surface: GlutinSurface<WindowSurface>,
    pub(crate) gl_context: PossiblyCurrentContext,
    pub(crate) fb_info: FramebufferInfo,
    pub(crate) num_samples: usize,
    pub(crate) stencil_size: usize,
    pub(crate) scale_factor: f64,
}

impl SkiaSurface {
    pub fn create(window: &Window, gl_config: Config) -> Self {
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

        let mut gr_context =
            direct_contexts::make_gl(interface, None).expect("Could not create direct context");

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

        let mut surface = {
            let size = window.inner_size();
            let size = (
                size.width.try_into().expect("Could not convert width"),
                size.height.try_into().expect("Could not convert height"),
            );

            let backend_render_target =
                backend_render_targets::make_gl(size, num_samples, stencil_size, fb_info);

            wrap_backend_render_target(
                &mut gr_context,
                &backend_render_target,
                SurfaceOrigin::BottomLeft,
                ColorType::RGBA8888,
                None,
                None,
            ).expect("Could not create skia surface")
        };

        let scale_factor = window.scale_factor();

        surface
            .canvas()
            .scale((scale_factor as f32, scale_factor as f32));

        Self {
            gr_context,
            surface,
            gl_surface,
            gl_context,
            fb_info,
            num_samples,
            stencil_size,
            scale_factor
        }
    }

    pub fn resize(&mut self, window: &mut Window, new_size: PhysicalSize<u32>) {
        let size = window.inner_size();
        let size = (
            size.width.try_into().expect("Could not convert width"),
            size.height.try_into().expect("Could not convert height"),
        );

        let backend_render_target =
            backend_render_targets::make_gl(size, self.num_samples, self.stencil_size, self.fb_info);

        wrap_backend_render_target(
            &mut self.gr_context,
            &backend_render_target,
            SurfaceOrigin::BottomLeft,
            ColorType::RGBA8888,
            None,
            None,
        ).expect("Could not create skia surface");

        self.gl_surface.resize(
            &self.gl_context,
            NonZeroU32::new(new_size.width.max(1)).unwrap(),
            NonZeroU32::new(new_size.height.max(1)).unwrap(),
        );
    }
}
