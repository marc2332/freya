#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path::PathBuf;

use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let uri = "https://images.pexels.com/photos/842711/pexels-photo-842711.jpeg";
    let path = PathBuf::from("./examples/rust_logo.png");
    let embedded = ("rust-logo", include_bytes!("./rust_logo.png"));
    let broken = PathBuf::from("./examples/this-file-does-not-exist.png");

    rect()
        .expanded()
        .horizontal()
        .center()
        .child(
            ImageViewer::new(uri)
                .width(Size::percent(25.))
                .a11y_alt("Beautiful landscape."),
        )
        .child(ImageViewer::new(path).width(Size::percent(25.)))
        .child(ImageViewer::new(embedded).width(Size::percent(25.)))
        .child(
            ImageViewer::new(broken)
                .width(Size::percent(25.))
                .height(Size::px(200.))
                .error_renderer(|err: String| {
                    rect()
                        .padding(12.)
                        .rounded_sm()
                        .center()
                        .background((40, 0, 0))
                        .child(
                            label()
                                .color((255, 120, 120))
                                .text(format!("Failed to load image:\n{err}")),
                        )
                        .into()
                }),
        )
}
