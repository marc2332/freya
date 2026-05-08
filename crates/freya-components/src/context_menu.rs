use freya_core::{
    integration::ScopeId,
    layers::Layer,
    prelude::*,
};
use torin::prelude::{
    CursorPoint,
    Position,
};

use crate::menu::Menu;

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum ContextMenuCloseRequest {
    None,
    Pending,
}

/// Context for managing a global context menu.
///
/// Requires a [`ContextMenuViewer`] mounted somewhere in the app tree.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     rect().child(ContextMenuViewer::new()).child(
///         rect()
///             .on_secondary_down(move |e: Event<PressEventData>| {
///                 ContextMenu::open_from_event(
///                     &e,
///                     Menu::new().child(MenuButton::new().child("Option 1")),
///                 );
///             })
///             .child("Right click to open menu"),
///     )
/// }
/// ```
#[derive(Clone, Copy, PartialEq)]
pub struct ContextMenu {
    pub(crate) location: State<CursorPoint>,
    pub(crate) menu: State<Option<(CursorPoint, Menu)>>,
    pub(crate) close_request: State<ContextMenuCloseRequest>,
}

impl ContextMenu {
    /// Returns the global [`ContextMenu`] state.
    ///
    /// # Panics
    ///
    /// Panics if no [`ContextMenuViewer`] has been mounted in the app tree.
    pub fn get() -> Self {
        try_consume_root_context().expect(
            "ContextMenu requires a `ContextMenuViewer` mounted in the app tree. \
             Add `ContextMenuViewer::new()` somewhere inside your `app` component.",
        )
    }

    pub fn is_open() -> bool {
        try_consume_root_context::<Self>().is_some_and(|c| c.menu.read().is_some())
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
        if let Some(mut this) = try_consume_root_context::<Self>() {
            this.menu.set(None);
        }
    }
}

/// Provides the [`ContextMenu`] state and renders the floating menu overlay.
///
/// Mount this component once inside your `app` component to enable [`ContextMenu`].
/// Placing it inside the app tree (rather than above it) means the rendered menu
/// inherits any styling applied to the app's root element, e.g. `font_size`.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     rect()
///         .font_size(18.)
///         .child(ContextMenuViewer::new())
///         .child("Your app content here")
/// }
/// ```
#[derive(Default, Clone, PartialEq)]
pub struct ContextMenuViewer {
    key: DiffKey,
}

impl KeyExt for ContextMenuViewer {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl ContextMenuViewer {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ComponentOwned for ContextMenuViewer {
    fn render(self) -> impl IntoElement {
        let mut context = use_hook(|| {
            try_consume_root_context::<ContextMenu>().unwrap_or_else(|| {
                let state = ContextMenu {
                    location: State::create_in_scope(CursorPoint::default(), ScopeId::ROOT),
                    menu: State::create_in_scope(None, ScopeId::ROOT),
                    close_request: State::create_in_scope(
                        ContextMenuCloseRequest::None,
                        ScopeId::ROOT,
                    ),
                };
                provide_context_for_scope_id(state, ScopeId::ROOT);
                state
            })
        });

        rect()
            .on_global_pointer_move(move |e: Event<PointerEventData>| {
                context.location.set(e.global_location());
            })
            .maybe_child(context.menu.read().clone().map(|(location, menu)| {
                let location = location.to_f32();
                rect()
                    .layer(Layer::Overlay)
                    .position(Position::new_global().left(location.x).top(location.y))
                    .child(menu.on_close(move |_| match (context.close_request)() {
                        ContextMenuCloseRequest::None => {
                            context.close_request.set(ContextMenuCloseRequest::Pending);
                        }
                        ContextMenuCloseRequest::Pending => {
                            context.menu.set(None);
                            context.close_request.set(ContextMenuCloseRequest::None);
                        }
                    }))
            }))
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
