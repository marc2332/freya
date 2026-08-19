/// Runs a blocking closure off the caller's thread.
#[cfg(not(target_os = "emscripten"))]
pub async fn unblock<T: Send + 'static>(function: impl FnOnce() -> T + Send + 'static) -> T {
    blocking::unblock(function).await
}

/// Runs a blocking closure inline.
#[cfg(target_os = "emscripten")]
pub async fn unblock<T: Send + 'static>(function: impl FnOnce() -> T + Send + 'static) -> T {
    function()
}
