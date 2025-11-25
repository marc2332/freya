use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    rect()
        .center()
        .expanded()
        .child(ProgressBar::new(50.).width(Size::px(200.)))
}
