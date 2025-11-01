use std::rc::Rc;

use crate::{
    prelude::try_consume_root_context,
    user_event::UserEvent,
};

#[derive(Clone)]
pub struct EventNotifier {
    sender: Rc<dyn Fn(UserEvent)>,
}

impl EventNotifier {
    pub fn get() -> Self {
        try_consume_root_context::<Self>().unwrap()
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
