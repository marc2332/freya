use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    rect()
        .expanded()
        .center()
        .spacing(8.)
        .child(Button::new().child("Nice button"))
        .child(rect().debug().child("Some text"))
}
