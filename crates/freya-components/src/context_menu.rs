use freya_core::{
    integration::ScopeId,
    prelude::*,
};
use torin::prelude::CursorPoint;

use crate::menu::Menu;

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum ContextMenuCloseRequest {
    None,
    Pending,
}

/// Context for managing a global context menu.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     rect()
///         .on_secondary_down(move |e: Event<PressEventData>| {
///             ContextMenu::open_from_event(&e, Menu::new().child(MenuButton::new().child("Option 1")));
///         })
///         .child("Right click to open menu")
/// }
/// ```
#[derive(Clone, Copy, PartialEq)]
pub struct ContextMenu {
    pub(crate) location: State<CursorPoint>,
    pub(crate) menu: State<Option<(CursorPoint, Menu)>>,
    pub(crate) close_request: State<ContextMenuCloseRequest>,
}

impl ContextMenu {
    pub fn get() -> Self {
        match try_consume_root_context() {
            Some(rt) => rt,
            None => {
                let context_menu_state = ContextMenu {
                    location: State::create_in_scope(CursorPoint::default(), ScopeId::ROOT),
                    menu: State::create_in_scope(None, ScopeId::ROOT),
                    close_request: State::create_in_scope(
                        ContextMenuCloseRequest::None,
                        ScopeId::ROOT,
                    ),
                };
                provide_context_for_scope_id(context_menu_state, ScopeId::ROOT);
                context_menu_state
            }
        }
    }

    pub fn is_open() -> bool {
        Self::get().menu.read().is_some()
    }

    /// Open the context menu with the given menu.
    /// Prefer using [`ContextMenu::open_from_event`] instead as it correctly handles
    /// the close behavior based on the source event.
    pub fn open(menu: Menu) {
        let mut this = Self::get();
        this.menu.set(Some(((this.location)(), menu)));
        this.close_request.set(ContextMenuCloseRequest::None);
    }

    /// Open the context menu with the given menu, using the source event to determine
    /// the close behavior. When opened from a primary button (left click) press event,
    /// the first close request is consumed to prevent the menu from closing immediately.
    /// When opened from a secondary button (right click) down event, the menu can be
    /// closed with a single click.
    pub fn open_from_event(event: &Event<PressEventData>, menu: Menu) {
        let mut this = Self::get();
        this.menu.set(Some(((this.location)(), menu)));

        let close_request = match event.data() {
            PressEventData::Mouse(mouse) if mouse.button == Some(MouseButton::Left) => {
                ContextMenuCloseRequest::Pending
            }
            _ => ContextMenuCloseRequest::None,
        };
        this.close_request.set(close_request);
    }

    pub fn close() {
        Self::get().menu.set(None);
    }
}
