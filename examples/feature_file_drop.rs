#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::path::PathBuf;

use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app).with_size(500., 450.)))
}

#[derive(PartialEq)]
enum Status {
    Idle,
    Hovering,
    Dropped(PathBuf),
}

fn app() -> impl IntoElement {
    let mut status = use_state(|| Status::Idle);

    let (msg, background) = match &*status.read() {
        Status::Idle => ("Waiting for drop".to_string(), (109, 198, 227)),
        Status::Hovering => ("Drop it!".to_string(), (109, 198, 227)),
        Status::Dropped(path) => (path.to_str().unwrap().to_string(), (109, 198, 227)),
    };

    rect()
        .expanded()
        .center()
        .background(background)
        .color(Color::WHITE)
        .on_file_drop(move |e: Event<FileEventData>| {
            if let Some(file_path) = e.file_path.clone() {
                status.set(Status::Dropped(file_path));
            } else {
                status.set(Status::Idle);
            }
        })
        .on_global_file_hover(move |_| {
            status.set(Status::Hovering);
        })
        .on_global_file_hover_cancelled(move |_| {
            status.set(Status::Idle);
        })
        .child(msg)
}
