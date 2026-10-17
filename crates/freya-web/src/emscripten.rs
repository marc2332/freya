use std::ffi::{
    CStr,
    CString,
    c_char,
    c_double,
    c_int,
    c_uint,
    c_ushort,
    c_void,
};

/// Runs a snippet of JavaScript.
pub fn run_script(script: &str) {
    if let Ok(script) = CString::new(script) {
        unsafe { emscripten_run_script(script.as_ptr()) };
    }
}

/// Runs a snippet of JavaScript and returns its result as a string.
pub fn run_script_string(script: &str) -> Option<String> {
    let script = CString::new(script).ok()?;
    let result = unsafe { emscripten_run_script_string(script.as_ptr()) };
    if result.is_null() {
        return None;
    }

    let result = unsafe { CStr::from_ptr(result) };
    Some(result.to_string_lossy().into_owned())
}

pub const EMSCRIPTEN_EVENT_MOUSEDOWN: c_int = 5;
pub const EMSCRIPTEN_EVENT_MOUSEUP: c_int = 6;
pub const EMSCRIPTEN_EVENT_KEYUP: c_int = 3;
pub const EMSCRIPTEN_EVENT_FOCUS: c_int = 13;
pub const EMSCRIPTEN_EVENT_TOUCHSTART: c_int = 22;
pub const EMSCRIPTEN_EVENT_TOUCHEND: c_int = 23;
pub const EMSCRIPTEN_EVENT_TOUCHCANCEL: c_int = 25;

pub const DOM_DELTA_LINE: c_uint = 1;
pub const DOM_DELTA_PAGE: c_uint = 2;

pub const EMSCRIPTEN_RESULT_SUCCESS: c_int = 0;

pub const EM_CALLBACK_THREAD_CONTEXT_CALLING_THREAD: PthreadT = 0x2 as PthreadT;

/// Sentinel pointer Emscripten uses for the window.
pub const TARGET_WINDOW: *const c_char = 2 as *const c_char;
pub const TARGET_CANVAS: *const c_char = c"#canvas".as_ptr();

pub type PthreadT = *mut c_void;
pub type WebGLContext = c_int;

const SHORT_STRING_LEN: usize = 32;

#[repr(C)]
pub struct EmscriptenKeyboardEvent {
    pub timestamp: c_double,
    pub location: c_uint,
    pub ctrl_key: bool,
    pub shift_key: bool,
    pub alt_key: bool,
    pub meta_key: bool,
    pub repeat: bool,
    pub char_code: c_uint,
    pub key_code: c_uint,
    pub which: c_uint,
    pub key: [c_char; SHORT_STRING_LEN],
    pub code: [c_char; SHORT_STRING_LEN],
    pub char_value: [c_char; SHORT_STRING_LEN],
    pub locale: [c_char; SHORT_STRING_LEN],
}

#[repr(C)]
pub struct EmscriptenMouseEvent {
    pub timestamp: c_double,
    pub screen_x: c_int,
    pub screen_y: c_int,
    pub client_x: c_int,
    pub client_y: c_int,
    pub ctrl_key: bool,
    pub shift_key: bool,
    pub alt_key: bool,
    pub meta_key: bool,
    pub button: c_ushort,
    pub buttons: c_ushort,
    pub movement_x: c_int,
    pub movement_y: c_int,
    pub target_x: c_int,
    pub target_y: c_int,
    pub canvas_x: c_int,
    pub canvas_y: c_int,
    pub padding: c_int,
}

#[repr(C)]
pub struct EmscriptenWheelEvent {
    pub mouse: EmscriptenMouseEvent,
    pub delta_x: c_double,
    pub delta_y: c_double,
    pub delta_z: c_double,
    pub delta_mode: c_uint,
}

#[repr(C)]
pub struct EmscriptenTouchPoint {
    pub identifier: c_int,
    pub screen_x: c_int,
    pub screen_y: c_int,
    pub client_x: c_int,
    pub client_y: c_int,
    pub page_x: c_int,
    pub page_y: c_int,
    pub is_changed: bool,
    pub on_target: bool,
    pub target_x: c_int,
    pub target_y: c_int,
    pub canvas_x: c_int,
    pub canvas_y: c_int,
}

#[repr(C)]
pub struct EmscriptenTouchEvent {
    pub timestamp: c_double,
    pub num_touches: c_int,
    pub ctrl_key: bool,
    pub shift_key: bool,
    pub alt_key: bool,
    pub meta_key: bool,
    pub touches: [EmscriptenTouchPoint; 32],
}

#[repr(C)]
#[derive(Default)]
pub struct EmscriptenWebGLContextAttributes {
    pub alpha: bool,
    pub depth: bool,
    pub stencil: bool,
    pub antialias: bool,
    pub premultiplied_alpha: bool,
    pub preserve_drawing_buffer: bool,
    pub power_preference: c_int,
    pub fail_if_major_performance_caveat: bool,
    pub major_version: c_int,
    pub minor_version: c_int,
    pub enable_extensions_by_default: bool,
    pub explicit_swap_control: bool,
    pub proxy_context_to_main_thread: c_int,
    pub render_via_offscreen_back_buffer: bool,
}

