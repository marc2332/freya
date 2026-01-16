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
};

#[derive(PartialEq, Clone)]
pub struct WebViewComponent {
    pub webview_id: WebViewId,
    pub url: String,
    layout: LayoutData,
}

impl WebViewComponent {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            webview_id: WebViewId::new(),
            url: url.into(),
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
        let config = WebViewConfig {
            url: url.clone(),
            transparent: false,
            user_agent: None,
        };

        use_drop({
            let lifecycle_sender = lifecycle_sender.clone();
            move || {
                lifecycle_sender
                    .lock()
                    .unwrap()
                    .push(WebViewLifecycleEvent::Hide { id: webview_id });
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
