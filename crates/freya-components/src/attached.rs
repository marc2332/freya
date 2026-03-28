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
        let mut container_size = use_state(Area::default);
        let mut attached_size = use_state(Area::default);

        let on_container_sized = move |e: Event<SizedEventData>| {
            container_size.set(e.area);
        };

        let on_attached_sized = move |e: Event<SizedEventData>| {
            attached_size.set(e.area);
        };

        let container_w = container_size.read().width();
        let container_h = container_size.read().height();
        let attached_w = attached_size.read().width();
        let attached_h = attached_size.read().height();

        let attached_position = match self.position {
            AttachedPosition::Top => Position::new_absolute()
                .top(-attached_h)
                .left((container_w - attached_w) / 2.),
            AttachedPosition::Bottom => Position::new_absolute()
                .top(container_h)
                .left((container_w - attached_w) / 2.),
            AttachedPosition::Left => Position::new_absolute()
                .top((container_h - attached_h) / 2.)
                .left(-attached_w),
            AttachedPosition::Right => Position::new_absolute()
                .top((container_h - attached_h) / 2.)
                .left(container_w),
        };

        rect()
            .on_sized(on_container_sized)
            .child(self.inner.clone())
            .child(
                rect()
                    .on_sized(on_attached_sized)
                    .position(attached_position)
                    .layer(Layer::Overlay)
                    .children(self.children.clone()),
            )
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
