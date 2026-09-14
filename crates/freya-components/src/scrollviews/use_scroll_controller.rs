use freya_core::prelude::*;
use torin::{
    geometry::{
        Point2D,
        Size2D,
    },
    prelude::Direction,
};

use crate::scrollviews::shared::get_corrected_scroll_position;

/// Where along an axis a scroll should land, the beginning or the end.
#[derive(Default, PartialEq, Eq)]
pub enum ScrollPosition {
    #[default]
    Start,
    End,
}

impl ScrollPosition {
    /// Scroll offset in pixels of this position.
    fn offset(&self) -> i32 {
        match self {
            Self::Start => 0,
            Self::End => ScrollController::END,
        }
    }
}

/// Initial configuration for a [`ScrollController`] created with [`use_scroll_controller`].
#[derive(Default)]
pub struct ScrollConfig {
    /// Where the vertical axis starts scrolled to when first laid out.
    pub default_vertical_position: ScrollPosition,
    /// Where the horizontal axis starts scrolled to when first laid out.
    pub default_horizontal_position: ScrollPosition,
}

/// Handle to drive a scrollview programmatically.
///
/// By default a scrollview owns its scroll position and only the user can move it, through the
/// wheel, the scrollbar, arrow keys or dragging. A [`ScrollController`] lets your own code read and
/// change that position instead. Create one with [`use_scroll_controller`] and hand it to a
/// scrollview through its `new_controlled` constructor.
///
/// Some cases where a controller is needed:
///
/// - Jumping to the top or bottom in response to an action, for example scrolling a chat to the
///   newest message after sending one.
/// - Keeping several scrollviews in sync, like a diff view with two panes that move together.
/// - Reading the current scroll position to drive something else, such as a "scroll to top" button
///   that only appears once the user has scrolled down.
///
/// # Scrolling from code
///
/// [`scroll_to`](ScrollController::scroll_to) jumps to the start or end of an axis, with the end
/// resolved against the content on the next layout. This is the common way to snap a list to its
/// top or bottom.
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     let mut scroll_controller = use_scroll_controller(ScrollConfig::default);
///
///     rect()
///         .child(
///             Button::new()
///                 .on_press(move |_| {
///                     scroll_controller.scroll_to(ScrollPosition::End, Direction::Vertical);
///                 })
///                 .child("Scroll to bottom"),
///         )
///         .child(
///             ScrollView::new_controlled(scroll_controller)
///                 .children((0..100).map(|i| label().key(i).text(format!("Item {i}")))),
///         )
/// }
/// ```
///
/// For an exact pixel offset use [`scroll_to_y`](ScrollController::scroll_to_y) or
/// [`scroll_to_x`](ScrollController::scroll_to_x). The current position is available by converting
/// the controller into a `(i32, i32)` tuple of `(x, y)` pixels.
///
/// # Keeping scrollviews in sync
///
/// Because a [`ScrollController`] is a cheap [`Copy`] handle, you can pass the same one to several
/// scrollviews and they share a single scroll position, moving any of them moves the rest.
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     let scroll_controller = use_scroll_controller(ScrollConfig::default);
///
///     rect()
///         .horizontal()
///         .spacing(6.)
///         .child(
///             ScrollView::new_controlled(scroll_controller)
///                 .width(Size::flex(1.))
///                 .children((0..30).map(|i| label().key(i).text(format!("Left {i}")))),
///         )
///         .child(
///             ScrollView::new_controlled(scroll_controller)
///                 .width(Size::flex(1.))
///                 .children((0..30).map(|i| label().key(i).text(format!("Right {i}")))),
///         )
/// }
/// ```
///
/// # Starting position
///
/// The [`ScrollConfig`] passed to [`use_scroll_controller`] also decides where each axis starts.
/// Set [`default_vertical_position`](ScrollConfig::default_vertical_position) to
/// [`ScrollPosition::End`] to open a list already scrolled to the bottom.
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     let scroll_controller = use_scroll_controller(|| ScrollConfig {
///         default_vertical_position: ScrollPosition::End,
///         ..Default::default()
///     });
///
///     ScrollView::new_controlled(scroll_controller)
///         .children((0..100).map(|i| label().key(i).text(format!("Item {i}"))))
/// }
/// ```
#[derive(PartialEq, Clone, Copy)]
pub struct ScrollController {
    scroll: State<(i32, i32)>,
}

impl From<ScrollController> for (i32, i32) {
    /// Reads the current `(x, y)` scroll position in pixels.
    fn from(val: ScrollController) -> Self {
        *val.scroll.read()
    }
}

impl ScrollController {
    /// Offset of an axis scrolled to its end, resolved against the content size on the next layout.
    const END: i32 = i32::MIN;

    /// Creates a controller starting at the scroll position `(x, y)`.
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            scroll: State::create((x, y)),
        }
    }

    /// Turns an axis scrolled to its end into a pixel position, once the content size is known.
    pub(crate) fn apply_layout(&mut self, content_size: Size2D, viewport_size: Size2D) {
        let (x, y) = *self.scroll.peek();

        if x == Self::END {
            self.scroll_to_x(get_corrected_scroll_position(
                content_size.width,
                viewport_size.width,
                x as f32,
            ) as i32);
        }

        if y == Self::END {
            self.scroll_to_y(get_corrected_scroll_position(
                content_size.height,
                viewport_size.height,
                y as f32,
            ) as i32);
        }
    }

    pub(crate) fn position(self) -> Point2D {
        let (x, y): (i32, i32) = self.into();
        Point2D::new(x as f32, y as f32)
    }

    /// Scrolls the horizontal axis to `to` pixels. Returns whether the position actually changed.
    pub fn scroll_to_x(&mut self, to: i32) -> bool {
        let changed = self.scroll.peek().0 != to;
        if changed {
            self.scroll.write().0 = to;
        }
        changed
    }

    /// Scrolls the vertical axis to `to` pixels. Returns whether the position actually changed.
    pub fn scroll_to_y(&mut self, to: i32) -> bool {
        let changed = self.scroll.peek().1 != to;
        if changed {
            self.scroll.write().1 = to;
        }
        changed
    }

    /// Scrolls `scroll_direction` to `scroll_position`.
    pub fn scroll_to(&mut self, scroll_position: ScrollPosition, scroll_direction: Direction) {
        let to = scroll_position.offset();
        match scroll_direction {
            Direction::Vertical => self.scroll_to_y(to),
            Direction::Horizontal => self.scroll_to_x(to),
        };
    }
}

/// Creates a [`ScrollController`], configured by the returned [`ScrollConfig`].
pub fn use_scroll_controller(config: impl FnOnce() -> ScrollConfig) -> ScrollController {
    use_hook(|| {
        let config = config();

        ScrollController::new(
            config.default_horizontal_position.offset(),
            config.default_vertical_position.offset(),
        )
    })
}
