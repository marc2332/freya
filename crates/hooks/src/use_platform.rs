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
use freya_core::{
    accessibility::AccessibilityFocusStrategy,
    event_loop_messages::EventLoopMessage,
    platform::{
        CursorIcon,
        EventLoopProxy,
        Fullscreen,
        Window,
    },
};
use tokio::sync::{
    broadcast,
    mpsc::UnboundedSender,
};
use torin::prelude::Area;

#[derive(Clone, Copy, PartialEq)]
pub struct UsePlatform {
    ticker: Signal<Arc<broadcast::Receiver<()>>>,
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
        if let Some(event_loop_proxy) = &*self.event_loop_proxy.peek() {
            event_loop_proxy
                .send_event(event)
                .map_err(|_| UsePlatformError::EventLoopProxyFailed)?;
        } else if let Some(platform_emitter) = &*self.platform_emitter.peek() {
            platform_emitter
                .send(event)
                .map_err(|_| UsePlatformError::PlatformEmitterFailed)?;
        }
        Ok(())
    }

    /// Update the [CursorIcon].
    pub fn set_cursor(&self, cursor_icon: CursorIcon) {
        self.send(EventLoopMessage::SetCursorIcon(cursor_icon)).ok();
    }

    /// Update the title of the app/window.
    pub fn set_title(&self, title: impl Into<String>) {
        let title = title.into();
        self.with_window(move |window| {
            window.set_title(&title);
        });
    }

    /// Send a callback that will be called with the [Window] once this is available to get read.
    ///
    /// For a `Sync` + `Send` version of this method you can use [UsePlatform::sender].
    pub fn with_window(&self, cb: impl FnOnce(&Window) + 'static + Send + Sync) {
        self.send(EventLoopMessage::WithWindow(Box::new(cb))).ok();
    }

    /// Shortcut for [Window::drag_window].
    pub fn drag_window(&self) {
        self.with_window(|window| {
            window.drag_window().ok();
        });
    }

    /// Shortcut for [Window::set_maximized].
    pub fn set_maximize_window(&self, maximize: bool) {
        self.with_window(move |window| {
            window.set_maximized(maximize);
        });
    }

    /// Shortcut for [Window::set_maximized].
    ///
    /// Toggles the maximized state of the [Window].
    pub fn toggle_maximize_window(&self) {
        self.with_window(|window| {
            window.set_maximized(!window.is_maximized());
        });
    }

    /// Shortcut for [Window::set_minimized].
    pub fn set_minimize_window(&self, minimize: bool) {
        self.with_window(move |window| {
            window.set_minimized(minimize);
        });
    }

    /// Shortcut for [Window::set_minimized].
    ///
    /// Toggles the minimized state of the [Window].
    pub fn toggle_minimize_window(&self) {
        self.with_window(|window| {
            window.set_minimized(window.is_minimized().map(|v| !v).unwrap_or_default());
        });
    }

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
        self.send(EventLoopMessage::InvalidateArea(area)).ok();
    }

    /// Requests a new animation frame.
    ///
    /// You most likely dont want to use this unless you are dealing animations or canvas rendering.
    pub fn request_animation_frame(&self) {
        self.send(EventLoopMessage::RequestRerender).ok();
    }

    /// Request focus with a given [AccessibilityFocusStrategy].
    pub fn request_focus(&self, strategy: AccessibilityFocusStrategy) {
        self.send(EventLoopMessage::FocusAccessibilityNode(strategy))
            .ok();
    }

    /// Create a new frame [Ticker].
    ///
    /// You most likely dont want to use this unless you are dealing animations or canvas rendering.
    pub fn new_ticker(&self) -> Ticker {
        Ticker {
            inner: self.ticker.peek().resubscribe(),
        }
    }

    /// Closes the whole app.
    pub fn exit(&self) {
        self.send(EventLoopMessage::ExitApp).ok();
    }

    /// Get a [PlatformSender] that you can use to send events from other threads.
    pub fn sender(&self) -> PlatformSender {
        PlatformSender {
            event_loop_proxy: self.event_loop_proxy.read().clone(),
            platform_emitter: self.platform_emitter.read().clone(),
        }
    }
}

#[derive(Clone)]
pub struct PlatformSender {
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
    pub fn with_window(&self, cb: impl FnOnce(&Window) + 'static + Send + Sync) {
        self.send(EventLoopMessage::WithWindow(Box::new(cb))).ok();
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
