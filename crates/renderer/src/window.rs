use freya_common::EventMessage;
use freya_engine::prelude::*;
use gl::{types::*, *};
use glutin::context::GlProfile;
use glutin::context::NotCurrentGlContext;
use glutin::prelude::PossiblyCurrentGlContext;
use glutin::{
    config::{ConfigTemplateBuilder, GlConfig},
    context::{ContextApi, ContextAttributesBuilder, PossiblyCurrentContext},
    display::{GetGlDisplay, GlDisplay},
    prelude::GlSurface,
    surface::{Surface as GlutinSurface, SurfaceAttributesBuilder, WindowSurface},
};
use glutin_winit::DisplayBuilder;
use raw_window_handle::HasRawWindowHandle;
use std::ffi::CString;
use std::num::NonZeroU32;

use winit::dpi::{LogicalSize, PhysicalSize};
use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use crate::config::WindowConfig;

/// Manager for a Window
pub struct WindowEnv<State: Clone> {
    pub(crate) gr_context: DirectContext,
    pub(crate) surface: Surface,
    pub(crate) gl_surface: GlutinSurface<WindowSurface>,
    pub(crate) gl_context: PossiblyCurrentContext,
    pub(crate) window: Window,
    pub(crate) fb_info: FramebufferInfo,
    pub(crate) num_samples: usize,
    pub(crate) stencil_size: usize,
    pub(crate) window_config: WindowConfig<State>,
}

impl<T: Clone> Drop for WindowEnv<T> {
    fn drop(&mut self) {
        if !self.gl_context.is_current() && self.gl_context.make_current(&self.gl_surface).is_err()
        {
            self.gr_context.abandon();
        }
    }
}

impl<T: Clone> WindowEnv<T> {
    /// Setup the Window and related features
    pub fn new(mut window_config: WindowConfig<T>, event_loop: &EventLoop<EventMessage>) -> Self {
        let mut window_builder = WindowBuilder::new()
            .with_visible(false)
            .with_title(window_config.title)
            .with_decorations(window_config.decorations)
            .with_transparent(window_config.transparent)
            .with_window_icon(window_config.icon.take())
            .with_inner_size(LogicalSize::<f64>::new(
                window_config.width,
                window_config.height,
            ));

        set_resource_cache_total_bytes_limit(1000000); // 1MB
        set_resource_cache_single_allocation_byte_limit(Some(500000)); // 0.5MB

        if let Some(min_size) = window_config.min_width.zip(window_config.min_height) {
            window_builder = window_builder.with_min_inner_size(LogicalSize::<f64>::from(min_size))
        }

        if let Some(max_size) = window_config.max_width.zip(window_config.max_height) {
            window_builder = window_builder.with_max_inner_size(LogicalSize::<f64>::from(max_size))
        }

        if let Some(with_window_builder) = &window_config.window_builder_hook {
            window_builder = (with_window_builder)(window_builder);
        }

        let template = ConfigTemplateBuilder::new()
            .with_alpha_size(8)
            .with_transparency(window_config.transparent);

        let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));
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

        let mut window = window.expect("Could not create window with OpenGL context");
        window.set_ime_allowed(true);
        let raw_window_handle = window.raw_window_handle();

        let context_attributes = ContextAttributesBuilder::new()
            .with_profile(GlProfile::Core)
            .build(Some(raw_window_handle));

        let fallback_context_attributes = ContextAttributesBuilder::new()
            .with_profile(GlProfile::Core)
            .with_context_api(ContextApi::Gles(None))
            .build(Some(raw_window_handle));

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
            raw_window_handle,
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
            DirectContext::new_gl(interface, None).expect("Could not create direct context");

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

        let mut surface = create_surface(
            &mut window,
            fb_info,
            &mut gr_context,
            num_samples,
            stencil_size,
        );

        let sf = window.scale_factor() as f32;
        surface.canvas().scale((sf, sf));

        WindowEnv {
            surface,
            gl_surface,
            gl_context,
            gr_context,
            fb_info,
            num_samples,
            stencil_size,
            window,
            window_config,
        }
    }

    /// Get a reference to the Canvas.
    pub fn canvas(&mut self) -> &Canvas {
        self.surface.canvas()
    }

    /// Clear the canvas.
    pub fn clear(&mut self) {
        let canvas = self.surface.canvas();
        canvas.clear(self.window_config.background);
    }

    /// Flush and submit the canvas.
    pub fn finish_render(&mut self) {
        self.window.pre_present_notify();
        self.gr_context.flush_and_submit();
        self.gl_surface.swap_buffers(&self.gl_context).unwrap();
    }

    /// Resize the Window
    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.surface = create_surface(
            &mut self.window,
            self.fb_info,
            &mut self.gr_context,
            self.num_samples,
            self.stencil_size,
        );

        let (width, height): (u32, u32) = size.into();

        self.gl_surface.resize(
            &self.gl_context,
            NonZeroU32::new(width.max(1)).unwrap(),
            NonZeroU32::new(height.max(1)).unwrap(),
        );

        self.window.request_redraw();
    }

    /// Run the `on_setup` callback that was passed to the launch function
    pub fn run_on_setup(&mut self) {
        let on_setup = self.window_config.on_setup.clone();
        if let Some(on_setup) = on_setup {
            (on_setup)(&mut self.window)
        }
    }

    /// Run the `on_exit` callback that was passed to the launch function
    pub fn run_on_exit(&mut self) {
        let on_exit = self.window_config.on_exit.clone();
        if let Some(on_exit) = on_exit {
            (on_exit)(&mut self.window)
        }
    }
}

/// Create the surface for Skia to render in
fn create_surface(
    window: &mut Window,
    fb_info: FramebufferInfo,
    gr_context: &mut DirectContext,
    num_samples: usize,
    stencil_size: usize,
) -> Surface {
    let size = window.inner_size();
    let size = (
        size.width.try_into().expect("Could not convert width"),
        size.height.try_into().expect("Could not convert height"),
    );
    let backend_render_target =
        backend_render_targets::make_gl(size, num_samples, stencil_size, fb_info);
    wrap_backend_render_target(
        gr_context,
        &backend_render_target,
        SurfaceOrigin::BottomLeft,
        ColorType::RGBA8888,
        None,
        None,
    )
    .expect("Could not create skia surface")
}
