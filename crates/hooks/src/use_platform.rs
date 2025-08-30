use std::sync::Arc;

use dioxus_core::{
    prelude::{
        consume_context,
        provide_root_context,
        try_consume_context,
        use_hook,
    },
    ScopeId,
};
use dioxus_signals::{
    Readable,
    Signal,
};
#[cfg(feature = "winit")]
use freya_core::window_config::WindowConfig;
use freya_core::{
    accessibility::AccessibilityFocusStrategy,
    event_loop_messages::{
        EventLoopMessage,
        EventLoopMessageAction,
    },
    platform::CursorIcon,
};
use tokio::sync::{
    broadcast,
    mpsc::UnboundedSender,
};
use torin::prelude::Area;
#[cfg(feature = "winit")]
pub use winit::{
    event_loop::EventLoopProxy,
    window::{
        Fullscreen,
        Window,
        WindowId,
    },
};

#[derive(Clone, Copy, PartialEq)]
pub struct UsePlatform {
    ticker: Signal<Arc<broadcast::Receiver<()>>>,
    #[cfg(feature = "winit")]
    event_loop_proxy: Signal<Option<EventLoopProxy<EventLoopMessage>>>,
    platform_emitter: Signal<Option<UnboundedSender<EventLoopMessage>>>,
}

#[derive(PartialEq, Eq, Debug)]
pub enum UsePlatformError {
    EventLoopProxyFailed,
    PlatformEmitterFailed,
}

impl UsePlatform {
    /// Get the current [UsePlatform].
    pub fn current() -> Self {
        match try_consume_context() {
            Some(p) => p,
            None => provide_root_context(UsePlatform {
                #[cfg(feature = "winit")]
                event_loop_proxy: Signal::new_in_scope(
                    try_consume_context::<EventLoopProxy<EventLoopMessage>>(),
                    ScopeId::ROOT,
                ),
                platform_emitter: Signal::new_in_scope(
                    try_consume_context::<UnboundedSender<EventLoopMessage>>(),
                    ScopeId::ROOT,
                ),
                ticker: Signal::new_in_scope(
                    consume_context::<Arc<broadcast::Receiver<()>>>(),
                    ScopeId::ROOT,
                ),
            }),
        }
    }

    /// You most likely dont want to use this method. Check the other methods in [UsePlatform].
    pub fn send(&self, event: EventLoopMessage) -> Result<(), UsePlatformError> {
        #[cfg(feature = "winit")]
        if let Some(event_loop_proxy) = &*self.event_loop_proxy.peek() {
            return event_loop_proxy
                .send_event(event)
                .map_err(|_| UsePlatformError::EventLoopProxyFailed);
        }
        if let Some(platform_emitter) = &*self.platform_emitter.peek() {
            platform_emitter
                .send(event)
                .map_err(|_| UsePlatformError::PlatformEmitterFailed)?;
        }
        Ok(())
    }

    /// Update the [CursorIcon].
    pub fn set_cursor(&self, cursor_icon: CursorIcon) {
        self.send_app_event(EventLoopMessageAction::SetCursorIcon(cursor_icon));
    }

    #[cfg(feature = "winit")]
    /// Update the title of the app/window.
    pub fn set_title(&self, title: impl Into<String>) {
        let title = title.into();
        self.with_window(move |window| {
            window.set_title(&title);
        });
    }

    #[cfg(feature = "winit")]
    /// Send a callback that will be called with the [Window] once this is available to get read.
    ///
    /// For a `Sync` + `Send` version of this method you can use [UsePlatform::sender].
    pub fn with_window(&self, cb: impl FnOnce(&Window) + 'static + Send + Sync) {
        self.send_app_event(EventLoopMessageAction::WithWindow(Box::new(cb)));
    }

    #[cfg(feature = "winit")]
    /// Shortcut for [Window::drag_window].
    pub fn drag_window(&self) {
        self.with_window(|window| {
            window.drag_window().ok();
        });
    }

    #[cfg(feature = "winit")]
    /// Shortcut for [Window::set_maximized].
    pub fn set_maximize_window(&self, maximize: bool) {
        self.with_window(move |window| {
            window.set_maximized(maximize);
        });
    }

    #[cfg(feature = "winit")]
    /// Shortcut for [Window::set_maximized].
    ///
    /// Toggles the maximized state of the [Window].
    pub fn toggle_maximize_window(&self) {
        self.with_window(|window| {
            window.set_maximized(!window.is_maximized());
        });
    }

    #[cfg(feature = "winit")]
    /// Shortcut for [Window::set_minimized].
    pub fn set_minimize_window(&self, minimize: bool) {
        self.with_window(move |window| {
            window.set_minimized(minimize);
        });
    }

