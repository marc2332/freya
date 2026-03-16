#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::{
    prelude::*,
    winit::monitor::MonitorHandle,
};

fn main() {
    launch(LaunchConfig::new().with_future(|proxy| async move {
        let monitors: Vec<MonitorHandle> = proxy
            .with(|ctx| ctx.active_event_loop.available_monitors().collect())
            .await
            .unwrap();

        for monitor in monitors {
            let position = monitor.position();

            let _ = proxy
                .with(move |ctx| {
                    ctx.launch_window(
                        WindowConfig::new(move || app(monitor.clone()))
                            .with_size(400., 300.)
                            .with_window_attributes(move |attrs, _| attrs.with_position(position)),
                    )
                })
                .await;
        }
    }));
}

fn app(monitor: MonitorHandle) -> impl IntoElement {
    let name = monitor.name().unwrap_or_else(|| "Unknown".into());
    let size = monitor.size();
    let position = monitor.position();

    rect()
        .expanded()
        .center()
        .spacing(8.)
        .child(format!("Monitor: {name}"))
        .child(format!("Resolution: {}x{}", size.width, size.height))
        .child(format!("Position: ({}, {})", position.x, position.y))
}
