mod scroll_view;
mod virtual_scroll_view;

pub use scroll_view::*;
pub use virtual_scroll_view::*;

pub const SCROLLBAR_SIZE: u8 = 15;

#[derive(Debug, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
}

pub fn get_container_size(is_scrollbar_visible: bool) -> String {
    if is_scrollbar_visible {
        format!("calc(100% - {SCROLLBAR_SIZE})")
    } else {
        "100%".to_string()
    }
}

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

pub fn get_scroll_position_from_wheel(
    wheel_movement: f32,
    inner_size: f32,
    viewport_size: f32,
    scroll_position: f32,
) -> i32 {
    if viewport_size >= inner_size {
        return 0;
    }

    let new_position = scroll_position + (wheel_movement as f32 * 20.0);

    if new_position >= 0.0 && wheel_movement > 0.0 {
        return 0;
    }

    if new_position <= -(inner_size - viewport_size) && wheel_movement < 0.0 {
        return -(inner_size - viewport_size) as i32;
    }

    new_position as i32
}
