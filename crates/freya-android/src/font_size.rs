use std::rc::Rc;

use freya_core::style::font_size::FontSizeConverter;
use jni::{
    JValue,
    JavaVM,
    errors::Result,
    jni_sig,
    jni_str,
    objects::JObject,
};
use winit::platform::android::activity::AndroidApp;

const COMPLEX_UNIT_SP: i32 = 2;

pub(crate) fn converter(app: &AndroidApp) -> Result<FontSizeConverter> {
    // SAFETY: The AndroidApp JVM pointer is valid for the lifetime of the process.
    let vm = unsafe { JavaVM::from_raw(app.vm_as_ptr().cast()) };
    let (metrics, density) = vm.attach_current_thread(|env| -> Result<_> {
        // SAFETY: AndroidApp owns the activity's global reference.
        let activity = unsafe { JObject::from_raw(env, app.activity_as_ptr().cast()) };
        let resources = env
            .call_method(
                &activity,
                jni_str!("getResources"),
                jni_sig!("()Landroid/content/res/Resources;"),
                &[],
            )?
            .l()?;
        let current_metrics = env
            .call_method(
                &resources,
                jni_str!("getDisplayMetrics"),
                jni_sig!("()Landroid/util/DisplayMetrics;"),
                &[],
            )?
            .l()?;

        let metrics = env.new_object(
            jni_str!("android/util/DisplayMetrics"),
            jni_sig!("()V"),
            &[],
        )?;
        env.call_method(
            &metrics,
            jni_str!("setTo"),
            jni_sig!("(Landroid/util/DisplayMetrics;)V"),
            &[JValue::Object(&current_metrics)],
        )?;
        let density = env
            .get_field(&metrics, jni_str!("density"), jni_sig!("F"))?
            .f()?;
        Ok((env.new_global_ref(metrics)?, density))
    })?;

    Ok(Rc::new(move |size| {
        let pixels = vm.attach_current_thread(|env| -> Result<f32> {
            env.call_static_method(
                jni_str!("android/util/TypedValue"),
                jni_str!("applyDimension"),
                jni_sig!("(IFLandroid/util/DisplayMetrics;)F"),
                &[
                    JValue::Int(COMPLEX_UNIT_SP),
                    JValue::Float(size),
                    JValue::Object(metrics.as_ref()),
                ],
            )?
            .f()
        });

        match pixels {
            Ok(pixels) if density > 0. && density.is_finite() && pixels.is_finite() => {
                pixels / density
            }
            _ => size,
        }
    }))
}
