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
#[derive(Clone, Copy)]
struct SmoothDamp {
    position: Point2D,
    velocity: Vector2D,
    smooth_time: f32,
}

impl SmoothDamp {
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
struct Drag {
    velocity: Vector2D,
    last_move: Instant,
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
#[derive(Clone, Copy)]
pub struct SmoothScroll {
    scroll_controller: ScrollController,
    damp: State<SmoothDamp>,
    drag: State<Drag>,
    task: State<Option<TaskHandle>>,
}

impl SmoothScroll {
    pub fn create(scroll_controller: ScrollController) -> Self {
        Self {
            scroll_controller,
            damp: State::create(SmoothDamp {
                position: Point2D::zero(),
                velocity: Vector2D::zero(),
                smooth_time: TargetPlatform::Unknown.scroll_smoothing_time(),
            }),
            drag: State::create(Drag {
                velocity: Vector2D::zero(),
                last_move: Instant::now(),
            }),
            task: State::create(None),
        }
    }

    /// Position to render, the animated one while a scroll animation is running.
    pub fn position(&self, target: Point2D) -> Point2D {
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
        let scroll_controller = self.scroll_controller;
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
        self.scroll_controller.scroll_to_x(target_x as i32);
        self.scroll_controller.scroll_to_y(target_y as i32);
    }

    /// Freezes the scroll where it is, returning the velocity it was moving at.
    pub fn stop(&mut self) -> Vector2D {
        if let Some(task) = self.task.write().take() {
            task.cancel();

            let position = self.damp.peek().position.to_i32();
            self.scroll_controller.scroll_to_x(position.x);
            self.scroll_controller.scroll_to_y(position.y);
        }

        let velocity = self.damp.peek().velocity;
        self.damp.write().velocity = Vector2D::zero();
        velocity
    }
}

pub fn use_smooth_scroll(scroll_controller: impl FnOnce() -> ScrollController) -> SmoothScroll {
    use_hook(|| SmoothScroll::create(scroll_controller()))
}
