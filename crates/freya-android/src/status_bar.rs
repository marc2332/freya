use jni::objects::{
    JObject,
    JValue,
};
use winit::platform::android::activity::AndroidApp;

/// Set whether the Android status bar uses light appearance (dark icons for light backgrounds).
pub fn set_status_bar_light(app: &AndroidApp, light: bool) -> Result<(), jni::errors::Error> {
    let vm = unsafe { jni::JavaVM::from_raw(app.vm_as_ptr() as *mut _) }?;
    let env = vm.attach_current_thread()?;
    let activity: JObject = (app.activity_as_ptr() as *mut jni::sys::_jobject).into();

    let window = env
        .call_method(activity, "getWindow", "()Landroid/view/Window;", &[])?
        .l()?;

    let controller = env
        .call_method(
            window,
            "getInsetsController",
            "()Landroid/view/WindowInsetsController;",
            &[],
        )?
        .l()?;

    // APPEARANCE_LIGHT_STATUS_BARS = 8
    let appearance = if light { 8 } else { 0 };
    env.call_method(
        controller,
        "setSystemBarsAppearance",
        "(II)V",
        &[JValue::Int(appearance), JValue::Int(8)],
    )?;

    Ok(())
}
