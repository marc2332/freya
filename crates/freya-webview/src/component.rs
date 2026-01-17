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

#[derive(Clone)]
pub struct WebViewComponent {
    pub webview_id: WebViewId,
    pub url: String,
    pub close_on_drop: bool,
    pub on_created: Option<WebViewCallback>,
    layout: LayoutData,
}

impl PartialEq for WebViewComponent {
    fn eq(&self, other: &Self) -> bool {
        self.webview_id == other.webview_id
            && self.url == other.url
            && self.close_on_drop == other.close_on_drop
            && self.on_created.is_none() == other.on_created.is_none()
            && self.layout == other.layout
    }
}

impl WebViewComponent {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            webview_id: WebViewId::new(),
            url: url.into(),
            close_on_drop: true,
            on_created: None,
            layout: LayoutData::default(),
        }
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = url.into();
        self
    }

    pub fn id(mut self, id: WebViewId) -> Self {
        self.webview_id = id;
        self
    }

    pub fn close_on_drop(mut self, close: bool) -> Self {
        self.close_on_drop = close;
        self
    }

    pub fn on_created(
        mut self,
        on_created: impl Fn(wry::WebViewBuilder) -> wry::WebViewBuilder + Send + 'static,
    ) -> Self {
        self.on_created = Some(WebViewCallback::new(Box::new(on_created)));
        self
    }
}

impl LayoutExt for WebViewComponent {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.layout
    }
}
impl ContainerExt for WebViewComponent {}

impl Component for WebViewComponent {
    fn render(&self) -> impl IntoElement {
        let lifecycle_sender = consume_root_context::<crate::lifecycle::WebViewEvents>();

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
            let lifecycle_sender = lifecycle_sender.clone();
            move || {
                lifecycle_sender.lock().unwrap().push(if close_on_drop {
                    WebViewLifecycleEvent::Close { id: webview_id }
                } else {
                    WebViewLifecycleEvent::Hide { id: webview_id }
                });
            }
        });

        webview(&url)
            .layout(self.layout.clone())
            .on_sized(move |event: Event<SizedEventData>| {
                let lifecycle_sender = lifecycle_sender.clone();
                let area = event.area;
                lifecycle_sender
                    .lock()
                    .unwrap()
                    .push(WebViewLifecycleEvent::Resized {
                        id: webview_id,
                        area,
                        config: config.clone(),
                    });
            })
    }

    fn render_key(&self) -> DiffKey {
        DiffKey::from(&self.webview_id)
    }
}
