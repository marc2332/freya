use std::ops::Deref;

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
        let mut inner_area: State<Option<Area>> = use_state(|| None);
        let mut attached_area: State<Option<Area>> = use_state(|| None);

        let inner = inner_area.read();
        let attached = attached_area.read();

        let has_attachment = !self.children.is_empty();
        let is_measured = inner.deref().is_some() && attached.deref().is_some();

        let inner_width = inner.deref().map(|a| a.width()).unwrap_or_default();
        let inner_height = inner.deref().map(|a| a.height()).unwrap_or_default();
        let attached_width = attached.deref().map(|a| a.width()).unwrap_or_default();
        let attached_height = attached.deref().map(|a| a.height()).unwrap_or_default();

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
            .on_sized(move |e: Event<SizedEventData>| inner_area.set(Some(e.area)))
            .child(self.inner.clone())
            .maybe_child(has_attachment.then(|| {
                rect()
                    .on_sized(move |e: Event<SizedEventData>| {
                        if has_attachment {
                            attached_area.set(Some(e.area))
                        }
                    })
                    .position(position)
                    .layer(Layer::Overlay)
                    .opacity(if is_measured { 1. } else { 0. })
                    .children(self.children.clone())
            }))
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
