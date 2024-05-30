use std::sync::Arc;

use dioxus_core::prelude::{consume_context, try_consume_context, use_hook};
use dioxus_signals::{Readable, Signal};
use freya_common::EventMessage;
use freya_core::prelude::PlatformInformation;
use tokio::sync::{broadcast, mpsc::UnboundedSender};
use winit::{event_loop::EventLoopProxy, window::CursorIcon};

#[derive(Clone, Copy, PartialEq)]
pub struct UsePlatform {
    ticker: Signal<Arc<broadcast::Receiver<()>>>,
    event_loop_proxy: Signal<Option<EventLoopProxy<EventMessage>>>,
    platform_emitter: Signal<Option<UnboundedSender<EventMessage>>>,
    platform_information: Signal<PlatformInformation>,
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
            platform_information: consume_context::<Signal<PlatformInformation>>(),
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

    pub fn drag_window(&self) {
        self.send(EventMessage::DragWindow).ok();
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

    /// Get the information about the platform.
    pub fn info(&self) -> Signal<PlatformInformation> {
        self.platform_information
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
