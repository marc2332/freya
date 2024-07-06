#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

#[cfg(not(feature = "custom-tokio-rt"))]
fn main() {
    panic!("Run this example without the `custom-tokio-rt` feature.");
}

#[cfg(feature = "custom-tokio-rt")]
#[tokio::main]
async fn main() {
    launch_with_title(|| rsx!(label { "Hello, World!" }), "Custom Tokio Runtime")
}
