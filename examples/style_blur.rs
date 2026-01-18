#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app).with_size(500., 500.)))
}

fn app() -> impl IntoElement {
    ImageViewer::new(
        "https://github.com/user-attachments/assets/2528e366-a149-469f-a66c-82e5b572ca7c",
    )
    .aspect_ratio(AspectRatio::Max)
    .image_cover(ImageCover::Center)
    .expanded()
    .center()
    .horizontal()
    .spacing(16.)
    .child(
        rect()
            .width(Size::px(100.))
            .height(Size::px(100.))
            .center()
            .background((255, 255, 255, 0.30))
            .color((255, 255, 255))
            .corner_radius(12.0)
            .child("No Blur"),
    )
    .child(
        rect()
            .width(Size::px(100.))
            .height(Size::px(100.))
            .center()
            .background((255, 255, 255, 0.30))
            .color((255, 255, 255))
            .corner_radius(12.0)
            .blur(5.0)
            .child("Blur: 5px"),
    )
    .child(
        rect()
            .width(Size::px(100.))
            .height(Size::px(100.))
            .center()
            .background((255, 255, 255, 0.30))
            .color((255, 255, 255))
            .corner_radius(12.0)
            .blur(10.0)
            .child("Blur: 10px"),
    )
    .child(
        rect()
            .width(Size::px(100.))
            .height(Size::px(100.))
            .center()
            .background((255, 255, 255, 0.30))
            .color((255, 255, 255))
            .corner_radius(12.0)
            .blur(20.0)
            .child("Blur: 20px"),
    )
}
