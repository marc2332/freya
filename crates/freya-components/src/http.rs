use bytes::Bytes;
use url::Url;

#[cfg(not(target_os = "emscripten"))]
pub(crate) async fn fetch(url: Url) -> anyhow::Result<Bytes> {
    let client: reqwest::blocking::Client = freya_core::prelude::GlobalContexts::get()
        .get_context_or_insert(|| {
            reqwest::blocking::Client::builder()
                .build()
                .expect("Failed to build the HTTP client.")
        });
    freya_core::prelude::thread(move || Ok(client.get(url).send()?.error_for_status()?.bytes()?))
        .await
}

#[cfg(target_os = "emscripten")]
pub(crate) async fn fetch(url: Url) -> anyhow::Result<Bytes> {
    use std::ffi::{
        CString,
        c_char,
        c_int,
        c_void,
    };

    use anyhow::Context;
    use futures_channel::oneshot::{
        Sender,
        channel,
    };

    unsafe extern "C" {
        fn emscripten_async_wget_data(
            url: *const c_char,
            argument: *mut c_void,
            on_load: extern "C" fn(*mut c_void, *mut c_void, c_int),
            on_error: extern "C" fn(*mut c_void),
        );
    }

    extern "C" fn on_load(argument: *mut c_void, data: *mut c_void, size: c_int) {
        let sender = unsafe { Box::from_raw(argument.cast::<Sender<Option<Bytes>>>()) };
        let data = unsafe { std::slice::from_raw_parts(data.cast::<u8>(), size as usize) };
        sender.send(Some(Bytes::copy_from_slice(data))).ok();
    }

    extern "C" fn on_error(argument: *mut c_void) {
        let sender = unsafe { Box::from_raw(argument.cast::<Sender<Option<Bytes>>>()) };
        sender.send(None).ok();
    }

    let c_url = CString::new(url.as_str())?;
    let (sender, receiver) = channel::<Option<Bytes>>();
    let argument = Box::into_raw(Box::new(sender)).cast();

    unsafe { emscripten_async_wget_data(c_url.as_ptr(), argument, on_load, on_error) };

    receiver
        .await
        .ok()
        .flatten()
        .with_context(|| format!("Failed to fetch {url}."))
}
