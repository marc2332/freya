use std::sync::Arc;

use freya_common::EventMessage;
use futures_task::{waker, ArcWake};
use winit::event_loop::EventLoopProxy;

pub fn winit_waker(proxy: &EventLoopProxy<EventMessage>) -> std::task::Waker {
    struct DomHandle(EventLoopProxy<EventMessage>);

    unsafe impl Send for DomHandle {}
    unsafe impl Sync for DomHandle {}

    impl ArcWake for DomHandle {
        fn wake_by_ref(arc_self: &Arc<Self>) {
            _ = arc_self.0.send_event(EventMessage::PollVDOM);
        }
    }

    waker(Arc::new(DomHandle(proxy.clone())))
}
