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

    pub fn set_cursor(&self, cursor_icon: CursorIcon) {
        self.send(EventLoopMessage::SetCursorIcon(cursor_icon)).ok();
    }

    #[cfg(feature = "winit")]
    pub fn set_title(&self, title: impl Into<String>) {
        let title = title.into();
        self.with_window(move |window| {
            window.set_title(&title);
        });
    }

    #[cfg(feature = "winit")]
    pub fn with_window(&self, cb: impl FnOnce(&Window) + 'static + Send + Sync) {
        self.send(EventLoopMessage::WithWindow(Box::new(cb))).ok();
    }

    #[cfg(feature = "winit")]
    pub fn drag_window(&self) {
        self.with_window(|window| {
            window.drag_window().ok();
        });
    }

    #[cfg(feature = "winit")]
    pub fn set_maximize_window(&self, maximize: bool) {
        self.with_window(move |window| {
            window.set_maximized(maximize);
        });
    }

    #[cfg(feature = "winit")]
    pub fn toggle_maximize_window(&self) {
        self.with_window(|window| {
            window.set_maximized(!window.is_maximized());
        });
    }

    #[cfg(feature = "winit")]
    pub fn set_minimize_window(&self, minimize: bool) {
        self.with_window(move |window| {
            window.set_minimized(minimize);
        });
    }

    #[cfg(feature = "winit")]
    pub fn toggle_minimize_window(&self) {
        self.with_window(|window| {
            window.set_minimized(window.is_minimized().map(|v| !v).unwrap_or_default());
        });
    }

    #[cfg(feature = "winit")]
    pub fn toggle_fullscreen_window(&self) {
        self.with_window(|window| match window.fullscreen() {
            Some(_) => window.set_fullscreen(None),
            None => window.set_fullscreen(Some(Fullscreen::Borderless(None))),
        });
    }

    #[cfg(feature = "winit")]
    pub fn set_fullscreen_window(&self, fullscreen: bool) {
        self.with_window(move |window| {
            if fullscreen {
                window.set_fullscreen(Some(Fullscreen::Borderless(None)))
            } else {
                window.set_fullscreen(None)
            }
        });
    }

    pub fn invalidate_drawing_area(&self, area: Area) {
        self.send(EventLoopMessage::InvalidateArea(area)).ok();
    }

    pub fn request_animation_frame(&self) {
        self.send(EventLoopMessage::RequestRerender).ok();
    }

    pub fn focus(&self, strategy: AccessibilityFocusStrategy) {
        self.send(EventLoopMessage::FocusAccessibilityNode(strategy))
            .ok();
    }

    pub fn new_ticker(&self) -> Ticker {
        Ticker {
            inner: self.ticker.peek().resubscribe(),
        }
    }

    /// Closes the whole app.
    pub fn exit(&self) {
        self.send(EventLoopMessage::ExitApp).ok();
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
