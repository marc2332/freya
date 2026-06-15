use freya_core::{
    events::data::{
        Event,
        SizedEventData,
    },
    prelude::*,
};

use crate::{
    element::webview,
    lifecycle::WebViewLifecycleEvent,
    prelude::{
        WebViewConfig,
        WebViewId,
    },
    registry::WebViewCallback,
};

/// Embeds a native WebView pointing to a given URL as a regular element in your UI.
///
/// # Requirements
///
/// The [`WebViewPlugin`](crate::plugin::WebViewPlugin) must be registered in your `LaunchConfig`
/// for this component to work, otherwise it will panic:
///
/// ```rust,no_run
/// # use freya::prelude::*;
/// # use freya_webview::prelude::*;
/// launch(
///     LaunchConfig::new()
///         .with_plugin(WebViewPlugin::new())
///         .with_window(WindowConfig::new(app)),
/// );
/// # fn app() -> impl IntoElement { rect() }
/// ```
///
/// WebViews are not supported on every platform, see the [crate-level docs](crate#platform-compatibility)
/// for details.
///
/// # Example
///
/// ```rust,no_run
/// # use freya::prelude::*;
/// # use freya_webview::prelude::*;
/// fn app() -> impl IntoElement {
///     WebView::new("https://example.com").expanded()
/// }
/// ```
#[derive(Clone)]
pub struct WebView {
    webview_id: WebViewId,
    url: String,
    close_on_drop: bool,
    on_created: Option<WebViewCallback>,
    layout: LayoutData,
}

impl PartialEq for WebView {
    fn eq(&self, other: &Self) -> bool {
        self.webview_id == other.webview_id
            && self.url == other.url
            && self.close_on_drop == other.close_on_drop
            && match (&self.on_created, &other.on_created) {
                (None, None) => true,
                (Some(a), Some(b)) => std::sync::Arc::ptr_eq(a, b),
                _ => false,
            }
            && self.layout == other.layout
    }
}

impl WebView {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            webview_id: WebViewId::new(),
            url: url.into(),
            close_on_drop: true,
            on_created: None,
            layout: LayoutData::default(),
        }
    }

    /// Set the URL the WebView will load.
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = url.into();
        self
    }

    /// Set a custom [`WebViewId`] so the WebView can be referenced later, e.g. with
    /// [`WebViewManager::close`](crate::lifecycle::WebViewManager::close).
    pub fn id(mut self, id: WebViewId) -> Self {
        self.webview_id = id;
        self
    }

    /// If you decide to not close the webview on drop you will need to manually close it with [WebViewManager::close](crate::lifecycle::WebViewManager::close).
    pub fn close_on_drop(mut self, close: bool) -> Self {
        self.close_on_drop = close;
        self
    }

    /// Customize the underlying [`wry::WebViewBuilder`] right before the WebView is built, for
    /// example to set a custom user agent or inject initialization scripts.
    pub fn on_created(
        mut self,
        on_created: impl Fn(wry::WebViewBuilder) -> wry::WebViewBuilder + Send + 'static,
    ) -> Self {
        self.on_created = Some(WebViewCallback::new(Box::new(on_created)));
        self
    }
}

impl LayoutExt for WebView {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.layout
    }
}
impl ContainerExt for WebView {}

const WEBVIEW_CONTEXT_ERROR: &str = "
Error: Make sure to register the WebViewPlugin in your LaunchConfig:

LaunchConfig::new()
    .with_plugin(WebViewPlugin::new())
";

impl Component for WebView {
    fn render(&self) -> impl IntoElement {
        let events = try_consume_root_context::<crate::lifecycle::WebViewEvents>()
            .expect(WEBVIEW_CONTEXT_ERROR);

        let webview_id = self.webview_id;
        let url = self.url.clone();
        let close_on_drop = self.close_on_drop;
        let on_created = self.on_created.clone();

        let config = WebViewConfig {
            url: url.clone(),
            transparent: false,
            user_agent: None,
            on_created,
        };

        use_drop({
            let events = events.clone();
            move || {
                events.lock().unwrap().push(if close_on_drop {
                    WebViewLifecycleEvent::Close { id: webview_id }
                } else {
                    WebViewLifecycleEvent::Hide { id: webview_id }
                });
            }
        });

        webview(&url)
            .layout(self.layout.clone())
            .on_sized(move |event: Event<SizedEventData>| {
                events.lock().unwrap().push(WebViewLifecycleEvent::Resized {
                    id: webview_id,
                    area: event.area,
                    config: config.clone(),
                });
                Platform::get().send(UserEvent::RequestRedraw);
            })
    }

    fn render_key(&self) -> DiffKey {
        DiffKey::from(&self.webview_id)
    }
}
