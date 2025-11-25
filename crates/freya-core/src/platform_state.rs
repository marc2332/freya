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

#[derive(Clone)]
pub struct PlatformState {
    pub focused_accessibility_id: State<AccessibilityId>,
    pub focused_accessibility_node: State<accesskit::Node>,
    pub root_size: State<Size2D>,
    pub navigation_mode: State<NavigationMode>,
}

impl PlatformState {
    pub fn get() -> Self {
        consume_root_context()
    }
}
