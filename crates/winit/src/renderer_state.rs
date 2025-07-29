use std::collections::HashMap;

use dioxus_core::VirtualDom;
use freya_components::{
    FreyaApp,
    FreyaAppProps,
};
use freya_core::{
    dom::{
        FreyaDOM,
        SafeDOM,
    },
    event_loop_messages::EventLoopMessage,
    plugins::PluginsManager,
    window_config::WindowConfig,
};
use freya_engine::prelude::*;
use winit::{
    dpi::LogicalSize,
    event_loop::{
        ActiveEventLoop,
        EventLoopProxy,
    },
    window::{
        Window,
        WindowId,
    },
};

use crate::{
    accessibility::WinitAcessibilityTree,
    app::Application,
    drivers::GraphicsDriver,
    size::WinitSize,
    EmbeddedFonts,
};

pub struct RendererState<State: Clone + 'static> {
    pub(crate) windows_configs: Vec<WindowConfig>,
    pub(crate) apps: HashMap<WindowId, Application>,

    pub(crate) font_collection: FontCollection,
    pub(crate) font_mgr: FontMgr,
    pub(crate) proxy: EventLoopProxy<EventLoopMessage>,

    pub(crate) state: Option<State>,
    pub(crate) plugins: PluginsManager,
    pub(crate) fallback_fonts: Vec<String>,

    pub(crate) resumed: bool,
}

impl<State: Clone + 'static> RendererState<State> {
    pub fn new(
        state: Option<State>,
        windows_configs: Vec<WindowConfig>,
        embedded_fonts: EmbeddedFonts<'_>,
        plugins: PluginsManager,
        fallback_fonts: Vec<String>,
        proxy: EventLoopProxy<EventLoopMessage>,
    ) -> Self {
        let mut font_collection = FontCollection::new();
        let def_mgr = FontMgr::default();

        let mut provider = TypefaceFontProvider::new();
        for (font_name, font_data) in embedded_fonts {
            let ft_type = def_mgr.new_from_data(font_data, None).unwrap();
            provider.register_typeface(ft_type, Some(font_name));
        }

        let font_mgr: FontMgr = provider.into();
        font_collection.set_default_font_manager(def_mgr, None);
        font_collection.set_dynamic_font_manager(font_mgr.clone());

        Self {
            state,
            windows_configs,
            plugins,
            fallback_fonts,

            apps: HashMap::default(),
            font_collection,
            font_mgr,
            proxy: proxy.clone(),

            resumed: false,
        }
    }

    pub fn with_app(
        &mut self,
        app: WindowId,
        cb: impl FnOnce(&mut Application, PartialRendererState<'_>),
    ) {
        if let Some(app) = self.apps.get_mut(&app) {
            cb(
                app,
                PartialRendererState {
                    fallback_fonts: &mut self.fallback_fonts,

                    font_collection: &mut self.font_collection,
                    font_mgr: &mut self.font_mgr,
                },
            );
        }
    }

    pub fn new_app(
        &mut self,
        event_loop: &ActiveEventLoop,
        mut window_config: WindowConfig,
    ) -> WindowId {
        let mut window_attributes = Window::default_attributes()
            .with_visible(false)
            .with_title(window_config.title)
            .with_decorations(window_config.decorations)
            .with_transparent(window_config.transparent)
            .with_window_icon(window_config.icon.take())
            .with_inner_size(LogicalSize::<f64>::from(window_config.size));

        if let Some(min_size) = window_config.min_size {
            window_attributes =
                window_attributes.with_min_inner_size(LogicalSize::<f64>::from(min_size));
        }
        if let Some(max_size) = window_config.max_size {
            window_attributes =
                window_attributes.with_max_inner_size(LogicalSize::<f64>::from(max_size));
        }

        if let Some(with_window_attributes) = window_config.window_attributes_hook.take() {
            window_attributes = (with_window_attributes)(window_attributes);
        }

        let (graphics_driver, window, mut surface) =
            GraphicsDriver::new(event_loop, window_attributes, &window_config);

        let accessibility = WinitAcessibilityTree::new(event_loop, &window, self.proxy.clone());

        if window_config.visible {
            window.set_visible(true);
        }

        // Allow IME
        window.set_ime_allowed(true);

        let mut dirty_surface = surface
            .new_surface_with_dimensions(window.inner_size().to_skia())
            .unwrap();

        let scale_factor = window.scale_factor();
        let id = window.id();

        surface
            .canvas()
            .scale((scale_factor as f32, scale_factor as f32));
        surface.canvas().clear(window_config.background);

        dirty_surface
            .canvas()
            .scale((scale_factor as f32, scale_factor as f32));
        dirty_surface.canvas().clear(window_config.background);

        let sdom = SafeDOM::new(FreyaDOM::default());
        let app = window_config.app.take().unwrap();
        let vdom = VirtualDom::new_with_props(FreyaApp, FreyaAppProps { app });

        let mut app = Application::new(
            sdom,
            vdom,
            &self.proxy,
            window,
            accessibility,
            surface,
            dirty_surface,
            graphics_driver,
            window_config,
            self.plugins.clone(),
        );

        app.init_doms(scale_factor as f32, self.state.clone());
        app.process_layout(
            scale_factor,
            &mut self.font_collection,
            &self.fallback_fonts,
        );

        self.apps.insert(id, app);

        id
    }
}

pub struct PartialRendererState<'a> {
    pub fallback_fonts: &'a mut Vec<String>,

    pub(crate) font_collection: &'a mut FontCollection,
    pub(crate) font_mgr: &'a mut FontMgr,
}
