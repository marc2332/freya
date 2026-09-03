use std::{
    cell::{
        Cell,
        RefCell,
    },
    ffi::{
        CStr,
        c_char,
        c_int,
        c_void,
    },
    ptr,
    str::FromStr,
};

use freya_core::{
    integration::*,
    prelude::{
        Code,
        Key,
        Modifiers,
    },
};
use torin::prelude::CursorPoint;

use crate::emscripten::*;

/// State pushed by the browser callbacks and drained once per frame.
#[derive(Default)]
pub struct BrowserState {
    pub events: Vec<PlatformEvent>,
    pub resized: Option<(i32, i32, f64)>,
    pub focused: Option<bool>,
}

impl BrowserState {
    pub fn take() -> Self {
        BROWSER.with_borrow_mut(std::mem::take)
    }
}

thread_local! {
    static BROWSER: RefCell<BrowserState> = RefCell::new(BrowserState::default());

    /// Device pixel ratio, updated on every resize.
    static PIXEL_RATIO: Cell<f64> = const { Cell::new(1.) };

    /// Last cursor position, in physical pixels.
    static LAST_CURSOR: Cell<CursorPoint> = const { Cell::new(CursorPoint::new(0., 0.)) };

    /// Whether the hidden IME input holds the browser focus.
    static IME_FOCUSED: Cell<bool> = const { Cell::new(false) };
}

/// Whether typing is currently routed through the hidden IME input.
pub fn ime_focused() -> bool {
    IME_FOCUSED.get()
}

pub fn set_ime_focused(focused: bool) {
    IME_FOCUSED.set(focused);
}

fn push(event: PlatformEvent) {
    BROWSER.with_borrow_mut(|browser| browser.events.push(event));
}

/// Resizes the drawing buffer to physical pixels, returning the new size and pixel ratio.
pub fn sync_canvas_size() -> (i32, i32, f64) {
    let mut css_width = 0.;
    let mut css_height = 0.;
    let pixel_ratio = unsafe {
        emscripten_get_element_css_size(TARGET_CANVAS, &mut css_width, &mut css_height);
        emscripten_get_device_pixel_ratio()
    };

    let width = (css_width * pixel_ratio) as i32;
    let height = (css_height * pixel_ratio) as i32;

    unsafe { emscripten_set_canvas_element_size(TARGET_CANVAS, width, height) };
    PIXEL_RATIO.set(pixel_ratio);
    BROWSER.with_borrow_mut(|browser| browser.resized = Some((width, height, pixel_ratio)));

    (width, height, pixel_ratio)
}

/// Registers every browser callback on the calling thread.
pub fn listen() {
    macro_rules! listen {
        ($($register:ident($target:expr, $callback:ident)),* $(,)?) => {
            unsafe {
                $(
                    $register(
                        $target,
                        ptr::null_mut(),
                        false,
                        $callback,
                        EM_CALLBACK_THREAD_CONTEXT_CALLING_THREAD,
                    );
                )*
            }
        };
    }

    listen!(
        emscripten_set_mousedown_callback_on_thread(TARGET_CANVAS, on_mouse),
        emscripten_set_mouseup_callback_on_thread(TARGET_WINDOW, on_mouse),
        emscripten_set_mousemove_callback_on_thread(TARGET_CANVAS, on_mouse),
        emscripten_set_wheel_callback_on_thread(TARGET_CANVAS, on_wheel),
        emscripten_set_keydown_callback_on_thread(TARGET_WINDOW, on_key),
        emscripten_set_keyup_callback_on_thread(TARGET_WINDOW, on_key),
        emscripten_set_touchstart_callback_on_thread(TARGET_CANVAS, on_touch),
        emscripten_set_touchmove_callback_on_thread(TARGET_CANVAS, on_touch),
        emscripten_set_touchend_callback_on_thread(TARGET_CANVAS, on_touch),
        emscripten_set_touchcancel_callback_on_thread(TARGET_CANVAS, on_touch),
        emscripten_set_resize_callback_on_thread(TARGET_WINDOW, on_resize),
        emscripten_set_focus_callback_on_thread(TARGET_WINDOW, on_focus),
        emscripten_set_blur_callback_on_thread(TARGET_WINDOW, on_focus),
    );
}

/// Converts canvas coordinates from CSS to physical pixels.
fn point(x: c_int, y: c_int) -> CursorPoint {
    let pixel_ratio = PIXEL_RATIO.get();
    CursorPoint::new(x as f64 * pixel_ratio, y as f64 * pixel_ratio)
}

fn read_string(bytes: &[c_char]) -> std::borrow::Cow<'_, str> {
    unsafe { CStr::from_ptr(bytes.as_ptr()) }.to_string_lossy()
}