pub type MouseCallback = extern "C" fn(c_int, *const EmscriptenMouseEvent, *mut c_void) -> bool;
pub type WheelCallback = extern "C" fn(c_int, *const EmscriptenWheelEvent, *mut c_void) -> bool;
pub type KeyCallback = extern "C" fn(c_int, *const EmscriptenKeyboardEvent, *mut c_void) -> bool;
pub type TouchCallback = extern "C" fn(c_int, *const EmscriptenTouchEvent, *mut c_void) -> bool;
pub type UiCallback = extern "C" fn(c_int, *const c_void, *mut c_void) -> bool;
pub type FocusCallback = extern "C" fn(c_int, *const c_void, *mut c_void) -> bool;
pub type MainLoopCallback = extern "C" fn();

unsafe extern "C" {
    pub fn emscripten_set_mousedown_callback_on_thread(
        target: *const c_char,
        user_data: *mut c_void,
        use_capture: bool,
        callback: MouseCallback,
        thread: PthreadT,
    ) -> c_int;

    pub fn emscripten_set_mouseup_callback_on_thread(
        target: *const c_char,
        user_data: *mut c_void,
        use_capture: bool,
        callback: MouseCallback,
        thread: PthreadT,
    ) -> c_int;

    pub fn emscripten_set_mousemove_callback_on_thread(
        target: *const c_char,
        user_data: *mut c_void,
        use_capture: bool,
        callback: MouseCallback,
        thread: PthreadT,
    ) -> c_int;

    pub fn emscripten_set_wheel_callback_on_thread(
        target: *const c_char,
        user_data: *mut c_void,
        use_capture: bool,
        callback: WheelCallback,
        thread: PthreadT,
    ) -> c_int;

    pub fn emscripten_set_keydown_callback_on_thread(
        target: *const c_char,
        user_data: *mut c_void,
        use_capture: bool,
        callback: KeyCallback,
        thread: PthreadT,
    ) -> c_int;

    pub fn emscripten_set_keyup_callback_on_thread(
        target: *const c_char,
        user_data: *mut c_void,
        use_capture: bool,
        callback: KeyCallback,
        thread: PthreadT,
    ) -> c_int;

    pub fn emscripten_set_touchstart_callback_on_thread(
        target: *const c_char,
        user_data: *mut c_void,
        use_capture: bool,
        callback: TouchCallback,
        thread: PthreadT,
    ) -> c_int;

    pub fn emscripten_set_touchmove_callback_on_thread(
        target: *const c_char,
        user_data: *mut c_void,
        use_capture: bool,
        callback: TouchCallback,
        thread: PthreadT,
    ) -> c_int;

    pub fn emscripten_set_touchend_callback_on_thread(
        target: *const c_char,
        user_data: *mut c_void,
        use_capture: bool,
        callback: TouchCallback,
        thread: PthreadT,
    ) -> c_int;

    pub fn emscripten_set_touchcancel_callback_on_thread(
        target: *const c_char,
        user_data: *mut c_void,
        use_capture: bool,
        callback: TouchCallback,
        thread: PthreadT,
    ) -> c_int;

    pub fn emscripten_set_resize_callback_on_thread(
        target: *const c_char,
        user_data: *mut c_void,
        use_capture: bool,
        callback: UiCallback,
        thread: PthreadT,
    ) -> c_int;

    pub fn emscripten_set_focus_callback_on_thread(
        target: *const c_char,
        user_data: *mut c_void,
        use_capture: bool,
        callback: FocusCallback,
        thread: PthreadT,
    ) -> c_int;

    pub fn emscripten_set_blur_callback_on_thread(
        target: *const c_char,
        user_data: *mut c_void,
        use_capture: bool,
        callback: FocusCallback,
        thread: PthreadT,
    ) -> c_int;

    pub fn emscripten_set_canvas_element_size(
        target: *const c_char,
        width: c_int,
        height: c_int,
    ) -> c_int;

    pub fn emscripten_get_element_css_size(
        target: *const c_char,
        width: *mut c_double,
        height: *mut c_double,
    ) -> c_int;

    pub fn emscripten_get_device_pixel_ratio() -> c_double;

    pub fn emscripten_set_main_loop(
        callback: MainLoopCallback,
        fps: c_int,
        simulate_infinite_loop: c_int,
    );

    pub fn emscripten_run_script(script: *const c_char);

    pub fn emscripten_run_script_string(script: *const c_char) -> *mut c_char;

    pub fn emscripten_GetProcAddress(name: *const c_char) -> *const c_void;

    pub fn emscripten_webgl_init_context_attributes(
        attributes: *mut EmscriptenWebGLContextAttributes,
    );

    pub fn emscripten_webgl_create_context(
        target: *const c_char,
        attributes: *const EmscriptenWebGLContextAttributes,
    ) -> WebGLContext;

    pub fn emscripten_webgl_make_context_current(context: WebGLContext) -> c_int;
}
