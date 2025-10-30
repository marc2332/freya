use cursor_icon::CursorIcon;

use crate::{
    event_notifier::EventNotifier,
    user_event::UserEvent,
};

pub struct Cursor;

impl Cursor {
    pub fn set(cursor_icon: CursorIcon) {
        let event_notifier = EventNotifier::get();
        event_notifier.send(UserEvent::SetCursorIcon(cursor_icon));
    }
}
