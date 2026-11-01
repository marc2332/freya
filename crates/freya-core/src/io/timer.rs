use std::time::Duration;

/// Waits for the given duration.
#[cfg(not(target_os = "emscripten"))]
pub async fn timer(duration: Duration) {
    async_io::Timer::after(duration).await;
}

/// Waits for the given duration, on a browser timer.
#[cfg(target_os = "emscripten")]
pub async fn timer(duration: Duration) {
    use std::ffi::c_void;

    use futures_channel::oneshot::{
        Sender,
        channel,
    };

    unsafe extern "C" {
        fn emscripten_async_call(
            callback: extern "C" fn(*mut c_void),
            argument: *mut c_void,
            millis: i32,
        );
    }

    extern "C" fn wake(argument: *mut c_void) {
        let sender = unsafe { Box::from_raw(argument.cast::<Sender<()>>()) };
        sender.send(()).ok();
    }

    let (sender, receiver) = channel::<()>();
    let argument = Box::into_raw(Box::new(sender)).cast();
    let millis = duration.as_millis().min(i32::MAX as u128) as i32;

    unsafe { emscripten_async_call(wake, argument, millis) };

    receiver.await.ok();
}
