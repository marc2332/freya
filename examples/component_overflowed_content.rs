use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app).with_size(500., 450.)))
}

fn app() -> impl IntoElement {
    Button::new().child(
        OverflowedContent::new()
            .width(Size::px(100.))
            .child(label().text("Hello, World! I like Rust!").max_lines(1)),
    )
}
