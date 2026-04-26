use jni::{
    JValue,
    jni_sig,
    jni_str,
    objects::JObject,
};
use winit::platform::android::activity::AndroidApp;

/// Show the Android soft keyboard.
pub fn show_keyboard(app: &AndroidApp) -> Result<(), jni::errors::Error> {
    let vm = unsafe { jni::JavaVM::from_raw(app.vm_as_ptr() as *mut _) };
    vm.attach_current_thread(|env| {
        let activity = unsafe { JObject::from_raw(env, app.activity_as_ptr() as *mut _) };

        let input_method_str = env.new_string("input_method")?;
        let imm = env
            .call_method(
                &activity,
                jni_str!("getSystemService"),
                jni_sig!("(Ljava/lang/String;)Ljava/lang/Object;"),
                &[JValue::Object(&input_method_str)],
            )?
            .l()?;

        let window = env
            .call_method(
                &activity,
                jni_str!("getWindow"),
                jni_sig!("()Landroid/view/Window;"),
                &[],
            )?
            .l()?;

        let decor_view = env
            .call_method(
                &window,
                jni_str!("getDecorView"),
                jni_sig!("()Landroid/view/View;"),
                &[],
            )?
            .l()?;

        env.call_method(
            &imm,
            jni_str!("showSoftInput"),
            jni_sig!("(Landroid/view/View;I)Z"),
            &[JValue::Object(&decor_view), JValue::Int(0)],
        )?;

        Ok(())
    })
}

/// Hide the Android soft keyboard.
pub fn hide_keyboard(app: &AndroidApp) -> Result<(), jni::errors::Error> {
    let vm = unsafe { jni::JavaVM::from_raw(app.vm_as_ptr() as *mut _) };
    vm.attach_current_thread(|env| {
        let activity = unsafe { JObject::from_raw(env, app.activity_as_ptr() as *mut _) };

        let input_method_str = env.new_string("input_method")?;
        let imm = env
            .call_method(
                &activity,
                jni_str!("getSystemService"),
                jni_sig!("(Ljava/lang/String;)Ljava/lang/Object;"),
                &[JValue::Object(&input_method_str)],
            )?
            .l()?;

        let window = env
            .call_method(
                &activity,
                jni_str!("getWindow"),
                jni_sig!("()Landroid/view/Window;"),
                &[],
            )?
            .l()?;

        let decor_view = env
            .call_method(
                &window,
                jni_str!("getDecorView"),
                jni_sig!("()Landroid/view/View;"),
                &[],
            )?
            .l()?;

        let window_token = env
            .call_method(
                &decor_view,
                jni_str!("getWindowToken"),
                jni_sig!("()Landroid/os/IBinder;"),
                &[],
            )?
            .l()?;

        env.call_method(
            &imm,
            jni_str!("hideSoftInputFromWindow"),
            jni_sig!("(Landroid/os/IBinder;I)Z"),
            &[JValue::Object(&window_token), JValue::Int(0)],
        )?;

        Ok(())
    })
}
