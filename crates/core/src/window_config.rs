use std::sync::Arc;

use dioxus_core::{
    fc_to_builder,
    Element,
};
use dioxus_core_macro::rsx;
use dioxus_signals::{
    GlobalSignal,
    Readable,
};
use winit::window::{
    Icon,
    Window,
    WindowAttributes,
};

use crate::{
    event_loop_messages::{
        EventLoopMessage,
        EventLoopMessageAction,
    },
    parsing::Parse,
    values::Color,
};

pub type WindowCallback = Box<dyn FnOnce(&mut Window) + Send + Sync>;
pub type OnCloseCallback = Box<dyn FnMut(&mut Window) -> OnCloseResponse + Send + Sync>;
pub type WindowBuilderHook = Box<dyn FnOnce(WindowAttributes) -> WindowAttributes + Send + Sync>;

impl From<accesskit_winit::Event> for EventLoopMessage {
    fn from(value: accesskit_winit::Event) -> Self {
        Self {
            window_id: Some(value.window_id),
            action: EventLoopMessageAction::Accessibility(value.window_event),
        }
    }
}

#[derive(PartialEq)]
pub enum OnCloseResponse {
    Close,
    NotClose,
}

/// Configuration for a Window.
pub struct WindowConfig {
    pub app: Arc<dyn Fn() -> Element + Send + Sync>,
    /// Size of the Window.
    pub size: (f64, f64),
    /// Minimum size of the Window.
    pub min_size: Option<(f64, f64)>,
    /// Maximum size of the Window.
    pub max_size: Option<(f64, f64)>,
    /// Enable Window decorations.
    pub decorations: bool,
    /// Title for the Window.
    pub title: &'static str,
    /// Make the Window transparent or not.
    pub transparent: bool,
    /// Background color of the Window.
    pub background: Color,
    /// Window visibility. Default to `true`.
    pub visible: bool,
    /// The Icon of the Window.
    pub icon: Option<Icon>,
    /// Setup callback.
    pub on_setup: Option<WindowCallback>,
    /// When window gets a close request.
    pub on_close: Option<OnCloseCallback>,
    /// Hook function called with the Window Attributes.
    pub window_attributes_hook: Option<WindowBuilderHook>,
    /// Max resource in bytes to be used by the GPU. Defaults to automatic.
    pub max_gpu_resources_bytes: Option<usize>,
}

impl WindowConfig {
    /// Create a window with the given app.
    pub fn new(app: fn() -> Element) -> Self {
        #[allow(non_snake_case)]
        let App = app;
        Self::new_with_defaults(Arc::new(move || rsx!(App {})))
    }

    /// Create a window with the given app and some props to pass to it.
    pub fn new_with_props<T: dioxus_core::Properties + Sync + Send>(
        app: fn(T) -> Element,
        props: T,
    ) -> Self {
        #[allow(non_snake_case)]
        let App = app;
        Self::new_with_defaults(Arc::new(move || rsx!(App { ..props.clone() })))
    }

    fn new_with_defaults(app: Arc<dyn Fn() -> Element + Send + Sync>) -> Self {
        Self {
            app,
            size: (700.0, 500.0),
            min_size: None,
            max_size: None,
            decorations: true,
            title: "Freya",
            transparent: false,
            background: Color::WHITE,
            visible: true,
            icon: None,
            on_setup: None,
            on_close: None,
            window_attributes_hook: None,
            max_gpu_resources_bytes: None,
        }
    }

    /// Specify a Window size.
    pub fn with_size(mut self, width: f64, height: f64) -> Self {
        self.size = (width, height);
        self
    }

    /// Specify a minimum Window size.
    pub fn with_min_size(mut self, min_width: f64, min_height: f64) -> Self {
        self.min_size = Some((min_width, min_height));
        self
    }

    /// Specify a maximum Window size.
    pub fn with_max_size(mut self, max_width: f64, max_height: f64) -> Self {
        self.max_size = Some((max_width, max_height));
        self
    }

    /// Whether the Window will have decorations or not.
    pub fn with_decorations(mut self, decorations: bool) -> Self {
        self.decorations = decorations;
        self
    }

    /// Specify the Window title.
    pub fn with_title(mut self, title: &'static str) -> Self {
        self.title = title;
        self
    }

    /// Make the Window transparent or not.
    pub fn with_transparency(mut self, transparency: bool) -> Self {
        self.transparent = transparency;
        self
    }
    /// Specify the max resources to be cached for the GPU, in bytes.
    pub fn with_max_gpu_resources_bytes(mut self, max_gpu_resources_bytes: usize) -> Self {
        self.max_gpu_resources_bytes = Some(max_gpu_resources_bytes);
        self
    }
    /// Specify the Window background color.
    pub fn with_background(mut self, background: &str) -> Self {
        self.background = Color::parse(background).unwrap_or(Color::WHITE);
        self
    }

    /// Specify the Window visibility at launch.
    pub fn with_visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }
    /// Specify the Window icon.
    pub fn with_icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Register a callback that will be executed when the window is created.
    pub fn on_setup(mut self, callback: impl FnOnce(&mut Window) + 'static + Send + Sync) -> Self {
        self.on_setup = Some(Box::new(callback));
        self
    }

    /// Register a callback that will be executed when the window is closed.
    pub fn on_close(
        mut self,
        callback: impl FnMut(&mut Window) -> OnCloseResponse + 'static + Send + Sync,
    ) -> Self {
        self.on_close = Some(Box::new(callback));
        self
    }

    /// Register a Window Attributes hook.
    pub fn with_window_attributes(
        mut self,
        window_attributes_hook: impl FnOnce(WindowAttributes) -> WindowAttributes
            + 'static
            + Send
            + Sync,
    ) -> Self {
        self.window_attributes_hook = Some(Box::new(window_attributes_hook));
        self
    }
}
