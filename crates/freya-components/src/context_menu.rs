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
/// A default viewer is provided automatically.
/// Mount a [`ContextMenuViewer`] locally to inherit custom text styles and theme changes.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     rect().child(
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
    viewers: State<Vec<ScopeId>>,
}

impl ContextMenu {
    /// Returns whether the context menu is open, or `false` if no viewer is mounted.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use freya::prelude::*;
    /// fn app() -> impl IntoElement {
    ///     rect().child(if ContextMenu::is_open() {
    ///         "Menu open"
    ///     } else {
    ///         "Menu closed"
    ///     })
    /// }
    /// ```
    pub fn is_open() -> bool {
        try_consume_root_context::<Self>().is_some_and(|c| c.menu.read().is_some())
    }

    /// Open the context menu, from a press event or programmatically.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use freya::prelude::*;
    /// fn app() -> impl IntoElement {
    ///     rect().child(Button::new().child("Open menu").on_press(|_| {
    ///         ContextMenu::open(Menu::new().child(MenuButton::new().child("Copy")));
    ///     }))
    /// }
    /// ```
    pub fn open(menu: Menu) {
        Self::open_with_phase(menu, ClosePhase::Idle);
    }

    /// Open the context menu from a pointer down event, like `on_secondary_down`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use freya::prelude::*;
    /// fn app() -> impl IntoElement {
    ///     rect().child(rect().child("Right click here").on_secondary_down(|_| {
    ///         ContextMenu::open_from_down(Menu::new().child(MenuButton::new().child("Copy")));
    ///     }))
    /// }
    /// ```
    pub fn open_from_down(menu: Menu) {
        Self::open_with_phase(menu, ClosePhase::PendingPress);
    }

    fn open_with_phase(menu: Menu, phase: ClosePhase) {
        let mut this = consume_root_context::<Self>();
        this.menu.set(Some(((this.location)(), menu)));
        this.close_phase.set(phase);
    }

    /// Closes the context menu, doing nothing if no viewer is mounted.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use freya::prelude::*;
    /// fn app() -> impl IntoElement {
    ///     rect().child(
    ///         Button::new()
    ///             .child("Close menu")
    ///             .on_press(|_| ContextMenu::close()),
    ///     )
    /// }
    /// ```
    pub fn close() {
        if let Some(mut this) = try_consume_root_context::<Self>() {
            this.menu.set_if_modified(None);
        }
    }
}

/// Overrides the automatic context menu viewer to inherit local text styles and theme changes.
///
/// Place it under styled elements or within a component providing a custom theme.
/// This changes menu styling for the whole window, not just the surrounding content.
///
/// The most recently mounted viewer renders the menu. Removing it restores the previous viewer.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     use_init_theme(dark_theme);
///
///     rect()
///         .theme_background()
///         .font_size(18.)
///         .child(ContextMenuViewer::new())
///         .child(Button::new().child("Open menu").on_press(|_| {
///             ContextMenu::open(
///                 Menu::new().child(
///                     MenuButton::new()
///                         .child("Close")
///                         .on_press(|_| ContextMenu::close()),
///                 ),
///             );
///         }))
/// }
/// ```
#[derive(Default, Clone, PartialEq)]
pub struct ContextMenuViewer {
    key: DiffKey,
}

impl KeyExt for ContextMenuViewer {
    fn key_mut(&mut self) -> &mut DiffKey {
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
        let scope_id = current_scope_id();
        let mut context = use_hook(|| {
            let mut context = try_consume_root_context::<ContextMenu>().unwrap_or_else(|| {
                let state = ContextMenu {
                    location: State::create_in_scope(CursorPoint::default(), ScopeId::ROOT),
                    menu: State::create_in_scope(None, ScopeId::ROOT),
                    close_phase: State::create_in_scope(ClosePhase::Idle, ScopeId::ROOT),
                    viewers: State::create_in_scope(Vec::new(), ScopeId::ROOT),
                };
                provide_context_for_scope_id(state, ScopeId::ROOT);
                state
            });
            context.viewers.write().push(scope_id);
            context
        });
        use_drop(move || {
            context.viewers.write().retain(|viewer| *viewer != scope_id);
        });

        use_side_effect(move || {
            if !*Platform::get().current_window().is_app_focused.read() {
                context.menu.set_if_modified(None);
            }
        });

        let is_active = context.viewers.read().last() == Some(&scope_id);

        rect().maybe(is_active, |el| {
            el.on_global_pointer_move(move |e: Event<PointerEventData>| {
                if context.viewers.peek().last() == Some(&scope_id) {
                    context.location.set(e.global_location());
                }
            })
            .on_global_pointer_down(move |_: Event<PointerEventData>| {
                if context.viewers.peek().last() == Some(&scope_id) && context.menu.read().is_some()
                {
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
                    .position(
                        Position::new_global()
                            .left(location.x.round())
                            .top(location.y.round()),
                    )
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
        })
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
