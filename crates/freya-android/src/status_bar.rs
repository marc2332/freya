use jni::{
    JValue,
    jni_sig,
    jni_str,
    objects::JObject,
};
use winit::platform::android::activity::AndroidApp;

/// Set whether the Android status bar uses light appearance (dark icons for light backgrounds).
pub fn set_status_bar_light(app: &AndroidApp, light: bool) -> Result<(), jni::errors::Error> {
    let vm = unsafe { jni::JavaVM::from_raw(app.vm_as_ptr() as *mut _) };
    vm.attach_current_thread(|env| {
        let activity = unsafe { JObject::from_raw(env, app.activity_as_ptr() as *mut _) };

        let window = env
            .call_method(
                &activity,
                jni_str!("getWindow"),
                jni_sig!("()Landroid/view/Window;"),
                &[],
            )?
            .l()?;

        let controller = env
            .call_method(
                &window,
                jni_str!("getInsetsController"),
                jni_sig!("()Landroid/view/WindowInsetsController;"),
                &[],
            )?
            .l()?;

        // APPEARANCE_LIGHT_STATUS_BARS = 8
        let appearance = if light { 8 } else { 0 };
        env.call_method(
            &controller,
            jni_str!("setSystemBarsAppearance"),
            jni_sig!("(II)V"),
            &[JValue::Int(appearance), JValue::Int(8)],
        )?;

        Ok(())
    })
}
