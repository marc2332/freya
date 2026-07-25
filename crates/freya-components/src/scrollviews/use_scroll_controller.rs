use freya_core::prelude::*;
use torin::prelude::Direction;

/// Where along an axis a scroll should land, the beginning or the end.
#[derive(Default, PartialEq, Eq)]
pub enum ScrollPosition {
    #[default]
    Start,
    End,
}

/// Initial configuration for a [`ScrollController`] created with [`use_scroll_controller`].
#[derive(Default)]
pub struct ScrollConfig {
    /// Where the vertical axis starts scrolled to when first laid out.
    pub default_vertical_position: ScrollPosition,
    /// Where the horizontal axis starts scrolled to when first laid out.
    pub default_horizontal_position: ScrollPosition,
}

/// A pending request to scroll an axis to a given [`ScrollPosition`], consumed on the next layout.
pub struct ScrollRequest {
    pub(crate) position: ScrollPosition,
    pub(crate) direction: Direction,
    pub(crate) init: bool,
}

impl ScrollRequest {
    /// Creates a request to scroll `direction` to `position`.
    pub fn new(position: ScrollPosition, direction: Direction) -> ScrollRequest {
        ScrollRequest {
            position,
            direction,
            init: false,
        }
    }
}

/// An absolute scroll movement along one axis, in pixels.
pub enum ScrollEvent {
    X(i32),
    Y(i32),
}

/// Handle to drive and read a scrollable area programmatically.
///
/// By default a scrollable owns its scroll position and only the user can move it, through the
/// wheel, the scrollbar, arrow keys or dragging. A [`ScrollController`] lets your own code read and
/// change that position instead. Create one with [`use_scroll_controller`] and hand it to a
/// scrollable through its `new_controlled` constructor.
///
/// Some cases where a controller is needed:
///
/// - Jumping to the top or bottom in response to an action, for example scrolling a chat to the
///   newest message after sending one.
/// - Keeping several scrollables in sync, like a diff view with two panes that move together.
/// - Reading the current scroll position to drive something else, such as a "scroll to top" button
///   that only appears once the user has scrolled down.
///
/// # Scrolling from code
///
/// [`scroll_to`](ScrollController::scroll_to) queues a jump to the start or end of an axis, applied
/// on the next layout. This is the common way to snap a list to its top or bottom.
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
///                 .children((0..100).map(|i| label().key(i).text(format!("Item {i}")).into())),
///         )
/// }
/// ```
///
/// For an exact pixel offset use [`scroll_to_y`](ScrollController::scroll_to_y) or
/// [`scroll_to_x`](ScrollController::scroll_to_x). The current position is available by converting
/// the controller into a `(i32, i32)` tuple of `(x, y)` pixels.
///
/// # Keeping scrollables in sync
///
/// Because a [`ScrollController`] is a cheap [`Copy`] handle, pass the same one to several
/// scrollables and they share a single scroll position: moving any of them moves the rest.
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
///                 .children((0..30).map(|i| label().key(i).text(format!("Left {i}")).into())),
///         )
///         .child(
///             ScrollView::new_controlled(scroll_controller)
///                 .width(Size::flex(1.))
///                 .children((0..30).map(|i| label().key(i).text(format!("Right {i}")).into())),
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
///         .children((0..100).map(|i| label().key(i).text(format!("Item {i}")).into()))
/// }
/// ```
#[derive(PartialEq, Clone, Copy)]
pub struct ScrollController {
    notifier: State<()>,
    requests: State<Vec<ScrollRequest>>,
    on_scroll: State<Callback<ScrollEvent, bool>>,
    get_scroll: State<Callback<(), (i32, i32)>>,
}

impl From<ScrollController> for (i32, i32) {
    /// Reads the current `(x, y)` scroll position in pixels.
    fn from(val: ScrollController) -> Self {
        val.get_scroll.read().call(())
    }
}

