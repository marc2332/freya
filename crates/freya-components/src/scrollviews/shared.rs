use freya_core::prelude::*;
use torin::size::Size;

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
    if is_scrollbar_enabled {
        viewport_size > 0. && viewport_size < inner_size
    } else {
        false
    }
}

const MIN_SCROLLBAR_SIZE: f32 = 50.0;

#[doc(hidden)]
pub fn get_scrollbar_pos_and_size(
    inner_size: f32,
    viewport_size: f32,
    scroll_position: f32,
) -> (f32, f32) {
    if viewport_size >= inner_size {
        return (0.0, inner_size);
    }

    let viewable_ratio = viewport_size / inner_size;
    let mut scrollbar_size = viewport_size * viewable_ratio;

    if scrollbar_size < MIN_SCROLLBAR_SIZE {
        scrollbar_size = MIN_SCROLLBAR_SIZE;
    }

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
    if viewport_size >= inner_size {
        return 0;
    }

    let viewable_ratio = viewport_size / inner_size;
    let mut scrollbar_size = viewport_size * viewable_ratio;

    if scrollbar_size < MIN_SCROLLBAR_SIZE {
        scrollbar_size = MIN_SCROLLBAR_SIZE;
    }

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
) -> Option<(f32, f32)> {
    let y_page_delta = viewport_height;
    let y_line_delta = y_page_delta / 5.0;
    let x_page_delta = viewport_width;
    let x_line_delta = x_page_delta / 5.0;

    // TODO(tropix126): Handle spacebar and spacebar + shift as Home and End

    match key {
        Key::ArrowUp => {
            y = get_corrected_scroll_position(inner_height, viewport_height, y + y_line_delta)
        }
        Key::ArrowDown => {
            y = get_corrected_scroll_position(inner_height, viewport_height, y - y_line_delta)
        }
        Key::PageUp => {
            y = get_corrected_scroll_position(inner_height, viewport_height, y + y_line_delta)
        }
        Key::PageDown => {
            y = get_corrected_scroll_position(inner_height, viewport_height, y - y_line_delta)
        }
        Key::ArrowLeft => {
            x = get_corrected_scroll_position(inner_width, viewport_width, x + x_line_delta)
        }
        Key::ArrowRight => {
            x = get_corrected_scroll_position(inner_width, viewport_width, x - x_line_delta)
        }
        Key::Home => {
            y = 0.0;
        }
        Key::End => {
            y = -inner_height;
        }
        _ => return None,
    };
    Some((x, y))
}
