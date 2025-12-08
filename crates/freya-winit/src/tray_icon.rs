use std::borrow::Cow;

use freya_core::prelude::ScreenReader;
use freya_engine::prelude::{
    FontCollection,
    FontMgr,
};
use rustc_hash::FxHashMap;
use tray_icon::{
    TrayIconEvent,
    menu::MenuEvent,
};
use winit::{
    event_loop::{
        ActiveEventLoop,
        EventLoopProxy,
    },
    window::WindowId,
};

use crate::{
    config::{
        TrayHandler,
        WindowConfig,
    },
    plugins::PluginsManager,
    renderer::{
        NativeEvent,
        NativeWindowEvent,
        NativeWindowEventAction,
        WinitRenderer,
    },
    window::AppWindow,
};

#[derive(Clone, Debug)]
pub enum TrayEvent {
    Icon(TrayIconEvent),
    Menu(MenuEvent),
}

pub struct TrayContext<'a> {
    pub windows: &'a mut FxHashMap<WindowId, AppWindow>,
    pub proxy: &'a mut EventLoopProxy<NativeEvent>,
    pub plugins: &'a mut PluginsManager,
    pub fallback_fonts: &'a mut Vec<Cow<'static, str>>,
    pub screen_reader: &'a mut ScreenReader,
    pub font_manager: &'a mut FontMgr,
    pub font_collection: &'a mut FontCollection,
    pub(crate) active_event_loop: &'a ActiveEventLoop,
}

impl<'a> TrayContext<'a> {
    pub fn new(
        active_event_loop: &'a ActiveEventLoop,
        renderer: &'a mut WinitRenderer,
    ) -> Option<(Self, &'a mut TrayHandler)> {
        let tray_handler = renderer.tray.1.as_mut()?;

        let context = TrayContext {
            active_event_loop,
            windows: &mut renderer.windows,
            proxy: &mut renderer.proxy,
            plugins: &mut renderer.plugins,
            fallback_fonts: &mut renderer.fallback_fonts,
            screen_reader: &mut renderer.screen_reader,
            font_manager: &mut renderer.font_manager,
            font_collection: &mut renderer.font_collection,
        };

        Some((context, tray_handler))
    }
}

impl TrayContext<'_> {
    pub fn launch_window(&mut self, window_config: WindowConfig) {
        let app_window = AppWindow::new(
            window_config,
            self.active_event_loop,
            self.proxy,
            self.plugins,
            self.font_collection,
            self.font_manager,
            self.fallback_fonts,
            self.screen_reader.clone(),
        );

        self.proxy
            .send_event(NativeEvent::Window(NativeWindowEvent {
                window_id: app_window.window.id(),
                action: NativeWindowEventAction::PollRunner,
            }))
            .ok();

        self.windows.insert(app_window.window.id(), app_window);
    }

    pub fn windows(&self) -> &FxHashMap<WindowId, AppWindow> {
        self.windows
    }

    pub fn windows_mut(&mut self) -> &mut FxHashMap<WindowId, AppWindow> {
        self.windows
    }

    pub fn exit(&mut self) {
        self.active_event_loop.exit();
    }
}
