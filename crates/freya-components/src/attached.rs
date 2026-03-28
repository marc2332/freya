use freya_core::prelude::*;
use torin::prelude::{
    Area,
    Position,
};

/// Position where the attached element will be placed relative to the inner element.
#[derive(PartialEq, Clone, Copy, Debug, Default)]
pub enum AttachedPosition {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
}

/// A container that attaches elements to the top, bottom, left, or right of an inner element.
///
/// Uses absolute positioning and measures the attached element's size
/// to offset it correctly relative to the inner content.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     let mut open = use_state(|| false);
///
///     Attached::new(
///         Button::new()
///             .on_press(move |_| open.toggle())
///             .child("Toggle"),
///     )
///     .bottom()
///     .maybe_child(open().then(|| label().text("Attached below!")))
/// }
/// ```
#[derive(PartialEq)]
pub struct Attached {
    inner: Element,
    children: Vec<Element>,
    position: AttachedPosition,
    key: DiffKey,
}

impl KeyExt for Attached {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl ChildrenExt for Attached {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl Attached {
    pub fn new(inner: impl IntoElement) -> Self {
        Self {
            inner: inner.into_element(),
            children: vec![],
            position: AttachedPosition::Bottom,
            key: DiffKey::None,
        }
    }

    pub fn position(mut self, position: AttachedPosition) -> Self {
        self.position = position;
        self
    }

    pub fn top(self) -> Self {
        self.position(AttachedPosition::Top)
    }

    pub fn bottom(self) -> Self {
        self.position(AttachedPosition::Bottom)
    }

    pub fn left(self) -> Self {
        self.position(AttachedPosition::Left)
    }

    pub fn right(self) -> Self {
        self.position(AttachedPosition::Right)
    }
}

impl Component for Attached {
    fn render(&self) -> impl IntoElement {
        let mut inner_area = use_state(Area::default);
        let mut attached_area = use_state(Area::default);

        let inner_width = inner_area.read().width();
        let inner_height = inner_area.read().height();
        let attached_width = attached_area.read().width();
        let attached_height = attached_area.read().height();

        let position = match self.position {
            AttachedPosition::Top => Position::new_absolute()
                .top(-attached_height)
                .left((inner_width - attached_width) / 2.),
            AttachedPosition::Bottom => Position::new_absolute()
                .top(inner_height)
                .left((inner_width - attached_width) / 2.),
            AttachedPosition::Left => Position::new_absolute()
                .top((inner_height - attached_height) / 2.)
                .left(-attached_width),
            AttachedPosition::Right => Position::new_absolute()
                .top((inner_height - attached_height) / 2.)
                .left(inner_width),
        };

        rect()
            .on_sized(move |e: Event<SizedEventData>| inner_area.set(e.area))
            .child(self.inner.clone())
            .child(
                rect()
                    .on_sized(move |e: Event<SizedEventData>| attached_area.set(e.area))
                    .position(position)
                    .layer(Layer::Overlay)
                    .children(self.children.clone()),
            )
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
