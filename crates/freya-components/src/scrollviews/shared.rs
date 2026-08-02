use std::{
    cell::Cell,
    rc::Rc,
    time::{
        Duration,
        Instant,
    },
};

use freya_core::prelude::*;
use torin::{
    prelude::{
        Area,
        Direction,
    },
    size::Size,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Axis {
    X,
    Y,
}

/// One wheel gesture is a run of wheel events with no gap larger than this. Wheel events carry
/// no gesture phases, so this window is how gestures are bounded; it also keeps a trackpad's
/// momentum tail inside the gesture that spawned it.
const WHEEL_GESTURE_WINDOW: Duration = Duration::from_millis(200);

/// A wheel gesture arriving at or slower than this rate is not accelerated at all: a reader
/// moving deliberately through a list is looking for something and wants the plain rate.
const WHEEL_ACCELERATION_SLOW: Duration = Duration::from_millis(90);

/// A wheel gesture arriving at or faster than this rate is accelerated to the ceiling. About as
/// fast as a wheel can physically be spun, so the ramp spends most of its range below it.
const WHEEL_ACCELERATION_FAST: Duration = Duration::from_millis(15);

/// The most a wheel delta is multiplied by. What a ceiling buys is the reader's place in the
/// list, so there is a second, tighter bound in [`accelerate_wheel_delta`]: a viewport per event.
const WHEEL_ACCELERATION_MAX: f32 = 10.0;

/// How much to scale a wheel delta given the gap since the previous event of the same gesture.
/// Squared so the ramp starts gently, keeping an ordinary browsing pace near the plain rate.
fn wheel_acceleration(gap: Duration) -> f32 {
    let slow = WHEEL_ACCELERATION_SLOW.as_secs_f32();
    let fast = WHEEL_ACCELERATION_FAST.as_secs_f32();
    let speed = ((slow - gap.as_secs_f32()) / (slow - fast)).clamp(0., 1.);
    1. + (WHEEL_ACCELERATION_MAX - 1.) * speed * speed
}

/// A wheel event's place in the gesture it belongs to, as the shared clock sees it.
#[derive(Clone, Copy)]
pub(crate) struct WheelGesture {
    /// The gesture's identity: the timestamp of its first event. Equal to the event's own
    /// timestamp exactly when that event started the gesture.
    pub start: Instant,
    /// What to multiply this event's line-granularity delta by, from how fast the gesture is
    /// arriving. One reading per event, shared by every view the event propagates through.
    pub acceleration: f32,
}

/// The clock's state between events: the gesture's identity, the last event that advanced it,
/// what that event was measured in, and its reading.
#[derive(Clone, Copy)]
struct WheelGestureState {
    start: Instant,
    last: Instant,
    granularity: WheelGranularity,
    acceleration: f32,
}

/// The window's shared wheel-gesture clock, lazily stored as a root context. Shared across every
/// scroll view in the window so gesture identity is shared: latching must distinguish starting a
/// gesture from joining one already in flight (the cursor drifting over a nested scroll view
/// mid-gesture, or content scrolling a nested view under a stationary cursor), and per-view clocks
/// cannot tell those apart. Not reactive on purpose, advancing the clock must never re-render a
/// view.
#[derive(Clone, Default)]
pub(crate) struct WheelGestureClock(Rc<Cell<Option<WheelGestureState>>>);

impl WheelGestureClock {
    /// Returns the window's shared clock, lazily creating it in the root context on first use.
    /// Call during render and capture the handle in the wheel handler.
    pub(crate) fn get() -> Self {
        try_consume_root_context::<Self>().unwrap_or_else(|| {
            let clock = Self::default();
            provide_root_context(clock.clone());
            clock
        })
    }

    /// Advances the clock with the event stamped `timestamp` and measured in `granularity`,
    /// returning that event's reading: the gesture it belongs to and how much to accelerate it.
    /// Every scroll view's wheel handler must call this (latching or not): a plain view keeps the
    /// clock honest so a latching descendant can recognise an in-flight gesture it doesn't own.
    ///
    /// One event, one reading. An event propagating through several views calls this once per
    /// view, always with the timestamp the platform stamped it with, which is what tells a repeat
    /// call apart from a genuinely fast one: measuring the gap against a per-view arrival time
    /// would read the second view's gap as zero and accelerate it to the ceiling.
    ///
    /// A rate is only meaningful between events measured the same way, so a change of granularity
    /// restarts the measurement while keeping the gesture's identity. Without that, a trackpad's
    /// momentum tail (pixels, every few milliseconds) would leave the reading at the ceiling for
    /// the next wheel notch, and a single notch would jump a whole viewport.
    pub(crate) fn advance(
        &self,
        timestamp: Instant,
        granularity: WheelGranularity,
    ) -> WheelGesture {
        let state = match self.0.get() {
            // The same event, reaching another view.
            Some(state) if state.last == timestamp => state,
            Some(state) => {
                let gap = timestamp.saturating_duration_since(state.last);
                if gap > WHEEL_GESTURE_WINDOW {
                    Self::opening(timestamp, granularity)
                } else {
                    WheelGestureState {
                        start: state.start,
                        last: timestamp,
                        granularity,
                        acceleration: if granularity == state.granularity {
                            wheel_acceleration(gap)
                        } else {
                            1.
                        },
                    }
                }
            }
            None => Self::opening(timestamp, granularity),
        };
        self.0.set(Some(state));
        WheelGesture {
            start: state.start,
            acceleration: state.acceleration,
        }
    }

    /// The state an event opens a gesture with. It has nothing to measure a rate against, so it
    /// always moves the plain distance: a single notch is a single notch however the last gesture
    /// ended.
    fn opening(timestamp: Instant, granularity: WheelGranularity) -> WheelGestureState {
        WheelGestureState {
            start: timestamp,
            last: timestamp,
            granularity,
            acceleration: 1.,
        }
    }
}

/// Scales a wheel delta by the gesture's speed, so a list of tens of thousands of rows can be
/// crossed with the wheel instead of only by dragging the scrollbar thumb.
///
/// Only a whole-line delta is scaled. A pixel delta comes from a precise device such as a macOS
/// trackpad, which the system has already accelerated, and scaling it again makes the surface fly
/// off at a flick. A sub-line delta is left alone for the same reason: a Windows precision
/// touchpad reports fractions of a line through the same channel a wheel uses, and it too is
/// accelerated before it arrives.
///
/// The result is capped at a viewport per event. Past a screenful the reader has lost their place,
/// and a small pane loses it sooner than a large one, which a bare multiplier ceiling cannot say.
/// An unmeasured viewport has no screenful to cap against, so it caps against nothing rather than
/// against zero.
pub(crate) fn accelerate_wheel_delta(
    delta: f32,
    granularity: WheelGranularity,
    acceleration: f32,
    viewport_size: f32,
) -> f32 {
    if granularity != WheelGranularity::Line || delta.abs() < WheelGranularity::LINE_SIZE as f32 {
        return delta;
    }

    let accelerated = delta * acceleration;
    if viewport_size > 0. {
        accelerated.clamp(-viewport_size, viewport_size)
    } else {
        accelerated
    }
}

/// Accelerates a wheel event's `(x, y)` movement against the viewport each axis scrolls within.
/// The one place a scroll view reaches for: applying the rule per axis at every call site is how
/// the two of them drift apart.
pub(crate) fn accelerate_wheel_movement(
    (x, y): (f32, f32),
    granularity: WheelGranularity,
    gesture: WheelGesture,
    viewport: Area,
) -> (f32, f32) {
    (
        accelerate_wheel_delta(x, granularity, gesture.acceleration, viewport.width()),
        accelerate_wheel_delta(y, granularity, gesture.acceleration, viewport.height()),
    )
}

#[doc(hidden)]
pub fn get_scroll_position_from_wheel(
    wheel_movement: f32,
    inner_size: f32,
    viewport_size: f32,
    scroll_position: f32,
) -> i32 {
    if !is_scrollable(inner_size, viewport_size) {
        return 0;
    }

    let new_position = scroll_position + wheel_movement;

    if new_position >= 0.0 && wheel_movement > 0.0 {
        return 0;
    }

    if new_position <= -(inner_size - viewport_size) && wheel_movement < 0.0 {
        return -(inner_size - viewport_size) as i32;
    }

    new_position as i32
}

#[doc(hidden)]
pub fn get_corrected_scroll_position(
    inner_size: f32,
    viewport_size: f32,
    scroll_position: f32,
) -> f32 {
    // Considering it was a vertical scroll view, the start would be on top and the end on bottom.
    let overscrolled_start = scroll_position > 0.0;
    let overscrolled_end = (-scroll_position + viewport_size) > inner_size;

    if overscrolled_start {
        0f32
    } else if overscrolled_end {
        if viewport_size < inner_size {
            -(inner_size - viewport_size)
        } else {
            0f32
        }
    } else {
        scroll_position
    }
}

#[doc(hidden)]
pub fn get_container_sizes(size: Size) -> (Size, Size) {
    if size == Size::Inner {
        (size.clone(), size)
    } else {
        (Size::percent(100.), Size::fill())
    }
}

/// Whether an axis can scroll: its content (`inner_size`) is larger than the viewport
/// (`viewport_size`) showing it. A zero or unmeasured viewport counts as not-yet-scrollable.
/// The single overflow test every scroll helper (scrollbar visibility, wheel/cursor clamping,
/// wheel latching) shares.
#[doc(hidden)]
pub fn is_scrollable(inner_size: f32, viewport_size: f32) -> bool {
    viewport_size > 0. && viewport_size < inner_size
}

/// Whether the scrollbar is drawn at all: the axis has to overflow *and* the viewport has to be
/// long enough to hold the minimum-sized thumb, since a shorter one could only show a thumb that
/// misreports how much content there is.
#[doc(hidden)]
pub fn is_scrollbar_visible(
    is_scrollbar_enabled: bool,
    inner_size: f32,
    viewport_size: f32,
) -> bool {
    is_scrollbar_enabled
        && is_scrollable(inner_size, viewport_size)
        && viewport_size > MIN_SCROLLBAR_SIZE
}

const MIN_SCROLLBAR_SIZE: f32 = 50.0;

#[doc(hidden)]
pub fn get_scrollbar_pos_and_size(
    inner_size: f32,
    viewport_size: f32,
    scroll_position: f32,
) -> (f32, f32) {
    if !is_scrollable(inner_size, viewport_size) || viewport_size <= MIN_SCROLLBAR_SIZE {
        return (0.0, 0.0);
    }

    let viewable_ratio = viewport_size / inner_size;
    let scrollbar_size = (viewport_size * viewable_ratio).max(MIN_SCROLLBAR_SIZE);

    let available_scroll_range = inner_size - viewport_size;
    let available_thumb_range = viewport_size - scrollbar_size;

    let normalized_scroll = -scroll_position / available_scroll_range;
    let scrollbar_position = normalized_scroll * available_thumb_range;

    (scrollbar_position, scrollbar_size)
}
#[doc(hidden)]
pub fn get_scroll_position_from_cursor(
    cursor_position: f32,
    inner_size: f32,
    viewport_size: f32,
) -> i32 {
    if !is_scrollable(inner_size, viewport_size) || viewport_size <= MIN_SCROLLBAR_SIZE {
        return 0;
    }

    let viewable_ratio = viewport_size / inner_size;
    let scrollbar_size = (viewport_size * viewable_ratio).max(MIN_SCROLLBAR_SIZE);

    let available_scroll_range = inner_size - viewport_size;
    let available_thumb_range = viewport_size - scrollbar_size;

    // Clamp cursor position
    let cursor_clamped = cursor_position.clamp(0.0, available_thumb_range);

    let normalized_scroll = cursor_clamped / available_thumb_range;
    let new_position = -(normalized_scroll * available_scroll_range);

    new_position as i32
}

pub fn handle_key_event(
    key: &Key,
    (mut x, mut y): (f32, f32),
    inner_height: f32,
    inner_width: f32,
    viewport_height: f32,
    viewport_width: f32,
    direction: Direction,
) -> Option<(f32, f32)> {
    let y_page_delta = viewport_height;
    let y_line_delta = y_page_delta / 5.0;
    let x_page_delta = viewport_width;
    let x_line_delta = x_page_delta / 5.0;

    // TODO(tropix126): Handle spacebar and spacebar + shift as Home and End

    match key {
        Key::Named(NamedKey::ArrowUp) => {
            y = get_corrected_scroll_position(inner_height, viewport_height, y + y_line_delta)
        }
        Key::Named(NamedKey::ArrowDown) => {
            y = get_corrected_scroll_position(inner_height, viewport_height, y - y_line_delta)
        }
        Key::Named(NamedKey::PageUp) => {
            y = get_corrected_scroll_position(inner_height, viewport_height, y + y_line_delta)
        }
        Key::Named(NamedKey::PageDown) => {
            y = get_corrected_scroll_position(inner_height, viewport_height, y - y_line_delta)
        }
        Key::Named(NamedKey::ArrowLeft) => {
            x = get_corrected_scroll_position(inner_width, viewport_width, x + x_line_delta)
        }
        Key::Named(NamedKey::ArrowRight) => {
            x = get_corrected_scroll_position(inner_width, viewport_width, x - x_line_delta)
        }
        Key::Named(NamedKey::Home) => {
            if direction == Direction::Vertical {
                y = 0.0;
            } else {
                x = 0.0;
            }
        }
        Key::Named(NamedKey::End) => {
            if direction == Direction::Vertical {
                y = -inner_height;
            } else {
                x = -inner_width;
            }
        }
        _ => return None,
    };
    Some((x, y))
}

#[cfg(test)]
mod tests {
    use std::time::{
        Duration,
        Instant,
    };

    use freya_core::prelude::WheelGranularity;

    use crate::scrollviews::shared::{
        WHEEL_ACCELERATION_FAST,
        WHEEL_ACCELERATION_MAX,
        WHEEL_ACCELERATION_SLOW,
        WHEEL_GESTURE_WINDOW,
        WheelGestureClock,
        accelerate_wheel_delta,
        is_scrollable,
        wheel_acceleration,
    };

    const NOTCH: f32 = WheelGranularity::LINE_SIZE as f32;
    /// Large enough that the viewport cap is not what any curve assertion is measuring.
    const TALL_VIEWPORT: f32 = 10_000.;

    #[test]
    fn wheel_acceleration_ramps_between_a_slow_and_a_fast_gesture() {
        // A gesture at or slower than the floor is not accelerated: a reader looking for
        // something keeps the plain, readable rate.
        assert_eq!(wheel_acceleration(WHEEL_ACCELERATION_SLOW), 1.);
        assert_eq!(
            wheel_acceleration(WHEEL_ACCELERATION_SLOW + Duration::from_millis(50)),
            1.
        );
        // A gesture at or faster than the ceiling is accelerated as far as it goes.
        assert_eq!(
            wheel_acceleration(WHEEL_ACCELERATION_FAST),
            WHEEL_ACCELERATION_MAX
        );
        assert_eq!(wheel_acceleration(Duration::ZERO), WHEEL_ACCELERATION_MAX);
        // In between it ramps, and squaring keeps the middle of the range nearer the plain rate
        // than the ceiling, so an ordinary browsing pace still reads.
        let middle = wheel_acceleration((WHEEL_ACCELERATION_SLOW + WHEEL_ACCELERATION_FAST) / 2);
        assert!(middle > 1. && middle < WHEEL_ACCELERATION_MAX);
        assert!(middle < (1. + WHEEL_ACCELERATION_MAX) / 2.);
    }

    #[test]
    fn a_gestures_first_event_is_never_accelerated() {
        let clock = WheelGestureClock::default();
        let start = Instant::now();

        // Nothing to measure a rate against.
        assert_eq!(
            clock.advance(start, WheelGranularity::Line).acceleration,
            1.
        );

        // Nor after the gesture window lapses: the next event starts a gesture of its own, so a
        // single notch is a single notch however the last gesture ended.
        let fast = start + WHEEL_ACCELERATION_FAST;
        assert!(clock.advance(fast, WheelGranularity::Line).acceleration > 1.);
        let next_gesture = fast + WHEEL_GESTURE_WINDOW + Duration::from_millis(1);
        let gesture = clock.advance(next_gesture, WheelGranularity::Line);
        assert_eq!(gesture.acceleration, 1.);
        assert_eq!(gesture.start, next_gesture);
    }

    #[test]
    fn one_event_reads_the_same_at_every_view_it_reaches() {
        let clock = WheelGestureClock::default();
        let start = Instant::now();
        clock.advance(start, WheelGranularity::Line);

        // An event propagating through nested views advances the clock once per view. Measuring
        // the gap per call would read the second view's gap as zero and accelerate to the
        // ceiling, so the reading is taken once and repeated.
        let slow = start + WHEEL_ACCELERATION_SLOW;
        let first_view = clock.advance(slow, WheelGranularity::Line);
        let second_view = clock.advance(slow, WheelGranularity::Line);
        assert_eq!(first_view.acceleration, second_view.acceleration);
        assert_eq!(first_view.acceleration, 1.);
        assert_eq!(first_view.start, second_view.start);
    }

    #[test]
    fn a_rate_is_only_measured_between_events_measured_the_same_way() {
        let clock = WheelGestureClock::default();
        let start = Instant::now();

        // A trackpad's momentum tail: pixel events a few milliseconds apart, which on their own
        // would read as the fastest possible gesture.
        let mut at = start;
        for _ in 0..4 {
            clock.advance(at, WheelGranularity::Pixel);
            at += WHEEL_ACCELERATION_FAST;
        }
        assert!(clock.advance(at, WheelGranularity::Pixel).acceleration > 1.);

        // A wheel notch arriving hard on its heels is still the same gesture, but the rate it
        // would be scaled by was measured on another device, so the notch moves its plain
        // distance rather than jumping a whole viewport.
        at += WHEEL_ACCELERATION_FAST;
        let notch = clock.advance(at, WheelGranularity::Line);
        assert_eq!(notch.acceleration, 1.);
        assert_eq!(notch.start, start);

        // From there the wheel measures its own rate as usual.
        at += WHEEL_ACCELERATION_FAST;
        assert_eq!(
            clock.advance(at, WheelGranularity::Line).acceleration,
            WHEEL_ACCELERATION_MAX
        );
    }

    #[test]
    fn an_unmeasured_viewport_caps_against_nothing() {
        // A wheel event can arrive before the view has been laid out. There is no screenful to
        // cap against, and capping against zero would swallow the scroll outright.
        assert_eq!(
            accelerate_wheel_delta(-NOTCH, WheelGranularity::Line, 4., 0.),
            -NOTCH * 4.
        );
    }

    #[test]
    fn only_a_whole_line_delta_is_accelerated() {
        // A wheel notch: the device has no acceleration of its own, so this is where it belongs.
        assert_eq!(
            accelerate_wheel_delta(-NOTCH, WheelGranularity::Line, 4., TALL_VIEWPORT),
            -NOTCH * 4.
        );
        // A trackpad's pixel delta is already accelerated by the system.
        assert_eq!(
            accelerate_wheel_delta(-NOTCH, WheelGranularity::Pixel, 4., TALL_VIEWPORT),
            -NOTCH
        );
        // A fraction of a line is a precision touchpad reporting through the wheel's channel,
        // which the system has likewise already accelerated.
        assert_eq!(
            accelerate_wheel_delta(-NOTCH / 4., WheelGranularity::Line, 4., TALL_VIEWPORT),
            -NOTCH / 4.
        );
    }

    #[test]
    fn an_accelerated_delta_is_capped_at_a_viewport() {
        // Past a screenful the reader has lost their place, and a small pane loses it sooner
        // than a large one.
        assert_eq!(
            accelerate_wheel_delta(-NOTCH, WheelGranularity::Line, WHEEL_ACCELERATION_MAX, 120.),
            -120.
        );
        assert_eq!(
            accelerate_wheel_delta(NOTCH, WheelGranularity::Line, WHEEL_ACCELERATION_MAX, 120.),
            120.
        );
    }

    #[test]
    fn is_scrollable_needs_measured_overflow() {
        // Content larger than the viewport showing it: there is something to scroll to.
        assert!(is_scrollable(200., 100.));
        // Content that fits (equal or smaller) than the viewport: nothing to scroll.
        assert!(!is_scrollable(100., 100.));
        assert!(!is_scrollable(80., 100.));
        // An unmeasured (zero) viewport reads as not-yet-scrollable, even with content.
        assert!(!is_scrollable(200., 0.));
    }
}