fn mouse_button(button: u16) -> MouseButton {
    match button {
        0 => MouseButton::Left,
        1 => MouseButton::Middle,
        2 => MouseButton::Right,
        3 => MouseButton::Back,
        4 => MouseButton::Forward,
        other => MouseButton::Other(other),
    }
}

extern "C" fn on_mouse(
    event_type: c_int,
    event: *const EmscriptenMouseEvent,
    _user_data: *mut c_void,
) -> bool {
    let event = unsafe { &*event };

    let name = match event_type {
        EMSCRIPTEN_EVENT_MOUSEDOWN => MouseEventName::MouseDown,
        EMSCRIPTEN_EVENT_MOUSEUP => MouseEventName::MouseUp,
        _ => MouseEventName::MouseMove,
    };

    let cursor = if name == MouseEventName::MouseUp {
        LAST_CURSOR.get()
    } else {
        let cursor = point(event.target_x, event.target_y);
        LAST_CURSOR.set(cursor);
        cursor
    };

    push(PlatformEvent::Mouse {
        name,
        cursor,
        button: Some(mouse_button(event.button)),
    });

    false
}

extern "C" fn on_wheel(
    _event_type: c_int,
    event: *const EmscriptenWheelEvent,
    _user_data: *mut c_void,
) -> bool {
    let event = unsafe { &*event };

    let scale = match event.delta_mode {
        DOM_DELTA_LINE => 20.,
        DOM_DELTA_PAGE => 400.,
        _ => 1.,
    };

    push(PlatformEvent::Wheel {
        name: WheelEventName::Wheel,
        scroll: CursorPoint::new(-event.delta_x * scale, -event.delta_y * scale),
        cursor: point(event.mouse.target_x, event.mouse.target_y),
        source: WheelSource::Device,
    });

    true
}

extern "C" fn on_key(
    event_type: c_int,
    event: *const EmscriptenKeyboardEvent,
    _user_data: *mut c_void,
) -> bool {
    let event = unsafe { &*event };

    let name = if event_type == EMSCRIPTEN_EVENT_KEYUP {
        KeyboardEventName::KeyUp
    } else {
        KeyboardEventName::KeyDown
    };

    let key = read_string(&event.key);
    let key = Key::from_str(&key).unwrap_or_else(|_| Key::Character(key.into_owned()));
    let code = Code::from_str(&read_string(&event.code)).unwrap_or(Code::Unidentified);

    let mut modifiers = Modifiers::empty();
    modifiers.set(Modifiers::CONTROL, event.ctrl_key);
    modifiers.set(Modifiers::SHIFT, event.shift_key);
    modifiers.set(Modifiers::ALT, event.alt_key);
    modifiers.set(Modifiers::META, event.meta_key);

    let alt_graph = modifiers.contains(Modifiers::CONTROL | Modifiers::ALT);
    let is_shortcut = modifiers.intersects(Modifiers::CONTROL | Modifiers::META) && !alt_graph;
    let select_all = is_shortcut
        && matches!(&key, Key::Character(character) if character.eq_ignore_ascii_case("a"));
    let emitted_by_ime = ime_focused()
        && name == KeyboardEventName::KeyDown
        && matches!(key, Key::Character(_))
        && !is_shortcut;

    if !emitted_by_ime {
        push(PlatformEvent::Keyboard {
            name,
            key,
            code,
            modifiers,
        });
    }

    code == Code::Tab || select_all
}

extern "C" fn on_touch(
    event_type: c_int,
    event: *const EmscriptenTouchEvent,
    _user_data: *mut c_void,
) -> bool {
    let event = unsafe { &*event };

    let (name, phase) = match event_type {
        EMSCRIPTEN_EVENT_TOUCHSTART => (TouchEventName::TouchStart, TouchPhase::Started),
        EMSCRIPTEN_EVENT_TOUCHEND => (TouchEventName::TouchEnd, TouchPhase::Ended),
        EMSCRIPTEN_EVENT_TOUCHCANCEL => (TouchEventName::TouchCancel, TouchPhase::Cancelled),
        _ => (TouchEventName::TouchMove, TouchPhase::Moved),
    };

    let touches = event.num_touches.clamp(0, event.touches.len() as i32) as usize;
    for touch in event.touches.iter().take(touches).filter(|t| t.is_changed) {
        push(PlatformEvent::Touch {
            name,
            location: point(touch.target_x, touch.target_y),
            finger_id: touch.identifier as u64,
            phase,
            force: None,
        });
    }

    true
}

extern "C" fn on_resize(
    _event_type: c_int,
    _event: *const c_void,
    _user_data: *mut c_void,
) -> bool {
    sync_canvas_size();
    false
}

extern "C" fn on_focus(event_type: c_int, _event: *const c_void, _user_data: *mut c_void) -> bool {
    BROWSER.with_borrow_mut(|browser| {
        browser.focused = Some(event_type == EMSCRIPTEN_EVENT_FOCUS);
    });
    false
}
