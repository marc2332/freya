use freya_core::{
    prelude::{
        EventNotifier,
        UserEvent,
    },
    user_event::SingleThreadErasedEvent,
};
use winit::window::WindowId;

use crate::{
    config::WindowConfig,
    renderer::NativeWindowErasedEventAction,
};

pub trait WinitEventNotifierExt {
    fn launch_window(&self, window_config: WindowConfig) -> impl Future<Output = WindowId>;

    fn close_window(&self, window_id: WindowId);
}

impl WinitEventNotifierExt for EventNotifier {
    async fn launch_window(&self, window_config: WindowConfig) -> WindowId {
        let (tx, rx) = futures_channel::oneshot::channel();
        self.send(UserEvent::Erased(SingleThreadErasedEvent(Box::new(
            NativeWindowErasedEventAction::LaunchWindow {
                window_config,
                ack: tx,
            },
        ))));
        rx.await.expect("Failed to create Window")
    }

    fn close_window(&self, window_id: WindowId) {
        self.send(UserEvent::Erased(SingleThreadErasedEvent(Box::new(
            NativeWindowErasedEventAction::CloseWindow(window_id),
        ))));
    }
}
