use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    rect()
        .expanded()
        .center()
        .offset_x(-50.)
        .offset_y(25.)
        .scale(0.5)
        .rotate(45.)
        .child(
            rect()
                .font_size(50.)
                .background((222, 231, 145))
                .child("hello!"),
        )
}
