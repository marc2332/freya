#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::time::Duration;

use async_io::Timer;
use freya::prelude::*;

/// Reproduces https://github.com/marc2332/freya/issues/2239
///
/// Copy some text in another app before running this on native Wayland.
/// Closing both windows in one go makes the compositor send the clipboard
/// offer to a data device that has just been released, and the next window
/// then aborts the event loop with "not a valid new object id".
fn main() {
    launch(
        LaunchConfig::new()
            .with_exit_on_close(false)
            .with_future(|proxy| async move {
                let open = |title: &'static str| {
                    proxy.post_callback(move |ctx| {
                        ctx.launch_window(
                            WindowConfig::new(app)
                                .with_title(title)
                                .with_size(400., 300.),
                        )
                    })
                };

                Timer::after(Duration::from_secs(1)).await;
                let first = open("First").await.ok();
                Timer::after(Duration::from_secs(1)).await;
                let second = open("Second").await.ok();
                Timer::after(Duration::from_secs(1)).await;

                let _ = proxy
                    .post_callback(move |ctx| {
                        for window_id in [second, first].into_iter().flatten() {
                            ctx.windows_mut().remove(&window_id);
                        }
                    })
                    .await;

                Timer::after(Duration::from_secs(1)).await;
                let _ = open("Third").await;
            }),
    )
}

fn app() -> impl IntoElement {
    rect().expanded().center().child("Still alive")
}
