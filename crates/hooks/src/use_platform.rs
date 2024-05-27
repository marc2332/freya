use std::sync::{Arc, Mutex};

use dioxus_core::prelude::{consume_context, try_consume_context, use_hook};
use dioxus_signals::{Readable, Signal};
use freya_common::EventMessage;
use tokio::sync::{broadcast, mpsc::UnboundedSender};
use torin::geometry::Size2D;
use winit::{
    dpi::PhysicalSize,
    event_loop::EventLoopProxy,
    window::{CursorIcon, Window},
};
use winit::window::Fullscreen;

#[derive(Clone, Copy, PartialEq)]
pub struct UsePlatform {
    ticker: Signal<Arc<broadcast::Receiver<()>>>,
    event_loop_proxy: Signal<Option<EventLoopProxy<EventMessage>>>,
    platform_emitter: Signal<Option<UnboundedSender<EventMessage>>>,
    platform_information: Signal<Arc<Mutex<PlatformInformation>>>,
}

#[derive(PartialEq, Eq, Debug)]
pub enum UsePlatformError {
    EventLoopProxyFailed,
    PlatformEmitterFailed,
}

impl UsePlatform {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        UsePlatform {
            event_loop_proxy: Signal::new(try_consume_context::<EventLoopProxy<EventMessage>>()),
            platform_emitter: Signal::new(try_consume_context::<UnboundedSender<EventMessage>>()),
            ticker: Signal::new(consume_context::<Arc<broadcast::Receiver<()>>>()),
            platform_information: Signal::new(consume_context::<Arc<Mutex<PlatformInformation>>>()),
        }
    }

    pub fn send(&self, event: EventMessage) -> Result<(), UsePlatformError> {
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

    pub fn set_cursor(&self, cursor_icon: CursorIcon) {
        self.send(EventMessage::SetCursorIcon(cursor_icon)).ok();
    }

    pub fn set_title(&self, title: impl Into<String>) {
        let title = title.into();
        self.with_window(move |window| {
            window.set_title(&title);
        });
    }

    pub fn with_window(&self, cb: impl FnOnce(&Window) + 'static + Send + Sync) {
        self.send(EventMessage::WithWindow(Box::new(cb))).ok();
    }

    pub fn drag_window(&self) {
        self.with_window(|window| {
            window.drag_window().ok();
        });
    }

    pub fn maximize_window(&self) {
        self.with_window(|window| {
           window.set_maximized(!window.is_maximized());
        });
    }

    pub fn minimize_window(&self) {
        self.with_window(|window| {
            window.set_minimized(true);
        });
    }

    pub fn fullscreen_window(&self) {
        self.with_window(|window| {
            match window.fullscreen() {
                Some(_) => window.set_fullscreen(None),
                None => window.set_fullscreen(Some(Fullscreen::Borderless(None))),
            }
        });
    }

    pub fn request_animation_frame(&self) {
        self.send(EventMessage::RequestRerender).ok();
    }

    pub fn new_ticker(&self) -> Ticker {
        Ticker {
            inner: self.ticker.peek().resubscribe(),
        }
    }

    /// Closes the whole app.
    pub fn exit(&self) {
        self.send(EventMessage::ExitApp).ok();
    }

    /// Read information about the platform.
    ///
    /// **Important**: This will not subscribe to any changes about the information.
    pub fn info(&self) -> PlatformInformation {
        self.platform_information.read().lock().unwrap().clone()
    }
}

/// Get access to information and features of the platform.
pub fn use_platform() -> UsePlatform {
    use_hook(UsePlatform::new)
}

pub struct Ticker {
    inner: broadcast::Receiver<()>,
}

impl Ticker {
    pub async fn tick(&mut self) {
        self.inner.recv().await.ok();
    }
}

/// Information about the platform.
#[derive(Clone)]
pub struct PlatformInformation {
    pub window_size: Size2D,
}

impl PlatformInformation {
    pub fn from_winit(physical_size: PhysicalSize<u32>) -> Self {
        Self {
            window_size: Size2D::new(physical_size.width as f32, physical_size.height as f32),
        }
    }

    pub fn new(window_size: Size2D) -> Self {
        Self { window_size }
    }
}
