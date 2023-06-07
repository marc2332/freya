use dioxus_core::ScopeState;
use freya_common::EventMessage;
use tokio::sync::mpsc::UnboundedSender;
use winit::event_loop::EventLoopProxy;

#[derive(Clone)]
pub struct UsePlatform {
    event_loop_proxy: Option<EventLoopProxy<EventMessage>>,
    platform_emitter: Option<UnboundedSender<EventMessage>>,
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
}

pub fn use_platform(cx: &ScopeState) -> UsePlatform {
    UsePlatform {
        event_loop_proxy: cx.consume_context::<EventLoopProxy<EventMessage>>(),
        platform_emitter: cx.consume_context::<UnboundedSender<EventMessage>>(),
    }
}
