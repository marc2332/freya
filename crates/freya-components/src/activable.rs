use freya_core::prelude::*;

use crate::activable_context::ActivableContext;

/// User-controlled provider of [`ActivableContext`].
///
/// Exposes whether the descendants are considered active, useful to drive
/// active styling of components like [`SideBarItem`](crate::sidebar::SideBarItem)
/// or [`FloatingTab`](crate::floating_tab::FloatingTab) outside of a router context.
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
