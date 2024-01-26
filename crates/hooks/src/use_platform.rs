use std::sync::Arc;

use dioxus_core::prelude::{consume_context, try_consume_context};
use freya_common::EventMessage;
use tokio::sync::{broadcast, mpsc::UnboundedSender};
use winit::{event_loop::EventLoopProxy, window::CursorIcon};

#[derive(Clone)]
pub struct UsePlatform {
    ticker: Arc<broadcast::Receiver<()>>,
    event_loop_proxy: Option<EventLoopProxy<EventMessage>>,
    platform_emitter: Option<UnboundedSender<EventMessage>>,
}

impl PartialEq for UsePlatform {
    fn eq(&self, _other: &Self) -> bool {
        // The provided platform integrations will never change
        // during when running the app, so it is safe to assume their equality.
        true
    }
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
}

pub fn use_platform() -> UsePlatform {
    UsePlatform {
        event_loop_proxy: try_consume_context::<EventLoopProxy<EventMessage>>(),
        platform_emitter: try_consume_context::<UnboundedSender<EventMessage>>(),
        ticker: consume_context::<Arc<broadcast::Receiver<()>>>(),
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
