mod scroll_bar;
mod scroll_thumb;
mod scroll_view;
mod virtual_scroll_view;

use freya_elements::events::{keyboard::Key, KeyboardEvent};
pub use scroll_bar::*;
pub use scroll_thumb::*;
pub use scroll_view::*;
pub use virtual_scroll_view::*;

// Holding alt while scrolling makes it 5x faster (VSCode behavior).
#[doc(hidden)]
pub const SCROLL_SPEED_MULTIPLIER: f32 = 5.0;

#[doc(hidden)]
#[derive(Debug, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
}

#[doc(hidden)]
pub fn get_container_size(is_scrollbar_visible: bool, scrollbar_size: &str) -> String {
    if is_scrollbar_visible {
        format!("calc(100% - {scrollbar_size})")
    } else {
        "100%".to_string()
    }
}

#[doc(hidden)]
pub fn is_scrollbar_visible(
    is_scrollbar_enabled: bool,
    inner_size: f32,
    viewport_size: f32,
) -> bool {
    if is_scrollbar_enabled {
        viewport_size < inner_size
    } else {
        false
    }
}

#[doc(hidden)]
pub fn get_scrollbar_pos_and_size(
    inner_size: f32,
    viewport_size: f32,
    scroll_position: f32,
) -> (f32, f32) {
    let scrollbar_height = if viewport_size >= inner_size {
        inner_size
    } else {
        let viewable_ratio_height = viewport_size / inner_size;
        viewport_size * viewable_ratio_height
    };
    let scroll_position = (100.0 / inner_size) * -scroll_position;
    let scrollbar_position = (scroll_position / 100.0) * viewport_size;
    (scrollbar_position, scrollbar_height)
}

#[doc(hidden)]
pub fn get_scroll_position_from_cursor(
    cursor_position: f32,
    inner_size: f32,
    viewport_size: f32,
) -> i32 {
    let per = 100.0 / viewport_size * cursor_position;

    if viewport_size >= inner_size {
        return 0;
    }

    let new_position = -(inner_size / 100.0 * per);

    if new_position >= 0.0 {
        return 0;
    }

    if new_position <= -(inner_size - viewport_size) {
        return -(inner_size - viewport_size) as i32;
    }
    new_position as i32
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

/// Limit the scroll position to the scroll view bounds to avoid overflows
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

pub fn manage_key_event(
    e: KeyboardEvent,
    (mut x, mut y): (f32, f32),
    inner_height: f32,
    inner_width: f32,
    viewport_height: f32,
    viewport_width: f32,
) -> (f32, f32) {
    let y_page_delta = viewport_height;
    let y_line_delta = y_page_delta / 5.0;
    let x_page_delta = viewport_width;
    let x_line_delta = x_page_delta / 5.0;

    // TODO(tropix126): Handle spacebar and spacebar + shift as Home and End

    match e.key {
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
        _ => {}
    };

    (x, y)
}
