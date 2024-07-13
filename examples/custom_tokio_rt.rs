#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[cfg(not(feature = "custom-tokio-rt"))]
fn main() {
    panic!("Run this example without the `custom-tokio-rt` feature.");
}

#[cfg(feature = "custom-tokio-rt")]
fn main() {
    use freya::prelude::*;

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let _guard = rt.enter();
    launch_with_title(|| rsx!(label { "Hello, World!" }), "Custom Tokio Runtime")
}
