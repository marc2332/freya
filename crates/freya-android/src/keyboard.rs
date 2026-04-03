use jni::objects::{
    JObject,
    JValue,
};
use winit::platform::android::activity::AndroidApp;

/// Show the Android soft keyboard.
pub fn show_keyboard(app: &AndroidApp) -> Result<(), jni::errors::Error> {
    let vm = unsafe { jni::JavaVM::from_raw(app.vm_as_ptr() as *mut _) }?;
    let env = vm.attach_current_thread()?;
    let activity: JObject = (app.activity_as_ptr() as *mut jni::sys::_jobject).into();

    let imm = env
        .call_method(
            activity,
            "getSystemService",
            "(Ljava/lang/String;)Ljava/lang/Object;",
            &[JValue::Object(env.new_string("input_method")?.into())],
        )?
        .l()?;

    let window = env
        .call_method(activity, "getWindow", "()Landroid/view/Window;", &[])?
        .l()?;

    let decor_view = env
        .call_method(window, "getDecorView", "()Landroid/view/View;", &[])?
        .l()?;

    env.call_method(
        imm,
        "showSoftInput",
        "(Landroid/view/View;I)Z",
        &[JValue::Object(decor_view.into()), JValue::Int(0)],
    )?;

    Ok(())
}

/// Hide the Android soft keyboard.
pub fn hide_keyboard(app: &AndroidApp) -> Result<(), jni::errors::Error> {
    let vm = unsafe { jni::JavaVM::from_raw(app.vm_as_ptr() as *mut _) }?;
    let env = vm.attach_current_thread()?;
    let activity: JObject = (app.activity_as_ptr() as *mut jni::sys::_jobject).into();

    let imm = env
        .call_method(
            activity,
            "getSystemService",
            "(Ljava/lang/String;)Ljava/lang/Object;",
            &[JValue::Object(env.new_string("input_method")?.into())],
        )?
        .l()?;

    let window = env
        .call_method(activity, "getWindow", "()Landroid/view/Window;", &[])?
        .l()?;

    let decor_view = env
        .call_method(window, "getDecorView", "()Landroid/view/View;", &[])?
        .l()?;

    let window_token = env
        .call_method(decor_view, "getWindowToken", "()Landroid/os/IBinder;", &[])?
        .l()?;

    env.call_method(
        imm,
        "hideSoftInputFromWindow",
        "(Landroid/os/IBinder;I)Z",
        &[JValue::Object(window_token.into()), JValue::Int(0)],
    )?;

    Ok(())
}
