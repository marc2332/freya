#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::{
    camera::*,
    prelude::*,
};

fn main() {
    tracing_subscriber::fmt::init();

    if !freya::camera::init() {
        eprintln!("camera access denied");
        return;
    }

    launch(
        LaunchConfig::new().with_window(
            WindowConfig::new(app)
                .with_size(800., 600.)
                .with_title("Camera"),
        ),
    )
}

fn app() -> impl IntoElement {
    let camera = use_camera(CameraConfig::default);

    rect().expanded().background((20, 20, 20)).center().child(
        CameraViewer::new(camera)
            .corner_radius(12.)
            .loading_placeholder(label().text("Opening camera...").color(Color::WHITE))
            .error_renderer(|err: CameraError| {
                label()
                    .color((255, 120, 120))
                    .text(format!("Camera error: {err}"))
                    .into()
            }),
    )
}
