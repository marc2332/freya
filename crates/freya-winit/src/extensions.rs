use freya_core::{
    prelude::{
        EventNotifier,
        UserEvent,
    },
    user_event::SingleThreadErasedEvent,
};

use crate::config::WindowConfig;

pub trait WinitEventNotifierExt {
    fn launch_window(&self, window_config: WindowConfig);
}

impl WinitEventNotifierExt for EventNotifier {
    fn launch_window(&self, window_config: WindowConfig) {
        self.send(UserEvent::Erased(SingleThreadErasedEvent(Box::new(
            window_config,
        ))));
    }
}
