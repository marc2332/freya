use std::rc::Rc;

use torin::prelude::Size2D;

use crate::{
    accessibility::id::AccessibilityId,
    prelude::{
        State,
        consume_root_context,
    },
};

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug, Hash)]
pub enum NavigationMode {
    #[default]
    NotKeyboard,
    Keyboard,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum PreferredTheme {
    #[default]
    Light,
    Dark,
}

use crate::user_event::UserEvent;

#[derive(Clone)]
pub struct Platform {
    pub focused_accessibility_id: State<AccessibilityId>,
    pub focused_accessibility_node: State<accesskit::Node>,
    pub root_size: State<Size2D>,
    pub navigation_mode: State<NavigationMode>,
    pub preferred_theme: State<PreferredTheme>,
    pub sender: Rc<dyn Fn(UserEvent)>,
}

impl Platform {
    pub fn get() -> Self {
        consume_root_context()
    }

    pub fn send(&self, event: UserEvent) {
        (self.sender)(event)
    }
}
