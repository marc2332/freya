#[cfg(target_os = "windows")]
use std::time::Instant;
use std::{
    ffi::{
        CStr,
        CString,
    },
    num::NonZeroU32,
};

use freya_engine::prelude::{
    ColorType,
    DirectContext,
    Format,
    FramebufferInfo,
    Interface,
    Surface as SkiaSurface,
    SurfaceOrigin,
    backend_render_targets,
    direct_contexts,
};
use gl::{
    types::*,
    *,
};
use glutin::{
    config::{
        Config,
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
        Display,
        DisplayApiPreference,
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
use raw_window_handle::{
    HasDisplayHandle,
    HasWindowHandle,
};
use winit::{
    dpi::PhysicalSize,
    event_loop::ActiveEventLoop,
    window::{
        Window,
        WindowAttributes,
    },
};

use crate::drivers::surface::wrap_render_target;

/// Graphics driver using OpenGL.
pub struct OpenGLDriver {
    pub(crate) gr_context: DirectContext,
    pub(crate) gl_surface: GlutinSurface<WindowSurface>,
    pub(crate) gl_context: PossiblyCurrentContext,
    pub(crate) fb_info: FramebufferInfo,
    pub(crate) num_samples: usize,
    pub(crate) stencil_size: usize,
    pub(crate) surface: SkiaSurface,
    pub(crate) gpu_name: Option<String>,
    #[cfg(target_os = "windows")]
    last_present: Option<Instant>,
}

impl Drop for OpenGLDriver {
    fn drop(&mut self) {
        if !self.gl_context.is_current() && self.gl_context.make_current(&self.gl_surface).is_err()
        {
            self.gr_context.abandon();
            return;
        }

        self.gr_context.release_resources_and_abandon();
    }
}

impl OpenGLDriver {
    pub fn resource_cache_usage(&self) -> (usize, usize) {
        let usage = self.gr_context.resource_cache_usage();
        (usage.resource_bytes, self.gr_context.resource_cache_limit())
    }

    pub fn new(
        event_loop: &dyn ActiveEventLoop,
        window_attributes: WindowAttributes,
        gpu_resource_cache_limit: usize,
    ) -> Result<(Self, Box<dyn Window>), Box<dyn std::error::Error>> {
        let transparent = window_attributes.transparent();
        #[cfg(target_os = "windows")]
        {
            let window = event_loop.create_window(window_attributes)?;
            let driver = Self::from_window(
                event_loop,
                window.as_ref(),
                gpu_resource_cache_limit,
                transparent,
            )?;
            Ok((driver, window))
        }
        #[cfg(not(target_os = "windows"))]
        {
            let gl_config = Self::create_config(event_loop, None, transparent)?;
            #[cfg(target_os = "linux")]
            let window_attributes = {
                use glutin::platform::x11::X11GlConfigExt;
                use winit::platform::x11::WindowAttributesX11;
                if let Some(visual) = gl_config.x11_visual() {
                    let attributes = window_attributes
                        .platform
                        .as_ref()
                        .and_then(|attributes| attributes.cast_ref::<WindowAttributesX11>())
                        .cloned()
                        .unwrap_or_default()
                        .with_x11_visual(visual.visual_id() as _);
                    window_attributes.with_platform_attributes(Box::new(attributes))
                } else {
                    window_attributes
                }
            };
            let window = event_loop.create_window(window_attributes)?;
            let driver = Self::build(&gl_config, window.as_ref(), gpu_resource_cache_limit)?;
            Ok((driver, window))
        }
    }

    /// Build the driver on an existing window instead of creating a new one.
    pub fn from_window(
        event_loop: &dyn ActiveEventLoop,
        window: &dyn Window,
        gpu_resource_cache_limit: usize,
        transparent: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let gl_config = Self::create_config(event_loop, Some(window), transparent)?;
        Self::build(&gl_config, window, gpu_resource_cache_limit)
    }

    fn create_config(
        event_loop: &dyn ActiveEventLoop,
        window: Option<&dyn Window>,
        transparent: bool,
    ) -> Result<Config, Box<dyn std::error::Error>> {
        let window_handle = window
            .map(|window| window.window_handle().map(|handle| handle.as_raw()))
            .transpose()?;
        #[cfg(target_os = "linux")]
        let preference = DisplayApiPreference::EglThenGlx(Box::new(
            winit::platform::x11::register_xlib_error_hook,
        ));
        #[cfg(target_os = "windows")]
        let preference = DisplayApiPreference::WglThenEgl(window_handle);
        #[cfg(target_os = "android")]
        let preference = DisplayApiPreference::Egl;
        let display = unsafe { Display::new(event_loop.display_handle()?.as_raw(), preference)? };
        let mut template = Self::config_template(transparent);
        if let Some(window_handle) = window_handle {
            template = template.compatible_with_native_window(window_handle);
        }
        let configs = unsafe { display.find_configs(template.build())? };
        Ok(Self::pick_config(configs, transparent))
    }

    fn build(
        gl_config: &Config,
        window: &dyn Window,
        gpu_resource_cache_limit: usize,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let window_handle = window.window_handle()?.as_raw();

        let context_attributes = [
            ContextAttributesBuilder::new()
                .with_profile(GlProfile::Core)
                .build(Some(window_handle)),
            ContextAttributesBuilder::new()
                .with_profile(GlProfile::Compatibility)
                .build(Some(window_handle)),
            ContextAttributesBuilder::new()
                .with_context_api(ContextApi::Gles(None))
                .build(Some(window_handle)),
        ];

        let not_current_gl_context = context_attributes
            .iter()
            .find_map(|attributes| {
                unsafe { gl_config.display().create_context(gl_config, attributes) }
                    .inspect_err(|error| {
                        tracing::warn!("Failed to create an OpenGL context: {error}")
                    })
                    .ok()
            })
            .ok_or("could not create an OpenGL context with any profile")?;

        let size = window.surface_size();

        let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            window_handle,
            NonZeroU32::new(size.width).ok_or("OpenGL window has zero width")?,
            NonZeroU32::new(size.height).ok_or("OpenGL window has zero height")?,
        );

        let gl_surface = unsafe {
            gl_config
                .display()
                .create_window_surface(gl_config, &attrs)?
        };

        let gl_context = not_current_gl_context.make_current(&gl_surface)?;

        if let Err(error) =
            gl_surface.set_swap_interval(&gl_context, SwapInterval::Wait(NonZeroU32::MIN))
        {
            tracing::warn!("Failed to enable OpenGL vsync: {error}");
        }

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
        .ok_or("could not create OpenGL interface")?;

        let fb_info = {
            let mut fboid: GLint = 0;
            unsafe { GetIntegerv(FRAMEBUFFER_BINDING, &mut fboid) };

            FramebufferInfo {
                fboid: fboid.try_into()?,
                format: Format::RGBA8.into(),
                ..Default::default()
            }
        };

        let renderer = unsafe { GetString(RENDERER) };
        let gpu_name = (!renderer.is_null()).then(|| {
            unsafe { CStr::from_ptr(renderer.cast()) }
                .to_string_lossy()
                .into_owned()
        });

        let num_samples = gl_config.num_samples() as usize;
        let stencil_size = gl_config.stencil_size() as usize;

        let mut gr_context = direct_contexts::make_gl(interface, None)
            .ok_or("could not create OpenGL direct context")?;

        gr_context.set_resource_cache_limit(gpu_resource_cache_limit);

        let render_target = backend_render_targets::make_gl(
            (size.width as i32, size.height as i32),
            num_samples,
            stencil_size,
            fb_info,
        );
        let surface = wrap_render_target(
            &mut gr_context,
            &render_target,
            SurfaceOrigin::BottomLeft,
            ColorType::RGBA8888,
        )
        .ok_or("could not create OpenGL skia surface")?;

        let driver = OpenGLDriver {
            gl_context,
            gl_surface,
            gr_context,
            num_samples,
            stencil_size,
            fb_info,
            surface,
            gpu_name,
            #[cfg(target_os = "windows")]
            last_present: None,
        };

        Ok(driver)
    }

    pub fn present(&mut self, window: &dyn Window, render: impl FnOnce(&mut SkiaSurface)) {
        if !self.gl_context.is_current() {
            self.gl_context.make_current(&self.gl_surface).unwrap();
        }

        render(&mut self.surface);

        window.pre_present_notify();
        if let Err(error) = self.gr_context.flush_submit_and_sync_cpu() {
            tracing::error!(?error, "Failed to submit OpenGL rendering commands");
            return;
        }
        if let Err(error) = self.gl_surface.swap_buffers(&self.gl_context) {
            tracing::error!("Failed to swap buffers: {:?}", error);
        }

        #[cfg(target_os = "windows")]
        crate::drivers::wait_for_display(window, &mut self.last_present);
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        let render_target = backend_render_targets::make_gl(
            (size.width as i32, size.height as i32),
            self.num_samples,
            self.stencil_size,
            self.fb_info,
        );
        let surface = wrap_render_target(
            &mut self.gr_context,
            &render_target,
            SurfaceOrigin::BottomLeft,
            ColorType::RGBA8888,
        )
        .expect("Could not create skia surface");

        self.gl_surface.resize(
            &self.gl_context,
            NonZeroU32::new(size.width.max(1)).unwrap(),
            NonZeroU32::new(size.height.max(1)).unwrap(),
        );

        self.surface = surface;
    }

    fn config_template(transparent: bool) -> ConfigTemplateBuilder {
        ConfigTemplateBuilder::new()
            // Always with alpha on Android, without it, Skia cannot read the window's
            // framebuffer back there, and a backdrop blur silently draws nothing.
            .with_alpha_size(if transparent || cfg!(target_os = "android") {
                8
            } else {
                0
            })
            .with_transparency(transparent)
    }

    /// Pick the OpenGL config.
    fn pick_config(configs: Box<dyn Iterator<Item = Config> + '_>, transparent: bool) -> Config {
        configs
            .max_by_key(|config| {
                let supports_transparency =
                    transparent && config.supports_transparency().unwrap_or(false);

                (
                    config.hardware_accelerated(),
                    supports_transparency,
                    -(config.num_samples() as i32),
                )
            })
            .expect("at least one OpenGL config")
    }
}
