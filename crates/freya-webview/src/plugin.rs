//! WebView plugin for managing WRY WebViews.

use std::collections::HashMap;

use freya_winit::{
    plugins::{
        FreyaPlugin,
        PluginEvent,
        PluginHandle,
    },
    winit::window::Window,
};
use torin::prelude::Area;
use wry::{
    Rect,
    WebView as WryWebView,
    WebViewBuilder,
};

use crate::{
    lifecycle::{
        WebViewEvents,
        WebViewLifecycleEvent,
    },
    registry::{
        WebViewConfig,
        WebViewId,
    },
};

/// State for a managed WebView.
struct WebViewState {
    webview: WryWebView,
}

/// Plugin for managing WebView instances.
///
/// This plugin handles the lifecycle of WRY WebViews, creating them when
/// elements are added and destroying them when elements are removed.
pub struct WebViewPlugin {
    webviews: HashMap<WebViewId, WebViewState>,
    events: WebViewEvents,
    #[cfg(target_os = "linux")]
    gtk_initialized: bool,
}

impl Default for WebViewPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl WebViewPlugin {
    /// Create a new WebView plugin.
    pub fn new() -> Self {
        Self {
            webviews: HashMap::new(),
            events: WebViewEvents::default(),
            #[cfg(target_os = "linux")]
            gtk_initialized: false,
        }
    }

    #[cfg(target_os = "linux")]
    fn ensure_gtk_initialized(&mut self) {
        if !self.gtk_initialized {
            // Initialize GTK for WebKitGTK on Linux
            if gtk::init().is_ok() {
                tracing::debug!("WebViewPlugin: GTK initialized");
                self.gtk_initialized = true;
            } else {
                tracing::error!("WebViewPlugin: Failed to initialize GTK");
            }
        }
    }

    #[cfg(target_os = "linux")]
    fn advance_gtk_event_loop(&self) -> bool {
        if self.gtk_initialized {
            // Process pending GTK events and return true if any were processed
            let mut processed = false;
            while gtk::events_pending() {
                gtk::main_iteration_do(false);
                processed = true;
            }
            processed
        } else {
            false
        }
    }

    fn create_webview(
        &mut self,
        window: &Window,
        id: WebViewId,
        config: &WebViewConfig,
        area: &Area,
    ) {
        tracing::debug!(
            "WebViewPlugin: Creating WebView {:?} with URL {}",
            id,
            config.url
        );

        let mut builder = WebViewBuilder::new();

        if !config.url.is_empty() {
            builder = builder.with_url(&config.url);
        }

        if config.transparent {
            builder = builder.with_transparent(true);
        }

        if let Some(ref user_agent) = config.user_agent {
            builder = builder.with_user_agent(user_agent);
        }

        if let Some(ref on_created) = config.on_created {
            builder = (on_created)(builder)
        }

        // Set initial bounds
        let bounds = Rect {
            position: wry::dpi::Position::Logical(area.origin.to_f64().to_tuple().into()),
            size: wry::dpi::Size::Logical(area.size.to_f64().to_tuple().into()),
        };
        tracing::info!(
            "WebViewPlugin: Creating WebView with bounds: pos=({}, {}), size=({}, {})",
            area.min_x(),
            area.min_y(),
            area.width(),
            area.height()
        );
        builder = builder.with_bounds(bounds).with_visible(true);

        // Build the WebView as a child of the window
        match builder.build_as_child(window) {
            Ok(webview) => {
                // Ensure visibility after creation
                let _ = webview.set_visible(true);

                self.webviews.insert(id, WebViewState { webview });
                tracing::debug!("WebViewPlugin: WebView {:?} created successfully", id);
            }
            Err(e) => {
                tracing::error!("WebViewPlugin: Failed to create WebView {:?}: {}", id, e);
            }
        }
    }
}

impl FreyaPlugin for WebViewPlugin {
    fn inject_root_context(&mut self, runner: &mut freya_core::runner::Runner) {
        runner.provide_root_context(|| self.events.clone());
    }

    fn on_event(&mut self, event: &PluginEvent, _handle: PluginHandle) {
        match event {
            PluginEvent::WindowCreated { .. } => {
                // Window created, initialize GTK on Linux
                tracing::debug!("WebViewPlugin: Window created");
                #[cfg(target_os = "linux")]
                self.ensure_gtk_initialized();
            }

            PluginEvent::FinishedMeasuringLayout { window, .. } => {
                // First, drain all lifecycle events from the channel into a vec
                let events: Vec<WebViewLifecycleEvent> =
                    self.events.lock().unwrap().drain(..).collect();

                // Then process the events
                for event in events {
                    match event {
                        WebViewLifecycleEvent::Resized { id, area, config } => {
                            tracing::debug!("WebViewPlugin: Received Resized event for {:?}", id);
                            // Check if this is a pending webview that needs creation
                            if let Some(state) = self.webviews.get_mut(&id) {
                                if let Err(e) = state.webview.set_visible(true) {
                                    tracing::error!(
                                        "WebViewPlugin: Failed to set visible {:?}: {}",
                                        id,
                                        e
                                    );
                                }
                                let rect = Rect {
                                    position: wry::dpi::Position::Logical(
                                        area.origin.to_f64().to_tuple().into(),
                                    ),
                                    size: wry::dpi::Size::Logical(
                                        area.size.to_f64().to_tuple().into(),
                                    ),
                                };

                                let resize = match state.webview.bounds() {
                                    Ok(r) => r.size != rect.size || r.position != rect.position,
                                    _ => true,
                                };
                                if resize && let Err(e) = state.webview.set_bounds(rect) {
                                    tracing::error!(
                                        "WebViewPlugin: Failed to set bounds for {:?}: {}",
                                        id,
                                        e
                                    );
                                }
                            } else {
                                self.create_webview(window, id, &config, &area);
                            }
                        }
                        WebViewLifecycleEvent::Close { id } => {
                            tracing::debug!("WebViewPlugin: Received Close event for {:?}", id);
                            self.webviews.remove(&id);
                        }
                        WebViewLifecycleEvent::Hide { id } => {
                            if let Some(state) = self.webviews.get_mut(&id) {
                                let _ = state.webview.set_visible(false);
                            }
                        }
                    }
                }
            }

            PluginEvent::WindowClosed { .. } => {
                // Clean up all WebViews for this window
                tracing::debug!(
                    "WebViewPlugin: Window closed, cleaning up {} WebViews",
                    self.webviews.len()
                );
                self.webviews.clear();
            }

            #[cfg(target_os = "linux")]
            PluginEvent::AfterRedraw { window, .. } => {
                // Advance GTK event loop on Linux to process WebView events
                self.advance_gtk_event_loop();
                // If we have WebViews, continuously request redraws to keep GTK events flowing
                if !self.webviews.is_empty() {
                    window.request_redraw();
                }
            }

            PluginEvent::BeforeRender { .. } => {
                // Also advance GTK event loop before rendering
                #[cfg(target_os = "linux")]
                {
                    let _ = self.advance_gtk_event_loop();
                }
            }

            _ => {}
        }
    }
}
