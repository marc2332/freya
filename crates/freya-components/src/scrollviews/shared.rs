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
    prelude::Direction,
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

/// The window's shared wheel-gesture clock, `(gesture_start, last_event)`, lazily stored as a
/// root context. Shared across every scroll view in the window so gesture identity is shared:
/// latching must distinguish starting a gesture from joining one already in flight (the cursor
/// drifting over a nested scroll view mid-gesture, or content scrolling a nested view under a
/// stationary cursor), and per-view clocks cannot tell those apart. Not reactive on purpose,
/// advancing the clock must never re-render a view.
#[derive(Clone, Default)]
pub(crate) struct WheelGestureClock(Rc<Cell<Option<(Instant, Instant)>>>);

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

    /// Advances the clock with an event observed at `now`, returning the gesture's identity
    /// (its start instant): the in-flight gesture's if this event continues it, else `now`, so
    /// `identity == now` means this event starts a new gesture. Every scroll view's wheel
    /// handler must call this (latching or not): a plain view keeps the clock honest so a
    /// latching descendant can recognise an in-flight gesture it doesn't own. An event
    /// propagating through several views advances the clock once per view, harmlessly (the
    /// repeat calls land within the window and keep the same identity).
    pub(crate) fn advance(&self, now: Instant) -> Instant {
        let start = match self.0.get() {
            Some((start, last)) if now.duration_since(last) <= WHEEL_GESTURE_WINDOW => start,
            _ => now,
        };
        self.0.set(Some((start, now)));
        start
    }
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
    use super::is_scrollable;

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
