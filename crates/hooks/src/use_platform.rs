use std::sync::{Arc, Mutex, MutexGuard};

use dioxus_core::ScopeState;
use freya_common::EventMessage;
use tokio::sync::{broadcast, mpsc::UnboundedSender};
use winit::{dpi::PhysicalSize, event_loop::EventLoopProxy, window::CursorIcon};

#[derive(Clone)]
pub struct UsePlatform {
    ticker: Arc<broadcast::Receiver<()>>,
    event_loop_proxy: Option<EventLoopProxy<EventMessage>>,
    platform_emitter: Option<UnboundedSender<EventMessage>>,
    platform_information: Arc<Mutex<PlatformInformation>>,
}

#[derive(PartialEq, Eq, Debug)]
pub enum UsePlatformError {
    EventLoopProxyFailed,
    PlatformEmitterFailed,
}

impl UsePlatform {
    pub fn send(&self, event: EventMessage) -> Result<(), UsePlatformError> {
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

    pub fn set_cursor(&self, cursor_icon: CursorIcon) {
        self.send(EventMessage::SetCursorIcon(cursor_icon)).ok();
    }

    pub fn request_animation_frame(&self) {
        self.send(EventMessage::RequestRerender).ok();
    }

    pub fn new_ticker(&self) -> Ticker {
        Ticker {
            inner: self.ticker.resubscribe(),
        }
    }

    pub fn info(&self) -> MutexGuard<PlatformInformation> {
        self.platform_information.lock().unwrap()
    }
}

pub fn use_platform(cx: &ScopeState) -> UsePlatform {
    UsePlatform {
        event_loop_proxy: cx.consume_context::<EventLoopProxy<EventMessage>>(),
        platform_emitter: cx.consume_context::<UnboundedSender<EventMessage>>(),
        ticker: cx
            .consume_context::<Arc<broadcast::Receiver<()>>>()
            .expect("This is not expected, and likely a bug. Please, report it."),
        platform_information: cx
            .consume_context::<Arc<Mutex<PlatformInformation>>>()
            .expect("This is not expected, and likely a bug. Please, report it."),
    }
}

pub struct Ticker {
    inner: broadcast::Receiver<()>,
}

impl Ticker {
    pub async fn tick(&mut self) {
        self.inner.recv().await.ok();
    }
}

#[derive(Clone)]
pub struct PlatformInformation {
    pub window_size: PhysicalSize<u32>,
}
