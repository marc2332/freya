use std::sync::{Arc, Mutex, MutexGuard};

use dioxus_core::prelude::{consume_context, try_consume_context};
use freya_common::EventMessage;
use tokio::sync::{broadcast, mpsc::UnboundedSender};
use torin::geometry::Size2D;
use winit::{dpi::PhysicalSize, event_loop::EventLoopProxy, window::CursorIcon};

#[derive(Clone)]
pub struct UsePlatform {
    ticker: Arc<broadcast::Receiver<()>>,
    event_loop_proxy: Option<EventLoopProxy<EventMessage>>,
    platform_emitter: Option<UnboundedSender<EventMessage>>,
    platform_information: Arc<Mutex<PlatformInformation>>,
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

    /// Read information about the platform.
    /// 
    /// **Important**: This will not subscribe to any changes about the information.
    pub fn info(&self) -> MutexGuard<PlatformInformation> {
        self.platform_information.lock().unwrap()
    }
}

/// Get access to information and features of the platform.
pub fn use_platform() -> UsePlatform {
    UsePlatform {
        event_loop_proxy: try_consume_context::<EventLoopProxy<EventMessage>>(),
        platform_emitter: try_consume_context::<UnboundedSender<EventMessage>>(),
        ticker: consume_context::<Arc<broadcast::Receiver<()>>>(),
        platform_information: consume_context::<Arc<Mutex<PlatformInformation>>>(),
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
