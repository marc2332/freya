use std::mem;
use dioxus_core::VirtualDom;
use winit::{
    dpi::LogicalSize,
    event_loop::{ActiveEventLoop, EventLoopProxy},
    window::Window,
};
#[cfg(not(target_os = "macos"))]
use winit::window::WindowAttributes;
#[cfg(not(target_os = "macos"))]
use glutin::config::Config;
#[cfg(not(target_os = "macos"))]
use glutin_winit::DisplayBuilder;
#[cfg(not(target_os = "macos"))]
use glutin::{
    config::{ConfigTemplateBuilder, GlConfig},
};

use freya_common::EventMessage;
use freya_core::dom::SafeDOM;
use freya_engine::prelude::*;

use crate::{app::Application, config::WindowConfig, devtools::Devtools, LaunchConfig};
#[cfg(not(target_os = "macos"))]
use crate::skia::gl_surface::SkiaSurface;
#[cfg(target_os = "macos")]
use crate::skia::metal_surface::SkiaSurface;

pub struct NotCreatedState<'a, State: Clone + 'static> {
    pub(crate) sdom: SafeDOM,
    pub(crate) vdom: VirtualDom,
    pub(crate) devtools: Option<Devtools>,
    pub(crate) config: LaunchConfig<'a, State>,
}

pub struct CreatedState {
    pub(crate) skia_surface: SkiaSurface,
    pub(crate) window: Window,
    pub(crate) window_config: WindowConfig,
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
            .with_inner_size(LogicalSize::<f64>::from(config.window_config.size));

        set_resource_cache_total_bytes_limit(1000000); // 1MB
        set_resource_cache_single_allocation_byte_limit(Some(500000)); // 0.5MB

        if let Some(min_size) = config.window_config.min_size {
            window_attributes =
                window_attributes.with_min_inner_size(LogicalSize::<f64>::from(min_size));
        }
        if let Some(max_size) = config.window_config.max_size {
            window_attributes =
                window_attributes.with_max_inner_size(LogicalSize::<f64>::from(max_size));
        }

        if let Some(with_window_attributes) = &config.window_config.window_attributes_hook {
            window_attributes = (with_window_attributes)(window_attributes);
        }

        #[cfg(not(target_os = "macos"))]
        let template = ConfigTemplateBuilder::new()
            .with_alpha_size(8)
            .with_transparency(config.window_config.transparent);
        #[cfg(not(target_os = "macos"))]
        let (window, gl_config) = create_gl_window(window_attributes, event_loop, template);
        #[cfg(not(target_os = "macos"))]
        let skia_surface = SkiaSurface::create(&window, gl_config);

        #[cfg(target_os = "macos")]
        let window = event_loop.create_window(window_attributes).expect("Failed to create Window");
        #[cfg(target_os = "macos")]
        let skia_surface = SkiaSurface::create(&window);

        // Allow IME
        window.set_ime_allowed(true);

        // Mak the window visible once built
        window.set_visible(true);
        
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

        app.init_doms(skia_surface.scale_factor as f32, config.state.clone());
        app.process_layout(window.inner_size(), skia_surface.scale_factor);

        *self = WindowState::Created(CreatedState {
            skia_surface,
            window,
            app,
            window_config: config.window_config,
            is_window_focused: false,
        });
    }
}

#[cfg(not(target_os = "macos"))]
fn create_gl_window(
    window_attributes: WindowAttributes,
    event_loop: &ActiveEventLoop,
    template: ConfigTemplateBuilder,
) -> (Window, Config) {
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

    (window, gl_config)
}
