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

/// Closing is only allowed once a new click starts after the menu opened.
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum ClosePhase {
    /// A down event just happened, now waiting for its press.
    PendingPress,
    /// Waiting for a new click.
    Idle,
    /// A new click started, it may close the menu.
    CloseAllowed,
}

/// Global context menu state.
///
/// Requires a [`ContextMenuViewer`] in an ancestor scope.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     rect().child(ContextMenuViewer::new()).child(
///         rect()
///             .on_secondary_down(move |_| {
///                 ContextMenu::open_from_down(
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
    pub(crate) close_phase: State<ClosePhase>,
}

impl ContextMenu {
    /// # Panics
    ///
    /// Panics if no [`ContextMenuViewer`] is mounted in an ancestor scope.
    pub fn get() -> Self {
        try_consume_root_context()
            .expect("ContextMenu requires a `ContextMenuViewer` in an ancestor scope")
    }

    pub fn is_open() -> bool {
        try_consume_root_context::<Self>().is_some_and(|c| c.menu.read().is_some())
    }

    /// Open the context menu, from a press event or programmatically.
    pub fn open(menu: Menu) {
        Self::open_with_phase(menu, ClosePhase::Idle);
    }

    /// Open the context menu from a pointer down event, like `on_secondary_down`.
    pub fn open_from_down(menu: Menu) {
        Self::open_with_phase(menu, ClosePhase::PendingPress);
    }

    fn open_with_phase(menu: Menu, phase: ClosePhase) {
        let mut this = Self::get();
        this.menu.set(Some(((this.location)(), menu)));
        this.close_phase.set(phase);
    }

    pub fn close() {
        if let Some(mut this) = try_consume_root_context::<Self>() {
            this.menu.set(None);
        }
    }
}

/// Provides the [`ContextMenu`] state and renders the floating menu overlay.
///
/// Mount this as high up in your tree as possible (typically in your `app`
/// component) so the rendered menu inherits styling like `font_size` from
/// the app's root element.
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
                    close_phase: State::create_in_scope(ClosePhase::Idle, ScopeId::ROOT),
                };
                provide_context_for_scope_id(state, ScopeId::ROOT);
                state
            })
        });

        use_side_effect(move || {
            if !*Platform::get().is_app_focused.read() {
                context.menu.set(None);
            }
        });

        rect()
            .on_global_pointer_move(move |e: Event<PointerEventData>| {
                context.location.set(e.global_location());
            })
            .on_global_pointer_down(move |_: Event<PointerEventData>| {
                if context.menu.read().is_some() {
                    let phase = match (context.close_phase)() {
                        ClosePhase::PendingPress => ClosePhase::Idle,
                        _ => ClosePhase::CloseAllowed,
                    };
                    context.close_phase.set(phase);
                }
            })
            .maybe_child(context.menu.read().clone().map(|(location, menu)| {
                let location = location.to_f32();
                rect()
                    .layer(Layer::Overlay)
                    .position(Position::new_global().left(location.x).top(location.y))
                    .child(
                        menu.on_close(move |_| {
                            if (context.close_phase)() == ClosePhase::CloseAllowed {
                                context.menu.set(None);
                            }
                        })
                        .on_escape(move |_| {
                            context.menu.set(None);
                        }),
                    )
            }))
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
