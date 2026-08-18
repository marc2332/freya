use freya_core::prelude::*;

/// Renders its children only once its wrapper element has become visible.
///
/// By default the children stay rendered afterwards, use [`Lazy::keep_rendered`] to
/// unrender them whenever the wrapper goes out of view again.
/// Give the wrapper a size so that it can be scrolled into view while still empty.
///
/// # Example
///
/// ```rust,no_run
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     ScrollView::new()
///         .child(rect().height(Size::px(1000.)))
///         .child(
///             Lazy::new()
///                 .height(Size::px(200.))
///                 .child("Rendered once scrolled into view"),
///         )
/// }
/// ```
#[derive(PartialEq)]
pub struct Lazy {
    keep_rendered: bool,
    elements: Vec<Element>,
    layout: LayoutData,
    key: DiffKey,
}

impl KeyExt for Lazy {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl ChildrenExt for Lazy {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.elements
    }
}

impl LayoutExt for Lazy {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.layout
    }
}

impl ContainerExt for Lazy {}

impl ContainerWithContentExt for Lazy {}

impl Default for Lazy {
    fn default() -> Self {
        Self::new()
    }
}

impl Lazy {
    pub fn new() -> Self {
        Self {
            keep_rendered: true,
            elements: Vec::new(),
            layout: LayoutData::default(),
            key: DiffKey::None,
        }
    }

    /// Keep the children rendered once the wrapper goes out of view again. Enabled by default.
    pub fn keep_rendered(mut self, keep_rendered: bool) -> Self {
        self.keep_rendered = keep_rendered;
        self
    }
}

impl Component for Lazy {
    fn render(&self) -> impl IntoElement {
        let mut visible = use_state(|| false);
        let elements = self.elements.clone();
        let keep_rendered = self.keep_rendered;
        let is_visible = *visible.read();

        rect()
            .layout(self.layout.clone())
            .maybe(!is_visible, |el| el.on_visible(move |_| visible.set(true)))
            .maybe(is_visible && !keep_rendered, |el| {
                el.on_hidden(move |_| visible.set(false))
            })
            .maybe(is_visible, |el| el.children(elements))
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
