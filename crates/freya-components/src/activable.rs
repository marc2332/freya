use freya_core::prelude::*;

use crate::activable_context::ActivableContext;

/// User-controlled provider of [`ActivableContext`].
///
/// Wraps a subtree and exposes whether it is active to descendants via
/// [`use_is_active`](crate::activable_context::use_is_active). Pair with components like
/// [`SideBarItem`](crate::sidebar::SideBarItem) or [`FloatingTab`](crate::floating_tab::FloatingTab)
/// to drive their active styling outside of a router context.
///
/// ```rust, ignore
/// let active = use_state(|| false);
///
/// Activable::new(SideBarItem::new().child("Item"))
///     .active(active)
/// ```
#[derive(PartialEq, Clone)]
pub struct Activable {
    child: Element,
    active: Readable<bool>,
    key: DiffKey,
}

impl KeyExt for Activable {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Activable {
    pub fn new(child: impl Into<Element>) -> Self {
        Self {
            child: child.into(),
            active: false.into(),
            key: DiffKey::None,
        }
    }

    /// Set the active state. Accepts any reactive source convertible to [`Readable<bool>`],
    /// such as [`State<bool>`], [`Memo<bool>`], a `RadioSlice`, or a plain `bool`.
    pub fn active(mut self, active: impl Into<Readable<bool>>) -> Self {
        self.active = active.into();
        self
    }
}

impl Component for Activable {
    fn render(&self) -> impl IntoElement {
        let is_active = *self.active.read();

        let mut state = use_state(|| is_active);

        if *state.peek() != is_active {
            *state.write() = is_active;
        }

        use_provide_context::<ActivableContext>(|| ActivableContext(state.into_readable()));

        self.child.clone()
    }
}
