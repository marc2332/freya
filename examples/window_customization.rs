#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::{prelude::*, winit::dpi::LogicalPosition};

fn main() {
    let (width, height) = (600, 600);
    launch(
        LaunchConfig::new().with_window(
            WindowConfig::new(app)
                .with_size(width as f64, height as f64)
                .with_background(Color::TRANSPARENT)
                .with_decorations(false)
                .with_transparency(true)
                .with_window_attributes(move |attributes, el| {
                    // Centers the window
                    if let Some(monitor) = el
                        .primary_monitor()
                        .or_else(|| el.available_monitors().next())
                    {
                        let size = monitor.size();
                        attributes.with_position(LogicalPosition {
                            x: size.width as i32 / 2 - width / 2,
                            y: size.height as i32 / 2 - height / 2,
                        })
                    } else {
                        attributes
                    }
                })
                .with_window_handle(|window| {
                    let _ = window.set_cursor_hittest(false);
                }),
        ),
    )
}

fn app() -> impl IntoElement {
    rect()
        .center()
        .expanded()
        .color((0, 255, 0))
        .font_size(100)
        .child("Frameless window")
}
