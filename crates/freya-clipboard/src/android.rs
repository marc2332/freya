use std::error::Error;

use copypasta::ClipboardProvider;
use jni::{
    Env,
    JValue,
    jni_sig,
    jni_str,
    objects::{
        JObject,
        JString,
    },
};

/// Clipboard backed by Android's clipboard manager.
///
/// Requires an Android runtime that initializes `ndk-context`, such as `android-activity`.
/// Clipboard reads are subject to Android's foreground access restrictions.
pub struct AndroidClipboard;

impl AndroidClipboard {
    fn with_manager<T>(
        run: impl FnOnce(
            &mut Env<'_>,
            JObject<'_>,
            JObject<'_>,
        ) -> Result<T, Box<dyn Error + Send + Sync>>,
    ) -> Result<T, Box<dyn Error + Send + Sync>> {
        let context = ndk_context::android_context();
        let vm = unsafe { jni::JavaVM::from_raw(context.vm().cast()) };
        vm.attach_current_thread(|env| {
            let activity = unsafe { JObject::from_raw(env, context.context().cast()) };
            let service = env.new_string("clipboard")?;
            let manager = env
                .call_method(
                    &activity,
                    jni_str!("getSystemService"),
                    jni_sig!("(Ljava/lang/String;)Ljava/lang/Object;"),
                    &[JValue::Object(&service)],
                )?
                .l()?;

            if manager.is_null() {
                return Err("Android clipboard service is unavailable".into());
            }

            run(env, manager, activity)
        })
    }
}

impl ClipboardProvider for AndroidClipboard {
    fn get_contents(&mut self) -> Result<String, Box<dyn Error + Send + Sync>> {
        Self::with_manager(|env, manager, activity| {
            let clip = env
                .call_method(
                    &manager,
                    jni_str!("getPrimaryClip"),
                    jni_sig!("()Landroid/content/ClipData;"),
                    &[],
                )?
                .l()?;

            if clip.is_null() {
                return Err("Android clipboard is empty or inaccessible".into());
            }

            let item_count = env
                .call_method(&clip, jni_str!("getItemCount"), jni_sig!("()I"), &[])?
                .i()?;
            if item_count == 0 {
                return Err("Android clipboard is empty".into());
            }

            let item = env
                .call_method(
                    &clip,
                    jni_str!("getItemAt"),
                    jni_sig!("(I)Landroid/content/ClipData$Item;"),
                    &[JValue::Int(0)],
                )?
                .l()?;
            let text = env
                .call_method(
                    &item,
                    jni_str!("coerceToText"),
                    jni_sig!("(Landroid/content/Context;)Ljava/lang/CharSequence;"),
                    &[JValue::Object(&activity)],
                )?
                .l()?;

            if text.is_null() {
                return Err("Android clipboard has no text".into());
            }

            let text = env
                .call_method(
                    &text,
                    jni_str!("toString"),
                    jni_sig!("()Ljava/lang/String;"),
                    &[],
                )?
                .l()?;
            let text = env.cast_local::<JString>(text)?;
            Ok(text.try_to_string(env)?)
        })
    }

    fn set_contents(&mut self, contents: String) -> Result<(), Box<dyn Error + Send + Sync>> {
        Self::with_manager(|env, manager, _activity| {
            let label = env.new_string("text")?;
            let text = env.new_string(contents)?;
            let clip = env
                .call_static_method(
                    jni_str!("android/content/ClipData"),
                    jni_str!("newPlainText"),
                    jni_sig!("(Ljava/lang/CharSequence;Ljava/lang/CharSequence;)Landroid/content/ClipData;"),
                    &[JValue::Object(&label), JValue::Object(&text)],
                )?
                .l()?;
            env.call_method(
                &manager,
                jni_str!("setPrimaryClip"),
                jni_sig!("(Landroid/content/ClipData;)V"),
                &[JValue::Object(&clip)],
            )?;

            Ok(())
        })
    }
}
