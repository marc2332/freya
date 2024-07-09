use std::sync::Arc;

use freya_common::EventMessage;
use futures_task::{waker, ArcWake};
use winit::event_loop::EventLoopProxy;

/// Used to enqueue a new polling for the VirtualDOM once the current one has finished
pub fn winit_waker(proxy: &EventLoopProxy<EventMessage>) -> std::task::Waker {
    struct DomHandle(EventLoopProxy<EventMessage>);

    impl ArcWake for DomHandle {
        fn wake_by_ref(arc_self: &Arc<Self>) {
            _ = arc_self.0.send_event(EventMessage::PollVDOM);
        }
    }

    waker(Arc::new(DomHandle(proxy.clone())))
}
