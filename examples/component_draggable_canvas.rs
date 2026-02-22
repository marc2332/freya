#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    rect().expanded().child(
        DraggableCanvas::new()
            .width(Size::fill())
            .height(Size::fill())
            .child(
                Draggable::new().child(
                    rect()
                        .background(Color::from_rgb(200, 100, 20))
                        .child("Draggable"),
                ),
            )
            .child(
                ResizableDraggable::new((150., 150.))
                    .initial_position((50., 50.))
                    .child(
                        rect()
                            .expanded()
                            .background(Color::from_rgb(30, 100, 200))
                            .child("Resizable Draggable"),
                    ),
            ),
    )
}
