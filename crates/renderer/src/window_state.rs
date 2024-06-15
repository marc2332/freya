use std::{
    ffi::CString,
    mem,
    num::NonZeroU32,
};

use dioxus_core::VirtualDom;
use freya_common::EventMessage;
use freya_core::dom::SafeDOM;
use freya_engine::prelude::*;
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
    surface::{
        GlSurface,
        Surface as GlutinSurface,
        SurfaceAttributesBuilder,
        SwapInterval,
        WindowSurface,
    },
};
use glutin_winit::DisplayBuilder;
use winit::{
    dpi::LogicalSize,
    event_loop::{
        ActiveEventLoop,
        EventLoopProxy,
    },
    raw_window_handle::HasWindowHandle,
    window::Window,
};

use crate::{
    app::Application,
    config::WindowConfig,
    devtools::Devtools,
    LaunchConfig,
};

pub struct NotCreatedState<'a, State: Clone + 'static> {
    pub(crate) sdom: SafeDOM,
    pub(crate) vdom: VirtualDom,
    pub(crate) devtools: Option<Devtools>,
    pub(crate) config: LaunchConfig<'a, State>,
}

pub struct CreatedState {
    pub(crate) gr_context: DirectContext,
    pub(crate) surface: Surface,
    pub(crate) gl_surface: GlutinSurface<WindowSurface>,
    pub(crate) gl_context: PossiblyCurrentContext,
    pub(crate) window: Window,
    pub(crate) window_config: WindowConfig,
    pub(crate) fb_info: FramebufferInfo,
    pub(crate) num_samples: usize,
    pub(crate) stencil_size: usize,
    pub(crate) app: Application,
    pub(crate) is_window_focused: bool,
}

pub enum WindowState<'a, State: Clone + 'static> {
    NotCreated(NotCreatedState<'a, State>),
    Creating,
    Created(CreatedState),
}

impl<'a, State: Clone + 'a> WindowState<'a, State> {
    pub fn created_state(&mut self) -> &mut CreatedState {
        let Self::Created(created) = self else {
            panic!("Unexpected.")
        };
        created
    }

    pub fn has_been_created(&self) -> bool {
        matches!(self, Self::Created(..))
    }

    pub fn create(
        &mut self,
        event_loop: &ActiveEventLoop,
        event_loop_proxy: &EventLoopProxy<EventMessage>,
    ) {
        let Self::NotCreated(NotCreatedState {
            sdom,
            vdom,
            devtools,
            mut config,
        }) = mem::replace(self, WindowState::Creating)
        else {
            panic!("Unexpected.")
        };

        let mut window_attributes = Window::default_attributes()
            .with_visible(false)
            .with_title(config.window_config.title)
            .with_decorations(config.window_config.decorations)
            .with_transparent(config.window_config.transparent)
            .with_window_icon(config.window_config.icon.take())
            .with_inner_size(LogicalSize::<f64>::new(
                config.window_config.width,
                config.window_config.height,
            ));

        set_resource_cache_total_bytes_limit(1000000); // 1MB
        set_resource_cache_single_allocation_byte_limit(Some(500000)); // 0.5MB

        if let Some(min_size) = config
            .window_config
            .min_width
            .zip(config.window_config.min_height)
        {
            window_attributes =
                window_attributes.with_min_inner_size(LogicalSize::<f64>::from(min_size))
        }

        if let Some(max_size) = config
            .window_config
            .max_width
            .zip(config.window_config.max_height)
        {
            window_attributes =
                window_attributes.with_max_inner_size(LogicalSize::<f64>::from(max_size))
        }

        if let Some(with_window_attributes) = &config.window_config.window_attributes_hook {
            window_attributes = (with_window_attributes)(window_attributes);
        }

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

        let mut window = window.expect("Could not create window with OpenGL context");

        // Allow IME
        window.set_ime_allowed(true);

        // Mak the window visible once built
        window.set_visible(true);

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

        let scale_factor = window.scale_factor() as f32;
        surface.canvas().scale((scale_factor, scale_factor));

        let mut app = Application::new(
            sdom,
            vdom,
            event_loop_proxy,
            devtools,
            &window,
            config.embedded_fonts,
            config.plugins,
            config.default_fonts,
        );

        app.init_doms(scale_factor, config.state.clone());
        app.process_layout(window.inner_size(), scale_factor);

        *self = WindowState::Created(CreatedState {
            gr_context,
            surface,
            gl_surface,
            gl_context,
            window,
            fb_info,
            num_samples,
            stencil_size,
            app,
            window_config: config.window_config,
            is_window_focused: false,
        });
    }
}

/// Create the surface for Skia to render in
pub fn create_surface(
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