impl ScrollController {
    /// Creates a controller starting at scroll position `(x, y)` with a list of requests to apply.
    pub fn new(x: i32, y: i32, initial_requests: Vec<ScrollRequest>) -> Self {
        let mut scroll = State::create((x, y));
        Self {
            notifier: State::create(()),
            requests: State::create(initial_requests),
            on_scroll: State::create(Callback::new(move |ev| {
                let current = *scroll.read();
                match ev {
                    ScrollEvent::X(x) => {
                        scroll.write().0 = x;
                    }
                    ScrollEvent::Y(y) => {
                        scroll.write().1 = y;
                    }
                }
                current != *scroll.read()
            })),
            get_scroll: State::create(Callback::new(move |_| *scroll.read())),
        }
    }
    /// Builds a controller from externally owned state, letting the caller manage its storage.
    pub fn managed(
        notifier: State<()>,
        requests: State<Vec<ScrollRequest>>,
        on_scroll: State<Callback<ScrollEvent, bool>>,
        get_scroll: State<Callback<(), (i32, i32)>>,
    ) -> Self {
        Self {
            notifier,
            requests,
            on_scroll,
            get_scroll,
        }
    }

    /// Applies any pending requests against the given content size. Called by the scrollable on every layout.
    pub fn use_apply(&mut self, width: f32, height: f32) {
        let _ = self.notifier.read();
        for request in self.requests.write().drain(..) {
            match request {
                ScrollRequest {
                    position: ScrollPosition::Start,
                    direction: Direction::Vertical,
                    ..
                } => {
                    self.on_scroll.write().call(ScrollEvent::Y(0));
                }
                ScrollRequest {
                    position: ScrollPosition::Start,
                    direction: Direction::Horizontal,
                    ..
                } => {
                    self.on_scroll.write().call(ScrollEvent::X(0));
                }
                ScrollRequest {
                    position: ScrollPosition::End,
                    direction: Direction::Vertical,
                    init,
                    ..
                } => {
                    if init && height == 0. {
                        continue;
                    }
                    let (_x, y) = self.get_scroll.read().call(());
                    self.on_scroll
                        .write()
                        .call(ScrollEvent::Y(y - height as i32));
                }
                ScrollRequest {
                    position: ScrollPosition::End,
                    direction: Direction::Horizontal,
                    init,
                    ..
                } => {
                    if init && width == 0. {
                        continue;
                    }

                    let (x, _y) = self.get_scroll.read().call(());
                    self.on_scroll
                        .write()
                        .call(ScrollEvent::X(x - width as i32));
                }
            }
        }
    }

    /// Scrolls the horizontal axis to `to` pixels. Returns whether the position actually changed.
    pub fn scroll_to_x(&mut self, to: i32) -> bool {
        self.on_scroll.write().call(ScrollEvent::X(to))
    }

    /// Scrolls the vertical axis to `to` pixels. Returns whether the position actually changed.
    pub fn scroll_to_y(&mut self, to: i32) -> bool {
        self.on_scroll.write().call(ScrollEvent::Y(to))
    }

    /// Queues a scroll of `scroll_direction` to `scroll_position`, applied on the next layout.
    pub fn scroll_to(&mut self, scroll_position: ScrollPosition, scroll_direction: Direction) {
        self.requests
            .write()
            .push(ScrollRequest::new(scroll_position, scroll_direction));
        self.notifier.write();
    }
}

/// Creates a [`ScrollController`] tied to the component, configured by the returned [`ScrollConfig`].
pub fn use_scroll_controller(init: impl FnOnce() -> ScrollConfig) -> ScrollController {
    use_hook(|| {
        let config = init();

        ScrollController::new(
            0,
            0,
            vec![
                ScrollRequest {
                    position: config.default_vertical_position,
                    direction: Direction::Vertical,
                    init: true,
                },
                ScrollRequest {
                    position: config.default_horizontal_position,
                    direction: Direction::Horizontal,
                    init: true,
                },
            ],
        )
    })
}
