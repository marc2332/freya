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
use winit::{
    event::WindowEvent,
    platform::android::activity::AndroidApp,
};

use crate::window_insets::WindowInsets;

mod font_size;
mod status_bar;
mod window_insets;

/// Extension trait for [`Platform`] providing Android-specific APIs.
pub trait AndroidExt {
    /// Set whether the Android status bar uses light appearance (dark icons for light backgrounds).
    fn set_status_bar_light(&self, light: bool) -> Result<(), jni::errors::Error>;

    /// Show the Android soft keyboard.
    fn show_keyboard(&self) -> Result<(), jni::errors::Error>;

    /// Hide the Android soft keyboard.
    fn hide_keyboard(&self) -> Result<(), jni::errors::Error>;

    /// System bar and cutout padding in logical pixels for edge-to-edge layouts on API 30+.
    /// Call during rendering to track inset and scale changes.
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
        let Some(insets) = try_consume_root_context::<State<WindowInsets>>() else {
            return Gaps::default();
        };
        let insets = insets.read();
        let scale_factor = *self.current_window().scale_factor.read() as f32;

        Gaps::new(
            insets.top as f32 / scale_factor,
            insets.right as f32 / scale_factor,
            insets.bottom as f32 / scale_factor,
            insets.left as f32 / scale_factor,
        )
    }
}

/// Provides Android window insets and syncs status-bar appearance with the app theme.
pub struct AndroidPlugin {
    app: AndroidApp,
    insets: Option<State<WindowInsets>>,
}

impl AndroidPlugin {
    pub fn new(app: AndroidApp) -> Self {
        Self { app, insets: None }
    }

    fn refresh_insets(&mut self) {
        if let Some(insets) = &mut self.insets {
            match WindowInsets::read(&self.app) {
                Ok(value) => insets.set_if_modified(value),
                Err(error) => tracing::warn!(?error, "Failed to read Android window insets"),
            }
        }
    }
}

impl AndroidPlugin {
    fn refresh_font_size(&self, tree: &mut Tree) {
        let converter = match font_size::converter(&self.app) {
            Ok(converter) => Some(converter),
            Err(error) => {
                tracing::warn!(%error, "Could not read Android display metrics");
                None
            }
        };
        tree.set_font_size_converter(converter);
    }
}

impl FreyaPlugin for AndroidPlugin {
    fn plugin_id(&self) -> &'static str {
        "android"
    }

    fn on_event(&mut self, event: &mut PluginEvent, _handle: PluginHandle) {
        match event {
            PluginEvent::RunnerCreated { runner } => {
                GlobalContexts::get().insert_context(self.app.clone());
                self.insets =
                    Some(runner.provide_root_context(|| State::create(WindowInsets::default())));
                self.refresh_insets();
            }
            PluginEvent::TreeCreated { tree, .. } => self.refresh_font_size(tree),
            PluginEvent::WindowResumed { tree, .. }
            | PluginEvent::WindowEvent {
                raw: WindowEvent::ScaleFactorChanged { .. },
                tree,
                ..
            } => {
                self.refresh_insets();
                self.refresh_font_size(tree);
            }
            // TODO(marc2332): Refresh on inset changes that do not resize the surface.
            PluginEvent::WindowEvent {
                raw: WindowEvent::Focused(true) | WindowEvent::SurfaceResized(_),
                ..
            } => self.refresh_insets(),
            PluginEvent::WindowClosed { .. } => self.insets = None,
            _ => {}
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
