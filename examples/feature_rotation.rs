use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    rect().expanded().center().rotate(300.).child(
        rect()
            .font_size(50.)
            .background((222, 231, 145))
            .child("hello!"),
    )
}
