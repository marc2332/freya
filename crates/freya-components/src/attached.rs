use freya_core::prelude::*;
use torin::prelude::{
    Area,
    Position,
};

use crate::menu::overflow_offset;

/// Marker context provided by [`Attached`] to its subtree: the overlay's position is
/// already window-clamped, so hosted content (e.g. `MenuContainer`) must not apply its
/// own post-hoc overflow correction: a second, self-measured correction lags a frame
/// behind and paints a visible jump while `Attached` settles.
#[derive(Clone)]
pub(crate) struct AttachedHosted;

/// Position where the attached element will be placed relative to the inner element.
#[derive(PartialEq, Clone, Copy, Debug, Default)]
pub enum AttachedPosition {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
}

/// Cross-axis alignment of the attached element against the inner one. For `Top`/`Bottom` this is
/// horizontal (the attached element's left / centre / right edge lines up with the inner element's);
/// for `Left`/`Right` it's vertical. `Center` is the historical default; `End` gives corner anchoring
/// (e.g. `Bottom` + `End` = the two right edges align, so the panel opens down-and-left — useful for a
/// trigger near the right screen edge, so the panel doesn't overflow off-screen).
#[derive(PartialEq, Clone, Copy, Debug, Default)]
pub enum AttachedAlign {
    Start,
    #[default]
    Center,
    End,
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
    align: AttachedAlign,
    offset: f32,
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
            align: AttachedAlign::Center,
            offset: 0.,
            key: DiffKey::None,
        }
    }

    pub fn position(mut self, position: AttachedPosition) -> Self {
        self.position = position;
        self
    }

    /// Gap between the inner element and the attached one, along the attachment axis (default
    /// `0.`, i.e. flush). A dropdown or popover usually wants a few pixels so its card reads as
    /// a separate surface instead of growing out of its trigger. The window clamp still applies
    /// afterwards, so an offset can't push the overlay off-screen.
    pub fn offset(mut self, offset: f32) -> Self {
        self.offset = offset;
        self
    }

    /// Cross-axis alignment against the inner element (default [`AttachedAlign::Center`]).
    pub fn align(mut self, align: AttachedAlign) -> Self {
        self.align = align;
        self
    }

    /// Align the attached element to the inner element's start edge (left for `Top`/`Bottom`).
    pub fn align_start(self) -> Self {
        self.align(AttachedAlign::Start)
    }

    /// Align the attached element to the inner element's end edge (right for `Top`/`Bottom`) — corner
    /// anchoring, so a panel opens inward from a trigger near the screen edge.
    pub fn align_end(self) -> Self {
        self.align(AttachedAlign::End)
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

        use_provide_context(|| AttachedHosted);

        let inner = *inner_area.read();
        let attached = *attached_area.read();

        let is_measured = inner.is_some() && attached.is_some();

        let inner_width = inner.map(|a| a.width()).unwrap_or_default();
        let inner_height = inner.map(|a| a.height()).unwrap_or_default();
        let attached_width = attached.map(|a| a.width()).unwrap_or_default();
        let attached_height = attached.map(|a| a.height()).unwrap_or_default();

        // Cross-axis offset (horizontal for Top/Bottom, vertical for Left/Right): where the attached
        // element's start / centre / end edge lands against the inner element's span.
        let align_offset = |inner_span: f32, attached_span: f32| match self.align {
            AttachedAlign::Start => 0.,
            AttachedAlign::Center => (inner_span - attached_span) / 2.,
            AttachedAlign::End => inner_span - attached_span,
        };
        let cross_h = align_offset(inner_width, attached_width);
        let cross_v = align_offset(inner_height, attached_height);

        let (left, top) = match self.position {
            AttachedPosition::Top => (cross_h, -attached_height - self.offset),
            AttachedPosition::Bottom => (cross_h, inner_height + self.offset),
            AttachedPosition::Left => (-attached_width - self.offset, cross_v),
            AttachedPosition::Right => (inner_width + self.offset, cross_v),
        };

        // Window clamp, computed *with* the position in the same frame (the inner area's
        // origin is global, so the overlay's would-be global origin is known here): the
        // overlay slides back inside the window instead of hanging off an edge. Doing it
        // here, rather than the overlay content self-measuring and offsetting after the
        // fact, means there is never a frame positioned without its correction.
        let (left, top) = match inner {
            Some(inner_area) if is_measured => {
                let root_size = *Platform::get().root_size.peek();
                (
                    left + overflow_offset(
                        inner_area.origin.x + left,
                        attached_width,
                        root_size.width,
                    ),
                    top + overflow_offset(
                        inner_area.origin.y + top,
                        attached_height,
                        root_size.height,
                    ),
                )
            }
            _ => (left, top),
        };

        let position = Position::new_absolute().top(top).left(left);

        rect()
            .on_sized(move |e: Event<SizedEventData>| inner_area.set(Some(e.area)))
            .child(self.inner.clone())
            .maybe_child((!self.children.is_empty()).then(|| {
                rect()
                    .on_sized(move |e: Event<SizedEventData>| attached_area.set(Some(e.area)))
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
