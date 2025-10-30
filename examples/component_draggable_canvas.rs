use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> Element {
    rect()
        .expanded()
        .child(
            DraggableCanvas::new()
                .width(Size::fill())
                .height(Size::fill())
                .child(Draggable::new().child(Button::new().child("Hello, World!"))),
        )
        .into()
}
