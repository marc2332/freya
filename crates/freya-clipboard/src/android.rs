use std::error::Error;

use freya_core::data_transfer::{
    DataFormat,
    DataValue,
    TransferData,
};
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

use crate::clipboard::{
    ClipboardError,
    ClipboardFuture,
    ClipboardProvider,
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

impl AndroidClipboard {
    fn read_text() -> Result<String, Box<dyn Error + Send + Sync>> {
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

    fn write_text(contents: String) -> Result<(), Box<dyn Error + Send + Sync>> {
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

impl ClipboardProvider for AndroidClipboard {
    fn get(&self, format: DataFormat) -> ClipboardFuture<DataValue> {
        if format != DataFormat::Text {
            return Box::pin(async { Err(ClipboardError::UnsupportedFormat) });
        }
        let result = Self::read_text()
            .map(DataValue::Text)
            .map_err(|_| ClipboardError::FailedToRead);
        Box::pin(async move { result })
    }

    fn set(&self, data: TransferData) -> ClipboardFuture<()> {
        let mut formats = data.formats.into_iter();
        let Some((format, value)) = formats.next() else {
            return Box::pin(async { Err(ClipboardError::FailedToSet) });
        };
        if format != DataFormat::Text || formats.next().is_some() {
            return Box::pin(async { Err(ClipboardError::UnsupportedFormat) });
        }
        let contents = match value {
            DataValue::Text(text) => Ok(text),
            DataValue::Bytes(bytes) => {
                String::from_utf8(bytes).map_err(|_| ClipboardError::FailedToSet)
            }
            DataValue::Files(_) => Err(ClipboardError::FailedToSet),
        };
        let result = contents.and_then(|contents| {
            Self::write_text(contents).map_err(|_| ClipboardError::FailedToSet)
        });
        Box::pin(async move { result })
    }
}
