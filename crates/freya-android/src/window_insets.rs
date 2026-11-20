use jni::{
    JValue,
    jni_sig,
    jni_str,
    objects::JObject,
};
use winit::platform::android::activity::AndroidApp;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct WindowInsets {
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub left: i32,
}

impl WindowInsets {
    pub fn read(app: &AndroidApp) -> Result<Self, jni::errors::Error> {
        if AndroidApp::sdk_version() < 30 {
            return Ok(Self::default());
        }

        let vm = unsafe { jni::JavaVM::from_raw(app.vm_as_ptr() as *mut _) };
        vm.attach_current_thread(|env| {
            let activity = unsafe { JObject::from_raw(env, app.activity_as_ptr() as *mut _) };
            let manager = env
                .call_method(
                    &activity,
                    jni_str!("getWindowManager"),
                    jni_sig!("()Landroid/view/WindowManager;"),
                    &[],
                )?
                .l()?;
            let metrics = env
                .call_method(
                    &manager,
                    jni_str!("getCurrentWindowMetrics"),
                    jni_sig!("()Landroid/view/WindowMetrics;"),
                    &[],
                )?
                .l()?;
            let window_insets = env
                .call_method(
                    &metrics,
                    jni_str!("getWindowInsets"),
                    jni_sig!("()Landroid/view/WindowInsets;"),
                    &[],
                )?
                .l()?;
            let inset_types = jni_str!("android/view/WindowInsets$Type");
            let system_bars = env
                .call_static_method(inset_types, jni_str!("systemBars"), jni_sig!("()I"), &[])?
                .i()?;
            let display_cutout = env
                .call_static_method(inset_types, jni_str!("displayCutout"), jni_sig!("()I"), &[])?
                .i()?;
            let safe_area = env
                .call_method(
                    &window_insets,
                    jni_str!("getInsets"),
                    jni_sig!("(I)Landroid/graphics/Insets;"),
                    &[JValue::Int(system_bars | display_cutout)],
                )?
                .l()?;

            Ok(Self {
                top: env.get_field(&safe_area, jni_str!("top"), jni_sig!("I"))?.i()?,
                right: env.get_field(&safe_area, jni_str!("right"), jni_sig!("I"))?.i()?,
                bottom: env.get_field(&safe_area, jni_str!("bottom"), jni_sig!("I"))?.i()?,
                left: env.get_field(&safe_area, jni_str!("left"), jni_sig!("I"))?.i()?,
            })
        })
    }
}
