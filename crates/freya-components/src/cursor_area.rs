use freya_core::prelude::*;

/// A container that changes the cursor icon when hovered.
///
/// When the component is dropped while still being hovered, the cursor
/// is automatically reset to the default.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     CursorArea::new().child("Hover me!")
/// }
/// ```
#[derive(Clone, PartialEq)]
pub struct CursorArea {
    children: Vec<Element>,
    layout: LayoutData,
    key: DiffKey,
    cursor_icon: CursorIcon,
}

impl Default for CursorArea {
    fn default() -> Self {
        Self::new()
    }
}

impl ChildrenExt for CursorArea {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl KeyExt for CursorArea {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl LayoutExt for CursorArea {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.layout
    }
}

impl ContainerExt for CursorArea {}

impl CursorArea {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            layout: LayoutData::default(),
            key: DiffKey::None,
            cursor_icon: CursorIcon::Pointer,
        }
    }

    /// Set the cursor icon to show when hovered.
    /// Default is [CursorIcon::Pointer].
    pub fn icon(mut self, cursor_icon: CursorIcon) -> Self {
        self.cursor_icon = cursor_icon;
        self
    }
}

impl Component for CursorArea {
    fn render(&self) -> impl IntoElement {
        let mut hovering = use_state(|| false);
        let cursor_icon = self.cursor_icon;

        use_drop(move || {
            if hovering() {
                Cursor::set(CursorIcon::default());
            }
        });

        let on_pointer_enter = move |_: Event<PointerEventData>| {
            hovering.set(true);
            Cursor::set(cursor_icon);
        };

        let on_pointer_leave = move |_: Event<PointerEventData>| {
            hovering.set(false);
            Cursor::set(CursorIcon::default());
        };

        rect()
            .layout(self.layout.clone())
            .on_pointer_enter(on_pointer_enter)
            .on_pointer_leave(on_pointer_leave)
            .children(self.children.clone())
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
