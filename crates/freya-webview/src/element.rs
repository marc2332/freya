//! WebView element for rendering web content in Freya.

use std::{
    any::Any,
    borrow::Cow,
    collections::HashMap,
    rc::Rc,
};

use freya_core::{
    data::{
        AccessibilityData,
        EffectData,
        LayoutData,
        StyleState,
        TextStyleData,
    },
    diff_key::DiffKey,
    element::{
        Element,
        ElementExt,
        EventHandlerType,
    },
    events::name::EventName,
    layers::Layer,
    prelude::{
        AccessibilityExt,
        ContainerExt,
        EventHandlersExt,
        KeyExt,
        LayerExt,
        LayoutExt,
        MaybeExt,
    },
    tree::DiffModifies,
};
use rustc_hash::FxHashMap;

use crate::registry::{
    WebViewConfig,
    WebViewId,
};

/// WebView element data.
#[derive(Clone)]
pub struct WebViewElement {
    pub accessibility: AccessibilityData,
    pub layout: LayoutData,
    pub event_handlers: FxHashMap<EventName, EventHandlerType>,
    pub webview_id: WebViewId,
    pub config: WebViewConfig,
    pub relative_layer: Layer,
    pub effect: Option<EffectData>,
}

impl PartialEq for WebViewElement {
    fn eq(&self, other: &Self) -> bool {
        self.accessibility == other.accessibility
            && self.layout == other.layout
            && self.webview_id == other.webview_id
            && self.config == other.config
            && self.relative_layer == other.relative_layer
            && self.effect == other.effect
    }
}

impl ElementExt for WebViewElement {
    fn changed(&self, other: &Rc<dyn ElementExt>) -> bool {
        let Some(webview) = (other.as_ref() as &dyn Any).downcast_ref::<WebViewElement>() else {
            return false;
        };
        self != webview
    }

    fn diff(&self, other: &Rc<dyn ElementExt>) -> DiffModifies {
        let Some(webview) = (other.as_ref() as &dyn Any).downcast_ref::<WebViewElement>() else {
            return DiffModifies::all();
        };

        let mut diff = DiffModifies::empty();

        if self.accessibility != webview.accessibility {
            diff.insert(DiffModifies::ACCESSIBILITY);
        }

        if self.relative_layer != webview.relative_layer {
            diff.insert(DiffModifies::LAYER);
        }

        if self.layout != webview.layout || self.config != webview.config {
            diff.insert(DiffModifies::LAYOUT);
        }

        if self.effect != webview.effect {
            diff.insert(DiffModifies::EFFECT);
        }

        diff
    }

    fn layout(&'_ self) -> Cow<'_, LayoutData> {
        Cow::Borrowed(&self.layout)
    }

    fn effect(&'_ self) -> Option<Cow<'_, EffectData>> {
        self.effect.as_ref().map(Cow::Borrowed)
    }

    fn style(&'_ self) -> Cow<'_, StyleState> {
        Cow::Owned(StyleState::default())
    }

    fn text_style(&'_ self) -> Cow<'_, TextStyleData> {
        Cow::Owned(TextStyleData::default())
    }

    fn accessibility(&'_ self) -> Cow<'_, AccessibilityData> {
        Cow::Borrowed(&self.accessibility)
    }

    fn layer(&self) -> Layer {
        self.relative_layer
    }

    fn should_measure_inner_children(&self) -> bool {
        false
    }

    fn events_handlers(&'_ self) -> Option<Cow<'_, FxHashMap<EventName, EventHandlerType>>> {
        Some(Cow::Borrowed(&self.event_handlers))
    }
}

impl From<WebView> for Element {
    fn from(value: WebView) -> Self {
        Element::Element {
            key: value.key,
            element: Rc::new(value.element),
            elements: vec![],
        }
    }
}

/// WebView element builder.
///
/// Use [`webview()`] to create a new WebView element.
pub struct WebView {
    pub(crate) key: DiffKey,
    pub(crate) element: WebViewElement,
}

impl WebView {
    /// Try to downcast an ElementExt to a WebViewElement.
    pub fn try_downcast(element: &dyn ElementExt) -> Option<WebViewElement> {
        (element as &dyn Any)
            .downcast_ref::<WebViewElement>()
            .cloned()
    }

    /// Set the URL to load in the WebView.
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.element.config.url = url.into();
        self
    }

    /// Set whether the WebView background should be transparent.
    pub fn transparent(mut self, transparent: bool) -> Self {
        self.element.config.transparent = transparent;
        self
    }

    /// Set a custom user agent string.
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.element.config.user_agent = Some(user_agent.into());
        self
    }
}

impl KeyExt for WebView {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl EventHandlersExt for WebView {
    fn get_event_handlers(&mut self) -> &mut FxHashMap<EventName, EventHandlerType> {
        &mut self.element.event_handlers
    }
}

impl LayoutExt for WebView {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.element.layout
    }
}

impl ContainerExt for WebView {}

impl AccessibilityExt for WebView {
    fn get_accessibility_data(&mut self) -> &mut AccessibilityData {
        &mut self.element.accessibility
    }
}

impl MaybeExt for WebView {}

impl LayerExt for WebView {
    fn get_layer(&mut self) -> &mut Layer {
        &mut self.element.relative_layer
    }
}

/// Create a new WebView element.
///
/// # Example
///
/// ```rust,no_run
/// use freya::prelude::*;
/// use freya_webview::prelude::*;
///
/// fn app() -> impl IntoElement {
///     webview("https://example.com")
///         .width(Size::fill())
///         .height(Size::fill())
/// }
/// ```
pub fn webview(url: impl Into<String>) -> WebView {
    let mut accessibility = AccessibilityData::default();
    accessibility.builder.set_role(accesskit::Role::WebView);

    let webview_id = WebViewId::new();
    let config = WebViewConfig {
        url: url.into(),
        transparent: false,
        user_agent: None,
    };

    WebView {
        key: DiffKey::None,
        element: WebViewElement {
            accessibility,
            layout: LayoutData::default(),
            event_handlers: HashMap::default(),
            webview_id,
            config,
            relative_layer: Layer::default(),
            effect: None,
        },
    }
}
