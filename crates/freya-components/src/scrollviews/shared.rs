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

#[doc(hidden)]
pub fn get_scroll_position_from_wheel(
    wheel_movement: f32,
    inner_size: f32,
    viewport_size: f32,
    scroll_position: f32,
) -> i32 {
    if viewport_size >= inner_size {
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

#[doc(hidden)]
pub fn is_scrollbar_visible(
    is_scrollbar_enabled: bool,
    inner_size: f32,
    viewport_size: f32,
) -> bool {
    is_scrollbar_enabled && viewport_size > MIN_SCROLLBAR_SIZE && viewport_size < inner_size
}

const MIN_SCROLLBAR_SIZE: f32 = 50.0;

pub const SCROLLBAR_MARGIN: f32 = 3.0;

/// Thumb size, scrollable content range and thumb travel range inside the track.
fn get_thumb_ranges(inner_size: f32, viewport_size: f32) -> (f32, f32, f32) {
    let track_size = viewport_size - SCROLLBAR_MARGIN * 2.0;
    let viewable_ratio = viewport_size / inner_size;
    let minimum_thumb_size = if track_size > MIN_SCROLLBAR_SIZE {
        MIN_SCROLLBAR_SIZE
    } else {
        0.
    };
    let scrollbar_size = (track_size * viewable_ratio).max(minimum_thumb_size);

    let available_scroll_range = inner_size - viewport_size;
    let available_thumb_range = track_size - scrollbar_size;

    (
        scrollbar_size,
        available_scroll_range,
        available_thumb_range,
    )
}

#[doc(hidden)]
pub fn get_scrollbar_pos_and_size(
    inner_size: f32,
    viewport_size: f32,
    scroll_position: f32,
) -> (f32, f32) {
    if viewport_size <= MIN_SCROLLBAR_SIZE || viewport_size >= inner_size {
        return (0.0, 0.0);
    }

    let (scrollbar_size, available_scroll_range, available_thumb_range) =
        get_thumb_ranges(inner_size, viewport_size);

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
    if viewport_size <= MIN_SCROLLBAR_SIZE || viewport_size >= inner_size {
        return 0;
    }

    let (_, available_scroll_range, available_thumb_range) =
        get_thumb_ranges(inner_size, viewport_size);

    // Clamp cursor position
    let cursor_clamped = (cursor_position - SCROLLBAR_MARGIN).clamp(0.0, available_thumb_range);

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
    use crate::scrollviews::shared::{
        SCROLLBAR_MARGIN,
        get_scroll_position_from_cursor,
        get_scrollbar_pos_and_size,
    };

    #[test]
    fn scrollbar_can_drag_when_the_track_is_smaller_than_the_minimum_thumb() {
        for viewport_size in 51..=56 {
            let viewport_size = viewport_size as f32;
            let inner_size = viewport_size * 2.;
            let track_size = viewport_size - SCROLLBAR_MARGIN * 2.;

            let (_, thumb_size) = get_scrollbar_pos_and_size(inner_size, viewport_size, 0.);

            assert!(thumb_size < track_size);
            assert_eq!(
                get_scroll_position_from_cursor(viewport_size, inner_size, viewport_size),
                -viewport_size as i32
            );
        }
    }
}
