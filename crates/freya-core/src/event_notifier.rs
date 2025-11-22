use std::rc::Rc;

use crate::{
    prelude::consume_root_context,
    user_event::UserEvent,
};

#[derive(Clone)]
pub struct EventNotifier {
    sender: Rc<dyn Fn(UserEvent)>,
}

impl EventNotifier {
    pub fn get() -> Self {
        consume_root_context()
    }

    pub fn new(sender: impl Fn(UserEvent) + 'static) -> Self {
        Self {
            sender: Rc::new(sender),
        }
    }

    pub fn send(&self, event: UserEvent) {
        (self.sender)(event)
    }
}
