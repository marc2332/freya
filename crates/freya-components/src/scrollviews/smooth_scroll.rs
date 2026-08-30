use std::time::Instant;

use freya_core::prelude::*;
use torin::geometry::{
    Point2D,
    Size2D,
    Vector2D,
};

use crate::scrollviews::{
    ScrollController,
    shared::get_corrected_scroll_position,
};

/// Distance under which the animation is close enough to snap, in pixels.
const SETTLE_DISTANCE: f32 = 0.5;
/// Speed under which the animation is slow enough to stop, in pixels per second.
const SETTLE_SPEED: f32 = 5.0;

/// Seconds wheel and keyboard scrolls take to reach their destination.
const SMOOTHING_TIME: f32 = 0.1;
/// Seconds a fling takes to stop, which also scales how far it travels.
const FLING_TIME: f32 = 0.5;
/// Slowest drag release speed that still starts a fling, in pixels per second.
const FLING_MIN_SPEED: f32 = 50.0;

/// Follows the target held by a [`ScrollController`] with a critically damped `SmoothDamp` filter.
#[derive(Clone, Copy)]
pub struct SmoothScroll {
    scroll_controller: ScrollController,
    displayed: State<Point2D>,
    velocity: State<Vector2D>,
    task: State<Option<TaskHandle>>,
    smooth_time: State<f32>,
    drag_velocity: State<Vector2D>,
    last_drag_move: State<Instant>,
}

impl SmoothScroll {
    pub fn create(scroll_controller: ScrollController) -> Self {
        Self {
            scroll_controller,
            displayed: State::create(Point2D::zero()),
            velocity: State::create(Vector2D::zero()),
            task: State::create(None),
            smooth_time: State::create(SMOOTHING_TIME),
            drag_velocity: State::create(Vector2D::zero()),
            last_drag_move: State::create(Instant::now()),
        }
    }

    /// Position to render, the animated one while a scroll animation is running.
    pub fn position(&self, target: Point2D) -> Point2D {
        if self.task.read().is_some() {
            *self.displayed.read()
        } else {
            target
        }
    }

    /// Starts chasing the controller position from `current`, keeping the current velocity.
    pub fn animate_from(&mut self, current: Point2D) {
        self.start(current, None, SMOOTHING_TIME);
    }

    /// Like [`Self::animate_from`] but launched at `velocity` and decelerating slowly.
    fn fling_from(&mut self, current: Point2D, velocity: Vector2D) {
        self.start(current, Some(velocity), FLING_TIME);
    }

    fn start(&mut self, current: Point2D, velocity: Option<Vector2D>, smooth_time: f32) {
        self.smooth_time.set(smooth_time);
        if let Some(velocity) = velocity {
            self.velocity.set(velocity);
        }

        if self.task.read().is_some() {
            return;
        }
        self.displayed.set(current);

        let ticker = RenderingTicker::get();
        let platform = Platform::get();
        let animation_clock = AnimationClock::get();
        let scroll_controller = self.scroll_controller;
        let mut displayed = self.displayed;
        let mut velocity = self.velocity;
        let mut task = self.task;
        let smooth_time = self.smooth_time;

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
                let current = *displayed.peek();
                let current_velocity = *velocity.peek();

                // Pull strength, higher when the smooth time is shorter
                let omega = 2.0 / *smooth_time.peek();
                // Share of the distance that survives this frame
                let decay = (-omega * elapsed_seconds).exp();
                // Distance still to cover
                let change = current - target;
                // How much the current speed and distance push over this frame
                let linear_term = (current_velocity + change * omega) * elapsed_seconds;
                // Speed left after the pull has damped it
                let next_velocity = (current_velocity - linear_term * omega) * decay;
                // Position once the leftover distance has decayed
                let next = target + (change + linear_term) * decay;

                let remaining = target - next;
                if remaining.length() < SETTLE_DISTANCE && next_velocity.length() < SETTLE_SPEED {
                    displayed.set(target);
                    velocity.set(Vector2D::zero());
                    break;
                }

                displayed.set(next);
                velocity.set(next_velocity);
                platform.send(UserEvent::RequestRedraw);
            }

            task.write().take();
        });
        task.write().replace(animation_task);
    }

    /// Freezes any running animation and starts tracking a drag from the momentum it caught.
    pub fn begin_drag(&mut self) {
        let caught_velocity = self.stop();
        self.drag_velocity.set(caught_velocity);
        self.last_drag_move.set(Instant::now());
    }

    /// Feeds a drag movement, smoothing it into the velocity a later fling launches at.
    pub fn drag(&mut self, delta: Vector2D) {
        self.stop();

        let now = Instant::now();
        let elapsed_seconds = now
            .duration_since(*self.last_drag_move.peek())
            .as_secs_f32();
        if elapsed_seconds > 0.0 {
            let previous = *self.drag_velocity.peek();
            self.drag_velocity
                .set(previous.lerp(-delta / elapsed_seconds, 0.5));
        }
        self.last_drag_move.set(now);
    }

    /// Ends a drag, flinging from its velocity when it was fast enough to be a flick.
    pub fn release_drag(&mut self, from: Point2D, content: Size2D, viewport: Size2D) {
        let velocity = *self.drag_velocity.peek();
        if velocity.length() < FLING_MIN_SPEED {
            return;
        }

        let projected = from + velocity * FLING_TIME;
        let target_x = get_corrected_scroll_position(content.width, viewport.width, projected.x);
        let target_y = get_corrected_scroll_position(content.height, viewport.height, projected.y);

        self.fling_from(from, velocity);
        self.scroll_controller.scroll_to_x(target_x as i32);
        self.scroll_controller.scroll_to_y(target_y as i32);
    }

    /// Freezes the scroll at the displayed position, returning the velocity it was moving at.
    pub fn stop(&mut self) -> Vector2D {
        if let Some(task) = self.task.write().take() {
            task.cancel();

            let displayed = self.displayed.peek().to_i32();
            self.scroll_controller.scroll_to_x(displayed.x);
            self.scroll_controller.scroll_to_y(displayed.y);
        }

        let velocity = *self.velocity.peek();
        self.velocity.set(Vector2D::zero());
        velocity
    }
}

/// Creates a [`SmoothScroll`] tied to the component.
pub fn use_smooth_scroll(scroll_controller: impl FnOnce() -> ScrollController) -> SmoothScroll {
    use_hook(|| SmoothScroll::create(scroll_controller()))
}
