use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    rect()
        .width(Size::px(300.))
        .height(Size::px(300.))
        .overflow_mode(OverflowMode::Clip)
        .child(
            rect()
                .width(Size::px(600.))
                .height(Size::px(600.))
                .background((0, 119, 182)),
        )
}
