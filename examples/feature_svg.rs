use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> Element {
    rect()
        .expanded()
        .center()
        .child(
            svg(Bytes::from_static(include_bytes!("./ferris.svg")))
                .width(Size::px(300.))
                .height(Size::px(300.)),
        )
        .into()
}
