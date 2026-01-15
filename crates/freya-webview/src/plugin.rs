//! WebView plugin for managing WRY WebViews.

use std::collections::{
    HashMap,
    HashSet,
};

use freya_winit::plugins::{
    FreyaPlugin,
    PluginEvent,
    PluginHandle,
};
use torin::prelude::Area;
use wry::{
    Rect,
    WebView as WryWebView,
    WebViewBuilder,
};

use crate::{
    element::WebView,
    registry::{
        WebViewConfig,
        WebViewId,
    },
};

/// State for a managed WebView.
struct WebViewState {
    webview: WryWebView,
    config: WebViewConfig,
    last_area: Option<Area>,
}

/// Plugin for managing WebView instances.
///
/// This plugin handles the lifecycle of WRY WebViews, creating them when
/// elements are added and destroying them when elements are removed.
pub struct WebViewPlugin {
    webviews: HashMap<WebViewId, WebViewState>,
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
}

impl FreyaPlugin for WebViewPlugin {
    fn on_event(&mut self, event: &PluginEvent, _handle: PluginHandle) {
        match event {
            PluginEvent::WindowCreated { .. } => {
                // Window created, initialize GTK on Linux
                tracing::debug!("WebViewPlugin: Window created");
                #[cfg(target_os = "linux")]
                self.ensure_gtk_initialized();
            }

            PluginEvent::FinishedMeasuringLayout { window, tree, .. } => {
                // Collect all WebView elements currently in the tree
                let mut current_webviews: HashMap<WebViewId, (WebViewConfig, Area)> =
                    HashMap::new();

                tracing::info!(
                    "WebViewPlugin: Scanning {} elements in tree",
                    tree.elements.len()
                );

                for (node_id, element) in &tree.elements {
                    let is_webview = WebView::try_downcast(element.as_ref()).is_some();
                    tracing::debug!(
                        "WebViewPlugin: Element {:?} is_webview={}",
                        node_id,
                        is_webview
                    );
                    if let Some(webview_element) = WebView::try_downcast(element.as_ref()) {
                        if let Some(layout_node) = tree.layout.get(node_id) {
                            let area = layout_node.visible_area();
                            tracing::info!(
                                "WebViewPlugin: Found WebView {:?} at {:?} ({}x{})",
                                webview_element.webview_id,
                                (area.min_x(), area.min_y()),
                                area.width(),
                                area.height()
                            );
                            current_webviews.insert(
                                webview_element.webview_id,
                                (webview_element.config.clone(), area),
                            );
                        }
                    }
                }

                // Track which WebViews we've seen
                let current_ids: HashSet<WebViewId> = current_webviews.keys().copied().collect();
                let existing_ids: HashSet<WebViewId> = self.webviews.keys().copied().collect();

                // Create new WebViews
                for id in current_ids.difference(&existing_ids) {
                    if let Some((config, area)) = current_webviews.get(id) {
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

                        // Set initial bounds
                        let bounds = Rect {
                            position: wry::dpi::Position::Logical(wry::dpi::LogicalPosition::new(
                                area.min_x() as f64,
                                area.min_y() as f64,
                            )),
                            size: wry::dpi::Size::Logical(wry::dpi::LogicalSize::new(
                                area.width() as f64,
                                area.height() as f64,
                            )),
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

                                self.webviews.insert(
                                    *id,
                                    WebViewState {
                                        webview,
                                        config: config.clone(),
                                        last_area: Some(*area),
                                    },
                                );
                                tracing::debug!(
                                    "WebViewPlugin: WebView {:?} created successfully",
                                    id
                                );
                            }
                            Err(e) => {
                                tracing::error!(
                                    "WebViewPlugin: Failed to create WebView {:?}: {}",
                                    id,
                                    e
                                );
                            }
                        }
                    }
                }

                // Update existing WebViews
                for id in current_ids.intersection(&existing_ids) {
                    if let (Some((config, area)), Some(state)) =
                        (current_webviews.get(id), self.webviews.get_mut(id))
                    {
                        // Check if bounds changed
                        let bounds_changed = state.last_area.as_ref() != Some(area);

                        if bounds_changed {
                            let rect = Rect {
                                position: wry::dpi::Position::Logical(
                                    wry::dpi::LogicalPosition::new(
                                        area.min_x() as f64,
                                        area.min_y() as f64,
                                    ),
                                ),
                                size: wry::dpi::Size::Logical(wry::dpi::LogicalSize::new(
                                    area.width() as f64,
                                    area.height() as f64,
                                )),
                            };
                            if let Err(e) = state.webview.set_bounds(rect) {
                                tracing::error!(
                                    "WebViewPlugin: Failed to set bounds for {:?}: {}",
                                    id,
                                    e
                                );
                            }
                            state.last_area = Some(*area);
                        }

                        // Check if URL changed
                        if config.url != state.config.url && !config.url.is_empty() {
                            if let Err(e) = state.webview.load_url(&config.url) {
                                tracing::error!(
                                    "WebViewPlugin: Failed to load URL for {:?}: {}",
                                    id,
                                    e
                                );
                            }
                            state.config.url = config.url.clone();
                        }
                    }
                }

                // Remove WebViews that are no longer in the tree
                for id in existing_ids.difference(&current_ids) {
                    tracing::debug!("WebViewPlugin: Destroying WebView {:?}", id);
                    self.webviews.remove(id);
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

            PluginEvent::AfterRedraw { window, .. } => {
                // Advance GTK event loop on Linux to process WebView events
                #[cfg(target_os = "linux")]
                {
                    self.advance_gtk_event_loop();
                    // If we have WebViews, continuously request redraws to keep GTK events flowing
                    if !self.webviews.is_empty() {
                        window.request_redraw();
                    }
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
