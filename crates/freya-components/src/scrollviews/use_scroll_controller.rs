use std::time::Instant;

use freya_core::prelude::*;
use torin::{
    geometry::{
        Point2D,
        Size2D,
        Vector2D,
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
    pub(crate) scroll: State<(i32, i32)>,
    pub(crate) damp: State<SmoothDamp>,
    pub(crate) drag: State<Drag>,
    pub(crate) task: State<Option<TaskHandle>>,
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
            damp: State::create(SmoothDamp::new()),
            drag: State::create(Drag::default()),
            task: State::create(None),
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

/// Distance under which the animation is close enough to snap, in pixels.
const SETTLE_DISTANCE: f32 = 0.5;
/// Speed under which the animation is slow enough to stop, in pixels per second.
const SETTLE_SPEED: f32 = 20.0;

/// Slowest drag release speed that still starts a fling, in pixels per second.
const FLING_MIN_SPEED: f32 = 50.0;

/// Scrolling feel of a [`TargetPlatform`].
pub(crate) trait ScrollFeel {
    /// Seconds wheel and keyboard scrolls take to reach their destination.
    fn scroll_smoothing_time(&self) -> f32;
    /// Seconds a fling takes to stop, which also scales how far it travels.
    fn scroll_fling_time(&self) -> f32;
}

impl ScrollFeel for TargetPlatform {
    fn scroll_smoothing_time(&self) -> f32 {
        if self.is_mobile() { 0.1 } else { 0.06 }
    }

    fn scroll_fling_time(&self) -> f32 {
        if self.is_mobile() { 0.35 } else { 0.5 }
    }
}

/// Moves a value towards a target with a smooth and continuous animation.
#[derive(Clone, Copy, Default)]
pub(crate) struct SmoothDamp {
    position: Point2D,
    velocity: Vector2D,
    smooth_time: f32,
}

impl SmoothDamp {
    pub(crate) fn new() -> Self {
        Self {
            position: Point2D::zero(),
            velocity: Vector2D::zero(),
            smooth_time: TargetPlatform::Unknown.scroll_smoothing_time(),
        }
    }

    /// Returns whether it has settled onto `target`.
    fn advance(&mut self, target: Point2D, elapsed_seconds: f32) -> bool {
        let omega = 2.0 / self.smooth_time;
        let decay = (-omega * elapsed_seconds).exp();
        let change = self.position - target;
        let linear_term = (self.velocity + change * omega) * elapsed_seconds;

        let velocity = (self.velocity - linear_term * omega) * decay;
        let position = target + (change + linear_term) * decay;

        if (target - position).length() < SETTLE_DISTANCE && velocity.length() < SETTLE_SPEED {
            self.position = target;
            self.velocity = Vector2D::zero();
            return true;
        }

        self.position = position;
        self.velocity = velocity;
        false
    }
}

/// Velocity tracked while the content is dragged, to fling with on release.
#[derive(Clone, Copy)]
pub(crate) struct Drag {
    velocity: Vector2D,
    last_move: Instant,
}

impl Default for Drag {
    fn default() -> Self {
        Self {
            velocity: Vector2D::zero(),
            last_move: Instant::now(),
        }
    }
}

impl Drag {
    fn track(&mut self, delta: Vector2D) {
        let now = Instant::now();
        let elapsed_seconds = now.duration_since(self.last_move).as_secs_f32();
        if elapsed_seconds > 0.0 {
            self.velocity = self.velocity.lerp(-delta / elapsed_seconds, 0.5);
        }
        self.last_move = now;
    }
}

/// Follows the target held by a [`ScrollController`].
impl ScrollController {
    /// Position to render, the animated one while a scroll animation is running.
    pub fn animated_position(&self, target: Point2D) -> Point2D {
        if self.task.read().is_some() {
            self.damp.read().position
        } else {
            target
        }
    }

    /// Chases the controller position from `current`, keeping the current velocity.
    pub fn animate_from(&mut self, current: Point2D) {
        self.start(current, None, TargetPlatform::get().scroll_smoothing_time());
    }

    /// Like [`Self::animate_from`] but launched at `velocity` and slower to stop.
    fn fling_from(&mut self, current: Point2D, velocity: Vector2D) {
        self.start(
            current,
            Some(velocity),
            TargetPlatform::get().scroll_fling_time(),
        );
    }

    fn start(&mut self, current: Point2D, velocity: Option<Vector2D>, smooth_time: f32) {
        let is_animating = self.task.read().is_some();
        {
            let mut damp = self.damp.write();
            damp.smooth_time = smooth_time;
            if let Some(velocity) = velocity {
                damp.velocity = velocity;
            }
            if !is_animating {
                damp.position = current;
            }
        }
        if is_animating {
            return;
        }

        let ticker = RenderingTicker::get();
        let platform = Platform::get();
        let animation_clock = AnimationClock::get();
        let scroll_controller = *self;
        let mut damp = self.damp;
        let mut task = self.task;

        let animation_task = spawn(async move {
            platform.send(UserEvent::RequestRedraw);
            let mut previous_frame = Instant::now();

            loop {
                ticker.tick().await;

                let elapsed_seconds = animation_clock
                    .correct_elapsed_duration(previous_frame.elapsed())
                    .as_secs_f32();
                previous_frame = Instant::now();

                let target = scroll_controller.position();
                if damp.write().advance(target, elapsed_seconds) {
                    break;
                }

                platform.send(UserEvent::RequestRedraw);
            }

            task.write().take();
        });
        task.write().replace(animation_task);
    }

    /// Freezes the animation and starts a drag from the momentum it caught.
    pub fn begin_drag(&mut self) {
        let caught_velocity = self.stop();
        self.drag.set(Drag {
            velocity: caught_velocity,
            last_move: Instant::now(),
        });
    }

    /// Feeds a drag movement into the tracked velocity.
    pub fn drag(&mut self, delta: Vector2D) {
        self.stop();
        self.drag.write().track(delta);
    }

    /// Ends a drag, flinging when it was fast enough to be a flick.
    pub fn release_drag(&mut self, from: Point2D, content: Size2D, viewport: Size2D) {
        let velocity = self.drag.peek().velocity;
        if velocity.length() < FLING_MIN_SPEED {
            return;
        }

        let projected = from + velocity * TargetPlatform::get().scroll_fling_time();
        let target_x = get_corrected_scroll_position(content.width, viewport.width, projected.x);
        let target_y = get_corrected_scroll_position(content.height, viewport.height, projected.y);

        self.fling_from(from, velocity);
        self.scroll_to_x(target_x as i32);
        self.scroll_to_y(target_y as i32);
    }

    /// Freezes the scroll where it is, returning the velocity it was moving at.
    pub fn stop(&mut self) -> Vector2D {
        let task = self.task.write().take();
        if let Some(task) = task {
            task.cancel();

            let position = self.damp.peek().position.to_i32();
            self.scroll_to_x(position.x);
            self.scroll_to_y(position.y);
        }

        let velocity = self.damp.peek().velocity;
        self.damp.write().velocity = Vector2D::zero();
        velocity
    }
}
