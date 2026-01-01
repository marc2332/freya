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
            .child(Draggable::new().child(Button::new().child("Hello, World!"))),
    )
}
