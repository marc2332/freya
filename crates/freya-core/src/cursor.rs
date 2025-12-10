use cursor_icon::CursorIcon;

use crate::{
    platform::Platform,
    user_event::UserEvent,
};

pub struct Cursor;

impl Cursor {
    pub fn set(cursor_icon: CursorIcon) {
        let platform = Platform::get();
        platform.send(UserEvent::SetCursorIcon(cursor_icon));
    }
}
