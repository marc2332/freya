use std::future::Future;

/// Runs a blocking closure off the caller's thread.
#[cfg(not(target_os = "emscripten"))]
pub fn unblock<T: Send + 'static>(
    function: impl FnOnce() -> T + Send + 'static,
) -> impl Future<Output = T> {
    blocking::unblock(function)
}

/// Runs a blocking closure inline.
#[cfg(target_os = "emscripten")]
pub fn unblock<T: Send + 'static>(
    function: impl FnOnce() -> T + Send + 'static,
) -> impl Future<Output = T> {
    std::future::ready(function())
}
