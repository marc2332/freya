use freya_core::{
    integration::ScopeId,
    prelude::*,
};
use torin::prelude::CursorPoint;

use crate::menu::Menu;

/// Context for managing a global context menu.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     let mut show_menu = use_state(|| false);
///
///     rect()
///         .on_secondary_press(move |_| {
///             ContextMenu::open(Menu::new().child(MenuButton::new().child("Option 1")));
///             show_menu.set(true);
///         })
///         .child("Right click to open menu")
/// }
/// ```
#[derive(Clone, Copy, PartialEq)]
pub struct ContextMenu {
    pub(crate) location: State<CursorPoint>,
    pub(crate) menu: State<Option<(CursorPoint, Menu)>>,
}

impl ContextMenu {
    pub fn get() -> Self {
        match try_consume_root_context() {
            Some(rt) => rt,
            None => {
                let context_menu_state = ContextMenu {
                    location: State::create_in_scope(CursorPoint::default(), ScopeId::ROOT),
                    menu: State::create_in_scope(None, ScopeId::ROOT),
                };
                provide_context_for_scope_id(context_menu_state, ScopeId::ROOT);
                context_menu_state
            }
        }
    }

    pub fn is_open() -> bool {
        Self::get().menu.read().is_some()
    }

    pub fn open(menu: Menu) {
        let mut this = Self::get();
        this.menu.set(Some(((this.location)(), menu)));
    }

    pub fn close() {
        Self::get().menu.set(None);
    }
}
