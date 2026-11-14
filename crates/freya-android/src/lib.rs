#![cfg(target_os = "android")]

use freya_components::theming::{
    hooks::use_init_theme,
    themes::light_theme,
};
use freya_core::{
    integration::*,
    prelude::*,
};
use freya_winit::plugins::{
    FreyaPlugin,
    PluginEvent,
    PluginHandle,
};
use torin::prelude::Gaps;
use winit::platform::android::activity::AndroidApp;

mod status_bar;

/// Extension trait for [`Platform`] providing Android-specific APIs.
pub trait AndroidExt {
    /// Set whether the Android status bar uses light appearance (dark icons for light backgrounds).
    fn set_status_bar_light(&self, light: bool) -> Result<(), jni::errors::Error>;

    /// Show the Android soft keyboard.
    fn show_keyboard(&self) -> Result<(), jni::errors::Error>;

    /// Hide the Android soft keyboard.
    fn hide_keyboard(&self) -> Result<(), jni::errors::Error>;

    /// Content padding in logical pixels for the current window.
    /// Call during rendering to track size, scale, and focus changes.
    fn content_padding(&self) -> Gaps;
}

impl AndroidExt for Platform {
    fn set_status_bar_light(&self, light: bool) -> Result<(), jni::errors::Error> {
        if let Some(app) = GlobalContexts::get().try_get_context::<AndroidApp>() {
            status_bar::set_status_bar_light(&app, light)
        } else {
            Ok(())
        }
    }

    fn show_keyboard(&self) -> Result<(), jni::errors::Error> {
        if let Some(app) = GlobalContexts::get().try_get_context::<AndroidApp>() {
            app.show_soft_input(false);
        }

        Ok(())
    }

    fn hide_keyboard(&self) -> Result<(), jni::errors::Error> {
        if let Some(app) = GlobalContexts::get().try_get_context::<AndroidApp>() {
            app.hide_soft_input(false);
        }

        Ok(())
    }

    fn content_padding(&self) -> Gaps {
        let window = self.current_window();
        let size = *window.root_size.read();
        let scale_factor = *window.scale_factor.read() as f32;
        // Refresh the content rectangle when the app regains focus.
        let _ = *window.is_app_focused.read();

        let Some(app) = GlobalContexts::get().try_get_context::<AndroidApp>() else {
            return Gaps::default();
        };
        let content = app.content_rect();
        if content.right <= content.left || content.bottom <= content.top {
            return Gaps::default();
        }

        Gaps::new(
            (content.top as f32 / scale_factor).clamp(0., size.height),
            (size.width - content.right as f32 / scale_factor).clamp(0., size.width),
            (size.height - content.bottom as f32 / scale_factor).clamp(0., size.height),
            (content.left as f32 / scale_factor).clamp(0., size.width),
        )
    }
}

/// Registers the Android context and syncs status-bar appearance with the app theme.
pub struct AndroidPlugin {
    app: AndroidApp,
}

impl AndroidPlugin {
    pub fn new(app: AndroidApp) -> Self {
        Self { app }
    }
}

impl FreyaPlugin for AndroidPlugin {
    fn plugin_id(&self) -> &'static str {
        "android"
    }

    fn on_event(&mut self, event: &mut PluginEvent, _handle: PluginHandle) {
        if let PluginEvent::RunnerCreated { .. } = event {
            GlobalContexts::get().insert_context(self.app.clone());
        }
    }

    fn root_component(&self, root: Element) -> Element {
        AndroidRoot { inner: root }.into_element()
    }
}

/// Root component that manages Android platform integration.
#[derive(Clone)]
struct AndroidRoot {
    inner: Element,
}

impl PartialEq for AndroidRoot {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Component for AndroidRoot {
    fn render(&self) -> impl IntoElement {
        let theme = use_init_theme(light_theme);

        // Sync status bar appearance with theme
        use_side_effect(move || {
            let platform = Platform::get();
            let is_light = theme.read().name == "light";
            if let Err(err) = platform.set_status_bar_light(is_light) {
                tracing::error!("Failed to set status bar appearance: {err:?}");
            }
        });

        self.inner.clone()
    }
}
