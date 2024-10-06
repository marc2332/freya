use std::mem;

use dioxus_core::VirtualDom;
use freya_core::{
    dom::SafeDOM,
    prelude::EventMessage,
};
use freya_engine::prelude::*;
use winit::{
    dpi::LogicalSize,
    event_loop::{
        ActiveEventLoop,
        EventLoopProxy,
    },
    window::Window,
};

use crate::{
    app::Application,
    config::WindowConfig,
    devtools::Devtools,
    drivers::GraphicsDriver,
    size::WinitSize,
    LaunchConfig,
};

pub struct NotCreatedState<'a, State: Clone + 'static> {
    pub(crate) sdom: SafeDOM,
    pub(crate) vdom: VirtualDom,
    pub(crate) devtools: Option<Devtools>,
    pub(crate) config: LaunchConfig<'a, State>,
}

pub struct CreatedState {
    pub(crate) app: Application,
    pub(crate) surface: Surface,
    pub(crate) dirty_surface: Surface,
    pub(crate) graphics_driver: GraphicsDriver,
    pub(crate) window: Window,
    pub(crate) window_config: WindowConfig,
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

        if let Some(with_window_attributes) = config.window_config.window_attributes_hook.take() {
            window_attributes = (with_window_attributes)(window_attributes);
        }

        let (graphics_driver, window, mut surface) =
            GraphicsDriver::new(event_loop, window_attributes, &config);

        if config.window_config.visible {
            window.set_visible(true);
        }

        // Allow IME
        window.set_ime_allowed(true);

        let mut dirty_surface = surface
            .new_surface_with_dimensions(window.inner_size().to_skia())
            .unwrap();

        let scale_factor = window.scale_factor();

        surface
            .canvas()
            .scale((scale_factor as f32, scale_factor as f32));
        surface.canvas().clear(config.window_config.background);

        dirty_surface
            .canvas()
            .scale((scale_factor as f32, scale_factor as f32));
        dirty_surface
            .canvas()
            .clear(config.window_config.background);

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

        app.init_doms(scale_factor as f32, config.state.clone());
        app.process_layout(window.inner_size(), scale_factor);

        *self = WindowState::Created(CreatedState {
            surface,
            dirty_surface,
            graphics_driver,
            window,
            app,
            window_config: config.window_config,
            is_window_focused: false,
        });
    }
}
