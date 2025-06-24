use std::sync::Arc;

use freya_core::event_loop_messages::EventLoopMessage;
use futures_task::{
    waker,
    ArcWake,
};
use winit::event_loop::EventLoopProxy;

/// Used to enqueue a new polling for the VirtualDOM once the current one has finished
pub fn winit_waker(proxy: &EventLoopProxy<EventLoopMessage>) -> std::task::Waker {
    struct DomHandle(EventLoopProxy<EventLoopMessage>);

    impl ArcWake for DomHandle {
        fn wake_by_ref(arc_self: &Arc<Self>) {
            _ = arc_self.0.send_event(EventLoopMessage::PollVDOM);
        }
    }

    waker(Arc::new(DomHandle(proxy.clone())))
}