    #[cfg(feature = "winit")]
    /// Shortcut for [Window::set_minimized].
    ///
    /// Toggles the minimized state of the [Window].
    pub fn toggle_minimize_window(&self) {
        self.with_window(|window| {
            window.set_minimized(window.is_minimized().map(|v| !v).unwrap_or_default());
        });
    }

    #[cfg(feature = "winit")]
    /// Shortcut for [Window::set_fullscreen].
    pub fn set_fullscreen_window(&self, fullscreen: bool) {
        self.with_window(move |window| {
            if fullscreen {
                window.set_fullscreen(Some(Fullscreen::Borderless(None)))
            } else {
                window.set_fullscreen(None)
            }
        });
    }

    #[cfg(feature = "winit")]
    /// Shortcut for [Window::set_fullscreen].
    ///
    /// Toggles the fullscreen state of the [Window].
    pub fn toggle_fullscreen_window(&self) {
        self.with_window(|window| match window.fullscreen() {
            Some(_) => window.set_fullscreen(None),
            None => window.set_fullscreen(Some(Fullscreen::Borderless(None))),
        });
    }

    /// Invalidates a drawing area.
    ///
    /// You most likely dont want to use this unless you are dealing with advanced images/canvas rendering.
    pub fn invalidate_drawing_area(&self, area: Area) {
        self.send_app_event(EventLoopMessageAction::InvalidateArea(area));
    }

    /// Requests a new animation frame.
    ///
    /// You most likely dont want to use this unless you are dealing animations or canvas rendering.
    pub fn request_animation_frame(&self) {
        self.send_app_event(EventLoopMessageAction::RequestRerender);
    }

    /// Request focus with a given [AccessibilityFocusStrategy].
    pub fn request_focus(&self, strategy: AccessibilityFocusStrategy) {
        self.send_app_event(EventLoopMessageAction::FocusAccessibilityNode(strategy));
    }

    /// Create a new frame [Ticker].
    ///
    /// You most likely dont want to use this unless you are dealing animations or canvas rendering.
    pub fn new_ticker(&self) -> Ticker {
        Ticker {
            inner: self.ticker.peek().resubscribe(),
        }
    }

    /// Closes the window.
    pub fn close_window(&self) {
        self.send_app_event(EventLoopMessageAction::CloseWindow);
    }

    /// Get a [PlatformSender] that you can use to send events from other threads.
    pub fn sender(&self) -> PlatformSender {
        PlatformSender {
            #[cfg(feature = "winit")]
            window_id: try_consume_context(),
            event_loop_proxy: self.event_loop_proxy.read().clone(),
            platform_emitter: self.platform_emitter.read().clone(),
        }
    }

    #[cfg(feature = "winit")]
    pub fn new_window(&self, window_config: WindowConfig) {
        self.send_app_event(EventLoopMessageAction::NewWindow(window_config));
    }

    pub fn send_app_event(&self, action: EventLoopMessageAction) {
        self.send(EventLoopMessage {
            #[cfg(feature = "winit")]
            window_id: try_consume_context(),
            action,
        })
        .ok();
    }
}

#[derive(Clone)]
pub struct PlatformSender {
    #[cfg(feature = "winit")]
    window_id: Option<WindowId>,
    event_loop_proxy: Option<EventLoopProxy<EventLoopMessage>>,
    platform_emitter: Option<UnboundedSender<EventLoopMessage>>,
}

impl PlatformSender {
    pub fn send(&self, event: EventLoopMessage) -> Result<(), UsePlatformError> {
        if let Some(event_loop_proxy) = &self.event_loop_proxy {
            event_loop_proxy
                .send_event(event)
                .map_err(|_| UsePlatformError::EventLoopProxyFailed)?;
        } else if let Some(platform_emitter) = &self.platform_emitter {
            platform_emitter
                .send(event)
                .map_err(|_| UsePlatformError::PlatformEmitterFailed)?;
        }
        Ok(())
    }

    /// Send a callback that will be called with the [Window] once this is available to get read.
    #[cfg(feature = "winit")]
    pub fn with_window(&self, cb: impl FnOnce(&Window) + 'static + Send + Sync) {
        self.send(EventLoopMessage {
            window_id: self.window_id,
            action: EventLoopMessageAction::WithWindow(Box::new(cb)),
        })
        .ok();
    }
}

/// Get access to information and features of the platform.
pub fn use_platform() -> UsePlatform {
    use_hook(UsePlatform::current)
}

pub struct Ticker {
    inner: broadcast::Receiver<()>,
}

impl Ticker {
    pub async fn tick(&mut self) {
        self.inner.recv().await.ok();
    }
}
