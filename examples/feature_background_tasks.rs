#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::time::Duration;

use async_io::Timer;
use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_future(move |proxy| async move {
        loop {
            // Create the Window
            Timer::after(Duration::from_secs(1)).await;
            let mut window_id = proxy
                .with(|ctx| {
                    ctx.launch_window(WindowConfig::new(|| {
                        rect().center().expanded().child("Hello, World!")
                    }))
                })
                .await
                .ok();

            // Close the Window
            Timer::after(Duration::from_secs(1)).await;
            if let Some(window_id) = window_id.take() {
                let _ = proxy
                    .with(move |ctx| {
                        ctx.windows_mut().remove(&window_id);
                    })
                    .await;
            }
        }
    }))
}
