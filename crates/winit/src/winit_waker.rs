use std::sync::Arc;

use freya_core::event_loop_messages::{
    EventLoopMessage,
    EventLoopMessageAction,
};
use futures_task::{
    waker,
    ArcWake,
};
use winit::{
    event_loop::EventLoopProxy,
    window::WindowId,
};

/// Used to enqueue a new polling for the VirtualDOM once the current one has finished
pub fn winit_waker(proxy: &EventLoopProxy<EventLoopMessage>, id: WindowId) -> std::task::Waker {
    struct DomHandle(EventLoopProxy<EventLoopMessage>, WindowId);

    impl ArcWake for DomHandle {
        fn wake_by_ref(arc_self: &Arc<Self>) {
            _ = arc_self.0.send_event(EventLoopMessage {
                window_id: Some(arc_self.1),
                action: EventLoopMessageAction::PollVDOM,
            });
        }
    }

    waker(Arc::new(DomHandle(proxy.clone(), id)))
}
