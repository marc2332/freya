use freya::prelude::*;

fn main() {
    launch(
        LaunchConfig::new().with_window(
            WindowConfig::new(app)
                .with_background(Color::TRANSPARENT)
                .with_decorations(false)
                .with_transparency(true)
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
