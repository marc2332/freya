#![cfg(target_os = "android")]

use freya_components::theming::{
    hooks::use_init_theme, themes::light_theme,
};
use freya_core::{
    integration::*,
    prelude::*,
};
use freya_winit::{
    integration::is_ime_role,
    plugins::{
        FreyaPlugin,
        PluginEvent,
        PluginHandle,
    },
};
use winit::platform::android::activity::AndroidApp;

mod keyboard;
mod status_bar;

/// Extension trait for [`Platform`] providing Android-specific APIs.
pub trait AndroidExt {
    /// Set whether the Android status bar uses light appearance (dark icons for light backgrounds).
    fn set_status_bar_light(&self, light: bool) -> Result<(), jni::errors::Error>;

    /// Show the Android soft keyboard.
    fn show_keyboard(&self) -> Result<(), jni::errors::Error>;

    /// Hide the Android soft keyboard.
    fn hide_keyboard(&self) -> Result<(), jni::errors::Error>;
}

impl AndroidExt for Platform {
    fn set_status_bar_light(&self, light: bool) -> Result<(), jni::errors::Error> {
        if let Some(app) = try_consume_root_context::<AndroidApp>() {
            status_bar::set_status_bar_light(&app, light)
        } else {
            Ok(())
        }
    }

    fn show_keyboard(&self) -> Result<(), jni::errors::Error> {
        if let Some(app) = try_consume_root_context::<AndroidApp>() {
            keyboard::show_keyboard(&app)
        } else {
            Ok(())
        }
    }

    fn hide_keyboard(&self) -> Result<(), jni::errors::Error> {
        if let Some(app) = try_consume_root_context::<AndroidApp>() {
            keyboard::hide_keyboard(&app)
        } else {
            Ok(())
        }
    }
}

/// Freya plugin for Android integration.
///
/// Stores the [`AndroidApp`] handle, provides it as root context,
/// and registers a root component that syncs the status bar
/// appearance with the app theme and manages the soft keyboard.
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
        if let PluginEvent::RunnerCreated { runner } = event {
            let app = self.app.clone();
            runner.provide_root_context(move || app);
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

        // Show/hide keyboard based on focused node type
        use_side_effect(move || {
            let platform = Platform::get();
            let focused_node = platform.focused_accessibility_node.read();
            let result = if is_ime_role(focused_node.role()) {
                platform.show_keyboard()
            } else {
                platform.hide_keyboard()
            };
            if let Err(err) = result {
                tracing::error!("Failed to toggle soft keyboard: {err:?}");
            }
        });

        let on_global_pointer_down = move |_: Event<PointerEventData>| {
            let platform = Platform::get();
            if let Err(err) = platform.hide_keyboard() {
                tracing::error!("Failed to hide soft keyboard: {err:?}");
            }
        };

        rect()
            .expanded()
            .on_global_pointer_down(on_global_pointer_down)
            .child(self.inner.clone())
    }
}
